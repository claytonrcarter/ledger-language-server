use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tower_lsp::lsp_types::Range as LspRange;
use tower_lsp::lsp_types::*;
use tree_sitter::{Language, Node, Parser, Point, Tree};
use type_sitter::StreamingIterator;
use walkdir::WalkDir;

use crate::{backend_format, contents_of_path};

fn substring(source: &[u8], start_byte: usize, end_byte: usize) -> Result<String> {
    Ok(
        std::str::from_utf8(&source[start_byte..end_byte.min(source.len())])?
            .trim()
            .to_string(),
    )
}

fn word_boundary_range(line: &str, index: usize, addl_end_char: Option<char>) -> (usize, usize) {
    let start_boundary = vec![' ', '\t'];
    let end_boundary = addl_end_char.map_or_else(
        || start_boundary.clone(),
        |c| {
            let mut chars = start_boundary.clone();
            chars.push(c);
            chars
        },
    );
    if let Some((before_point, after_point)) = line.split_at_checked(index) {
        let start_offset = before_point
            .rfind(start_boundary.as_slice())
            .map_or_else(|| before_point.len(), |i| i + 1);
        let end_offset = after_point
            .find(end_boundary.as_slice())
            .unwrap_or(after_point.len());

        // dbg!(line, index, start_offset, end_offset, index + end_offset);

        (start_offset, index + end_offset)
    } else {
        (index, index)
    }
}

fn lsp_range_from_ts_range(range: tree_sitter::Range) -> LspRange {
    LspRange {
        start: Position {
            line: range.start_point.row as u32,
            character: range.start_point.column as u32,
        },
        end: Position {
            line: range.end_point.row as u32,
            character: range.end_point.column as u32,
        },
    }
}

#[derive(Debug)]
pub enum CompletionResult {
    Some {
        range: LspRange,
        completions: Vec<LedgerCompletion>,
    },
    None,
    NoNode(String),
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LedgerCompletion {
    Account(String),
    Directive(String),
    File(String),
    Payee(String),
    Period(String),
    PeriodSnippet(Snippet),
    Tag(String),
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Snippet {
    pub label: String,
    pub snippet: String,
}

#[derive(Debug)]
pub enum TransactionStatus {
    // Position is where the status would go: at the end of the date node.
    NotCleared(Position),

    // Range is where the current status is, including trailing whitespace, before code or payee.
    Pending(LspRange),
    Cleared(LspRange),
}

pub struct LedgerBackend {
    _test_included_content: Option<String>,
    _test_project_files: Option<Vec<String>>,

    /// Map of documents (ie source code text) to a parsed tree-sitter Tree
    trees_cache: HashMap<String, Tree>,
}

impl LedgerBackend {
    pub fn new() -> Self {
        Self {
            _test_included_content: None,
            _test_project_files: None,
            trees_cache: HashMap::new(),
        }
    }

    fn parser(&self) -> Result<Parser> {
        let mut parser = Parser::new();
        let language = Language::new(tree_sitter_ledger::LANGUAGE);
        parser.set_language(&language)?;
        Ok(parser)
    }

    /// Parse an input document (source code) and save the parsed Tree for use
    /// later. If the document has already been cached, no new parsing is done.
    pub fn parse_document(&mut self, content: &str) {
        if !self.trees_cache.contains_key(content) {
            if let Ok(mut parser) = self.parser() {
                if let Some(tree) = parser.parse(content, None) {
                    self.trees_cache.insert(content.to_string(), tree);
                }
            }
        }
    }

    pub fn transaction_at_position_status(
        &mut self,
        content: &str,
        position: &Position,
    ) -> Result<Option<TransactionStatus>> {
        let mut node = match self.node_at_position(content, position) {
            Some(node) => node,
            None => {
                return Ok(None);
            }
        };

        while node.kind() != "plain_xact" {
            if let Some(parent) = node.parent() {
                node = parent
            } else {
                // dbg!(position, node.kind(), node.range());
                return Ok(None);
            }
        }
        // dbg!(position, node.kind(), node.range());

        let mut date_node = None;
        let mut status_node = None;
        let mut code_or_payee_node = None;

        let mut cursor = node.walk();
        for node in node.named_children(&mut cursor) {
            if node.kind() == "date" {
                date_node = Some(node);
            } else if node.kind() == "status" {
                status_node = Some(node);
            } else if node.kind() == "code" || node.kind() == "payee" {
                code_or_payee_node = Some(node);
                break;
            }
        }

        if let Some(node) = status_node {
            let status = substring(
                content.as_bytes(),
                node.range().start_byte,
                node.range().end_byte,
            )?;

            // node range is only the status character, our replacement range
            // should include the leading whitespace
            let mut range = lsp_range_from_ts_range(node.range());
            range.start.character = range.start.character.saturating_sub(1);

            match status.trim() {
                "!" => Ok(Some(TransactionStatus::Pending(range))),
                "*" => Ok(Some(TransactionStatus::Cleared(range))),
                _ => Err(anyhow!("TODO")),
            }
        } else if let Some(node) = date_node {
            // add status at end of date node
            Ok(Some(TransactionStatus::NotCleared(
                lsp_range_from_ts_range(node.range()).end,
            )))
        } else if let Some(node) = code_or_payee_node {
            // add status before code or payee, preserving an existing whitespace
            let mut range = lsp_range_from_ts_range(node.range());
            range.start.character = range.start.character.saturating_sub(1);
            range.end = range.start;
            Ok(Some(TransactionStatus::NotCleared(range.start)))
        } else {
            Err(anyhow!("TODO"))
        }
    }

    pub fn pending_transaction_status_ranges(&mut self, content: &str) -> Result<Vec<LspRange>> {
        let tree = match self.trees_cache.get(content) {
            Some(tree) => tree.clone(),
            None => {
                return Err(anyhow!("no tree found for given contents"));
            }
        };

        let ts_query = tree_sitter::Query::new(
            match self.parser()?.language() {
                Some(ref language) => language,
                None => bail!("getting tree-sitter language"),
            },
            "(status) @status",
        )?;
        let mut cursor = tree_sitter::QueryCursor::new();

        let source = content.as_bytes();
        let mut matches = cursor.matches(&ts_query, tree.root_node(), source);
        let mut ranges = Vec::new();
        while let Some(m) = matches.next() {
            for status_node in m.nodes_for_capture_index(0) {
                let capture_text =
                    substring(source, status_node.start_byte(), status_node.end_byte())?;
                if capture_text == "!" {
                    ranges.push(lsp_range_from_ts_range(status_node.range()));
                }
            }
        }

        Ok(ranges)
    }

    pub fn completions_for_position(
        &mut self,
        buffer_path: &str,
        content: &str,
        position: &Position,
        visited: &mut HashSet<String>,
    ) -> Result<CompletionResult> {
        let mut completions = HashSet::new();

        let node = match self.node_at_position(content, position) {
            Some(node) => node,
            None => {
                return Ok(CompletionResult::NoNode(format!(
                    "No node found at position {position:?}"
                )));
            }
        };
        let current_node_content = substring(
            content.as_bytes(),
            node.range().start_byte,
            node.range().end_byte,
        )?;
        let mut range = node.range();

        let line_content = content.lines().nth(position.line as usize).unwrap_or("");

        // dbg!(position, node.kind(), node.range());
        match node.kind() {
            "account" => self.populate_completions(
                &mut completions,
                buffer_path,
                "(account) @account",
                content,
                &|account| {
                    if account != current_node_content {
                        Some(LedgerCompletion::Account(account))
                    } else {
                        // don't include current node content
                        None
                    }
                },
                visited,
            )?,

            "filename" => self.completions_insert_project_files(&mut completions, buffer_path)?,
            // we may be at the end of the include directive line
            "word_directive"
                if node.range().end_point.column == position.character as usize
                    && node
                        .named_child(0)
                        .is_some_and(|child| child.kind() == "filename") =>
            {
                if let Some(child) = node.named_child(0) {
                    range = child.range();
                    self.completions_insert_project_files(&mut completions, buffer_path)?
                }
            }

            "interval" => {
                let (start, end) =
                    word_boundary_range(line_content, position.character as usize, None);
                range.start_point.column = start;
                range.end_point.row = range.start_point.row;
                range.end_point.column = end;

                self.completions_insert_periods(&mut completions)
            }
            // (ERROR) w/ leading ~ => no interval or postings yet
            "ERROR" if line_content.starts_with("~") => {
                let (start, end) =
                    word_boundary_range(line_content, position.character as usize, None);
                range.start_point.column = start;
                range.end_point.row = range.start_point.row;
                range.end_point.column = end;

                self.completions_insert_periods(&mut completions)
            }

            "payee" => self.populate_completions(
                &mut completions,
                buffer_path,
                "(payee) @payee",
                content,
                &|payee| {
                    if payee != current_node_content {
                        Some(LedgerCompletion::Payee(payee))
                    } else {
                        // don't include current node content
                        None
                    }
                },
                visited,
            )?,

            // complete tags only for notes that are indented (ie for postings)
            "note" if range.start_point.column != 0 => {
                let (start, end) =
                    word_boundary_range(line_content, position.character as usize, Some(':'));
                range.start_point.column = start;
                range.end_point.row = range.start_point.row;
                range.end_point.column = end;

                self.populate_completions(
                    &mut completions,
                    buffer_path,
                    "(note) @note",
                    content,
                    &|note| {
                        if note == current_node_content {
                            // don't include current node content
                            return None;
                        }

                        match note
                            // https://ledger-cli.org/doc/ledger3.html#Commenting-on-your-Journal
                            .trim_start_matches([' ', '\t', ';', '#', '%', '|', '*'])
                            .split_once(": ")
                        {
                            Some((tag, _)) if !tag.contains(' ') => {
                                Some(LedgerCompletion::Tag(tag.to_owned()))
                            }
                            Some(_) | None => None,
                        }
                    },
                    visited,
                )?
            }

            // TODO subdirectives
            // if the error starts at the start of the line, maybe we're in the
            // middle of typing a directive
            "word_directive" => self.completions_insert_directives(&mut completions),

            "ERROR" if node.range().start_point.column == 0 => {
                self.completions_insert_directives(&mut completions)
            }

            // if we're at the start of an "empty" line
            "source_file" if position.character == 0 => {
                self.completions_insert_directives(&mut completions)
            }

            _ => return Ok(CompletionResult::None),
        };

        // remove trailing newline from range to replace
        if range.end_point.column == 0 && range.start_point.row != range.end_point.row {
            range.end_byte -= 1;
            range.end_point.row -= 1;
            range.end_point.column = content.lines().nth(range.end_point.row).unwrap_or("").len();
        }

        Ok(CompletionResult::Some {
            range: LspRange {
                start: Position {
                    line: range.start_point.row as u32,
                    character: range.start_point.column as u32,
                },
                end: Position {
                    line: range.end_point.row as u32,
                    character: range.end_point.column as u32,
                },
            },
            completions: completions.into_iter().collect(),
        })
    }

    pub fn populate_completions<F>(
        &mut self,
        completions: &mut HashSet<LedgerCompletion>,
        buffer_path: &str,
        query: &str,
        content: &str,
        completion_fn: &F,
        visited: &mut HashSet<String>,
    ) -> Result<()>
    where
        F: Fn(String) -> Option<LedgerCompletion>,
    {
        let current_dir = match Path::new(buffer_path).parent() {
            Some(dir) => dir,
            None => {
                // TODO ??
                return Err(anyhow!(
                    "[completions] Buffer has no parent dir? {buffer_path}"
                ));
            }
        };

        let tree = match self.trees_cache.get(content) {
            Some(tree) => tree.clone(),
            None => {
                // self.parse_document(content);
                // self.trees_cache.get(content).unwrap().clone()
                return Err(anyhow!(
                    "no tree found for contents of file '{buffer_path}'"
                ));
            }
        };

        let ts_query = tree_sitter::Query::new(
            match self.parser()?.language() {
                Some(ref language) => language,
                None => bail!("getting tree-sitter language"),
            },
            format!("{query} (filename) @filename").as_str(),
        )?;
        let mut cursor = tree_sitter::QueryCursor::new();

        let source = content.as_bytes();
        let mut matches = cursor.matches(&ts_query, tree.root_node(), source);
        while let Some(m) = matches.next() {
            // query as passed in
            for n in m.nodes_for_capture_index(0) {
                let capture_text = substring(source, n.start_byte(), n.end_byte())?;
                if let Some(completion) = completion_fn(capture_text) {
                    completions.insert(completion);
                }
            }

            // (filename) @filename
            for n in m.nodes_for_capture_index(1) {
                let filename = substring(source, n.start_byte(), n.end_byte())?;

                let path = Path::new(&filename);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    current_dir.join(path)
                };
                let filename = path.as_os_str().to_str().unwrap_or(&filename);

                if visited.contains(filename) {
                    continue;
                } else {
                    visited.insert(filename.to_string());
                }

                let included_content = self
                    ._test_included_content
                    .as_ref()
                    .map_or_else(
                        || contents_of_path(filename),
                        |content| Ok(content.to_string()),
                    )
                    .unwrap_or_else(|_| String::new());

                self.parse_document(&included_content);

                self.populate_completions(
                    completions,
                    filename,
                    query,
                    &included_content,
                    completion_fn,
                    visited,
                )?;
            }
        }

        Ok(())
    }

    fn completions_insert_project_files(
        &self,
        completions: &mut HashSet<LedgerCompletion>,
        buffer_path: &str,
    ) -> Result<()> {
        let current_dir = match Path::new(buffer_path).parent() {
            Some(dir) => dir,
            None => {
                // TODO ??
                return Err(anyhow!(
                    "[completions] Buffer has no parent dir? {buffer_path}"
                ));
            }
        };

        let project_files = self._test_project_files.clone().unwrap_or_else(|| {
            WalkDir::new(current_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_name()
                        .to_str()
                        .is_some_and(|f| f.ends_with(".ledger"))
                })
                .map(|f| {
                    f.path()
                        .strip_prefix(current_dir)
                        .unwrap_or_else(|_err| f.path())
                        .as_os_str()
                        .to_string_lossy()
                        .to_string()
                })
                .collect()
        });

        project_files.into_iter().for_each(|f| {
            if f.ends_with(".ledger") {
                completions.insert(LedgerCompletion::File(f.clone()));
            }
        });

        Ok(())
    }

    fn completions_insert_directives(&self, completions: &mut HashSet<LedgerCompletion>) {
        // only worrying about the most common for now
        // https://ledger-cli.org/doc/ledger3.html#Command-Directives
        vec![
            "account",
            "alias",
            "commodity",
            "include",
            "payee",
            "tag",
            "year",
        ]
        .into_iter()
        .for_each(|s| {
            completions.insert(LedgerCompletion::Directive(s.to_string()));
        });
    }

    fn completions_insert_periods(&self, completions: &mut HashSet<LedgerCompletion>) {
        // only worrying about the most common for now
        // https://ledger-cli.org/doc/ledger3.html#Period-Expressions
        vec![
            "Every Day",
            "Every Week",
            "Every Month",
            "Every Quarter",
            "Every Year",
            "Daily",
            "Weekly",
            "Biweekly",
            "Monthly",
            "Bimonthly",
            "Quarterly",
            "Yearly",
        ]
        .into_iter()
        .for_each(|s| {
            completions.insert(LedgerCompletion::Period(s.to_string()));
        });

        vec![
            "Every $1 Days",
            "Every $1 Weeks",
            "Every $1 Months",
            "Every $1 Quarters",
            "Every $1 Years",
        ]
        .into_iter()
        .for_each(|s| {
            completions.insert(LedgerCompletion::PeriodSnippet(Snippet {
                label: s.replace("$1", "N"),
                snippet: s.to_string(),
            }));
        });

        vec!["from $1", "since $1", "to $1", "until $1", "in $1"]
            .into_iter()
            .for_each(|s| {
                completions.insert(LedgerCompletion::PeriodSnippet(Snippet {
                    label: s.replace("$1", "DATE"),
                    snippet: s.to_string(),
                }));
            });

        completions.insert(LedgerCompletion::PeriodSnippet(Snippet {
            label: "from DATE to DATE".to_string(),
            snippet: "from $1 to $2".to_string(),
        }));
    }

    pub fn diagnostics(buffer_path: &str, content: &str) -> Vec<Diagnostic> {
        content
            .split('\n')
            .enumerate()
            .filter_map(|(i, line)| {
                let path = match line.trim().split_once(' ') {
                    Some(("include", maybe_path)) => {
                        let quotes: &[_] = &['"', '\''];
                        maybe_path.trim().trim_matches(quotes)
                    }
                    None | Some((_, _)) => return None,
                };

                let path_start_offset = line.find(path).unwrap_or(0) as u32;
                let path_len = path.len() as u32;

                Some((
                    path,
                    LspRange {
                        start: Position {
                            line: i as u32,
                            character: path_start_offset,
                        },
                        end: Position {
                            line: i as u32,
                            character: path_start_offset + path_len,
                        },
                    },
                ))
            })
            .filter_map(|(path, range)| {
                let fs_path = {
                    let path = Path::new(path);
                    if path.is_absolute() {
                        path.to_path_buf()
                    } else {
                        // FIXME what is parent() is None?
                        let dir = Path::new(buffer_path).parent()?;
                        dir.join(path)
                    }
                };

                if fs_path.exists() {
                    None
                } else {
                    Some(Diagnostic::new_simple(
                        range,
                        format!("File '{path}' does not exist"),
                    ))
                }
            })
            .collect()
    }

    pub fn format(content: &str, sort_transactions: bool) -> Result<String, String> {
        backend_format::format(content, sort_transactions)
            .map_err(|_err| "TODO convert io::Error to ???".to_string())
    }

    /// Get the smallest named node at the given position.
    fn node_at_position(&mut self, content: &str, position: &Position) -> Option<Node> {
        let debug = false;
        let tree = self.trees_cache.get(content)?;

        // FIXME this seems like it may be expensive; this fn is called for
        // every call for completions; collecting a large buffer to lines is
        // likely to affect perf
        let content_lines: Vec<&str> = content.lines().collect();
        let content_line = content_lines.get(position.line as usize).unwrap_or(&"");

        let point = {
            let mut point = Point {
                row: position.line as usize,
                column: position.character as usize,
            };

            let position_is_end_of_file =
                content_lines.len() == point.row + 1 && content_line.len() == point.column;

            if position_is_end_of_file {
                if point.column != 0 {
                    point.column -= 1;
                } else {
                    point.row -= 1;
                    point.column = content_lines.get(point.row).map_or(0, |l| l.len());
                }
            }
            point
        };
        let mut cursor = tree.walk();

        // descend to smallest node @ point
        while cursor.goto_first_child_for_point(point).is_some() {}

        if debug {
            eprintln!(
                "bottomed out at {}{} node '{}' {:?}-{:?}",
                if cursor.node().is_named() {
                    "named"
                } else {
                    "anon"
                },
                if cursor.node().is_error() {
                    " error"
                } else {
                    ""
                },
                cursor.node().kind(),
                cursor.node().range().start_point,
                cursor.node().range().end_point
            );
        }

        // seek to first named node; if the current (unnamed) node starts at
        // point, then the point/cursor could be at the "end" of the previous
        // node
        while !cursor.node().is_named() {
            if cursor.node().range().start_point == point {
                if !cursor.goto_previous_sibling() {
                    cursor.goto_parent();
                }
            } else {
                cursor.goto_parent();
            }

            // if current node is an error at end of line, maybe we're building
            // a line that happens to be invalid temporarily; try checking the
            // previous node
            if cursor.node().is_error()
                && cursor.node().range().end_point.column == content_line.len()
            {
                cursor.goto_previous_sibling();
            }
        }

        if debug {
            eprintln!(
                "ended up at {}{} node '{}' {:?}-{:?}",
                if cursor.node().is_named() {
                    "named"
                } else {
                    "anon"
                },
                if cursor.node().is_error() {
                    " error"
                } else {
                    ""
                },
                cursor.node().kind(),
                cursor.node().range().start_point,
                cursor.node().range().end_point
            );
        }

        Some(cursor.node())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_completions_payees() {
        let source = textwrap::dedent(
            "
            24/01/02 Payee1
                Account

            24/02/03 Payee2
                Account

            24/02/03 Mom & Dad
                Account
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 1,
                character: 10,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 1,
                    character: 9,
                },
                end: Position {
                    line: 1,
                    character: 15,
                },
            },
            [
                Payee(
                    "Mom & Dad",
                ),
                Payee(
                    "Payee2",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_accounts() {
        let source = textwrap::dedent(
            "
            24/01/02 Payee1
                Account1  $1
                Account2

            24/02/03 Payee2
                Account2  $2
                Account3

            24/02/03 Mom & Dad
                One & Two  $2
                Three & Four
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 2,
                character: 10,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 12,
                },
            },
            [
                Account(
                    "Account2",
                ),
                Account(
                    "Account3",
                ),
                Account(
                    "One & Two",
                ),
                Account(
                    "Three & Four",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_periods() {
        let source = vec!["~ ", ""].join("\n");

        let completions = get_completions(
            &source,
            &Position {
                line: 0,
                character: 2,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions.1,
        @r#"
        [
            Period(
                "Bimonthly",
            ),
            Period(
                "Biweekly",
            ),
            Period(
                "Daily",
            ),
            Period(
                "Every Day",
            ),
            Period(
                "Every Month",
            ),
            Period(
                "Every Quarter",
            ),
            Period(
                "Every Week",
            ),
            Period(
                "Every Year",
            ),
            Period(
                "Monthly",
            ),
            Period(
                "Quarterly",
            ),
            Period(
                "Weekly",
            ),
            Period(
                "Yearly",
            ),
            PeriodSnippet(
                Snippet {
                    label: "Every N Days",
                    snippet: "Every $1 Days",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "Every N Months",
                    snippet: "Every $1 Months",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "Every N Quarters",
                    snippet: "Every $1 Quarters",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "Every N Weeks",
                    snippet: "Every $1 Weeks",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "Every N Years",
                    snippet: "Every $1 Years",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "from DATE",
                    snippet: "from $1",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "from DATE to DATE",
                    snippet: "from $1 to $2",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "in DATE",
                    snippet: "in $1",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "since DATE",
                    snippet: "since $1",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "to DATE",
                    snippet: "to $1",
                },
            ),
            PeriodSnippet(
                Snippet {
                    label: "until DATE",
                    snippet: "until $1",
                },
            ),
        ]
        "#
        );
    }

    #[test]
    fn test_completions_periods_empty_xact() {
        let source = vec!["~ ", ""].join("\n");

        let completions = get_completions(
            &source,
            &Position {
                line: 0,
                character: 2,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions.0,
        @r#"
        Range {
            start: Position {
                line: 0,
                character: 2,
            },
            end: Position {
                line: 0,
                character: 2,
            },
        }
        "#
        );
        assert!(completions.1.len() > 0);
        if let LedgerCompletion::Period(_) = completions.1[0] {
            assert!(true);
        } else {
            panic!("completions do not include periods");
        }
    }

    #[test]
    fn test_completions_periods_adding_to_valid_interval() {
        let source = vec!["~ weekly ", ""].join("\n");

        let completions = get_completions(
            &source,
            &Position {
                line: 0,
                character: 9,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions.0,
        @r#"
        Range {
            start: Position {
                line: 0,
                character: 9,
            },
            end: Position {
                line: 0,
                character: 9,
            },
        }
        "#
        );
        assert!(completions.1.len() > 0);
        if let LedgerCompletion::Period(_) = completions.1[0] {
            assert!(true);
        } else {
            panic!("completions do not include periods");
        }
    }

    #[test]
    fn test_completions_periods_changing_interval_word() {
        let source = vec!["~ weekly from ", ""].join("\n");

        let completions = get_completions(
            &source,
            &Position {
                line: 0,
                character: 11,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions.0,
        @r#"
        Range {
            start: Position {
                line: 0,
                character: 9,
            },
            end: Position {
                line: 0,
                character: 13,
            },
        }
        "#
        );
        assert!(completions.1.len() > 0);
        if let LedgerCompletion::Period(_) = completions.1[0] {
            assert!(true);
        } else {
            panic!("completions do not include periods");
        }
    }

    #[test]
    fn test_completions_periods_partial_xact() {
        let source = textwrap::dedent(
            "
            ~ Ev
                Account
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 1,
                character: 3,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions.0,
        @r#"
        Range {
            start: Position {
                line: 1,
                character: 2,
            },
            end: Position {
                line: 1,
                character: 4,
            },
        }
        "#
        );
    }

    #[test]
    fn test_completions_directives() {
        // TODO empty line
        // TODO subdirective (starts w/ whitespace)
        let source = "
        i
        ";

        let completions = get_completions(
            &source,
            &Position {
                line: 1,
                character: 1,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 1,
                    character: 0,
                },
                end: Position {
                    line: 2,
                    character: 8,
                },
            },
            [
                Directive(
                    "account",
                ),
                Directive(
                    "alias",
                ),
                Directive(
                    "commodity",
                ),
                Directive(
                    "include",
                ),
                Directive(
                    "payee",
                ),
                Directive(
                    "tag",
                ),
                Directive(
                    "year",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_tags() {
        let source = textwrap::dedent(
            "
            24/01/02 Payee
                ; Tag1: foo
                ; Tag2: bar
                ;
                ; T
                Account
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 4,
                character: 5,
            },
            None,
        );
        // FIXME this should be starting at char 5, right?
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 4,
                    character: 4,
                },
                end: Position {
                    line: 4,
                    character: 5,
                },
            },
            [
                Tag(
                    "Tag1",
                ),
                Tag(
                    "Tag2",
                ),
            ],
        )
        "#
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 3,
                character: 7,
            },
            None,
        );
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 3,
                    character: 6,
                },
                end: Position {
                    line: 3,
                    character: 10,
                },
            },
            [
                Tag(
                    "Tag1",
                ),
            ],
        )
        "#
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 5,
                character: 7,
            },
            None,
        );
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 5,
                    character: 6,
                },
                end: Position {
                    line: 5,
                    character: 7,
                },
            },
            [
                Tag(
                    "Tag1",
                ),
                Tag(
                    "Tag2",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_files() {
        let source = "include ''";

        let mut be = LedgerBackend::new();
        be._test_project_files = Some(
            vec!["foo.ledger", "bar.yaml", "baz/qux.ledger"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        );
        be.parse_document(source);

        let completions = get_completions(
            &source,
            &Position {
                line: 0,
                character: 9,
            },
            Some(be),
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 0,
                    character: 8,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            },
            [
                File(
                    "baz/qux.ledger",
                ),
                File(
                    "foo.ledger",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_from_included_files() {
        let included = textwrap::dedent(
            "
            24/01/02 IncludedPayee
                IncludedAccount
            ",
        );
        let source = textwrap::dedent(
            "
            include foo.ledger

            24/01/02 Payee
                Account
            ",
        );

        let mut be = LedgerBackend::new();
        be._test_included_content = Some(included.clone());
        be._test_project_files = Some(vec![]);
        be.parse_document(&source);
        be.parse_document(&included);

        let completions = get_completions(
            &source,
            &Position {
                line: 3,
                character: 11,
            },
            Some(be),
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 3,
                    character: 9,
                },
                end: Position {
                    line: 3,
                    character: 14,
                },
            },
            [
                Payee(
                    "IncludedPayee",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_formatting() {
        let source = textwrap::dedent(
            "
            2023/09/28 (743) Check Withdrawal
                ; Memo: CHK#743
                SVFCU:Personal   $-160.00
                SVFCU:Personal   $-16.00
                Expenses:Uncategorized
            ",
        );

        insta::assert_snapshot!(LedgerBackend::format(&source, false).unwrap(),
        @r"
        2023/09/28 (743) Check Withdrawal
            ; Memo: CHK#743
            SVFCU:Personal                          $-160.00
            SVFCU:Personal                           $-16.00
            Expenses:Uncategorized

        ",
        );
    }

    #[test]
    fn test_transaction_status() -> Result<()> {
        let source = textwrap::dedent(
            "
            24/01/02 Payee1
                Account

            24/02/03 ! Payee2
                Account

            24/02/03 * Mom & Dad
                Account
            ",
        );
        let mut backend = LedgerBackend::new();
        backend._test_project_files = Some(vec![]);
        backend.parse_document(&source);

        let status = backend.transaction_at_position_status(
            &source,
            &Position {
                line: 1,
                character: 1,
            },
        )?;

        insta::assert_debug_snapshot!(status,
        @r#"
        Some(
            NotCleared(
                Position {
                    line: 1,
                    character: 8,
                },
            ),
        )
        "#
        );

        let status = backend.transaction_at_position_status(
            &source,
            &Position {
                line: 4,
                character: 1,
            },
        )?;

        insta::assert_debug_snapshot!(status,
        @r#"
        Some(
            Pending(
                Range {
                    start: Position {
                        line: 4,
                        character: 8,
                    },
                    end: Position {
                        line: 4,
                        character: 10,
                    },
                },
            ),
        )
        "#
        );

        let status = backend.transaction_at_position_status(
            &source,
            &Position {
                line: 7,
                character: 1,
            },
        )?;

        insta::assert_debug_snapshot!(status,
        @r#"
        Some(
            Cleared(
                Range {
                    start: Position {
                        line: 7,
                        character: 8,
                    },
                    end: Position {
                        line: 7,
                        character: 10,
                    },
                },
            ),
        )
        "#
        );

        Ok(())
    }

    #[test]
    fn test_pending_transaction_status_ranges() -> Result<()> {
        let source = textwrap::dedent(
            "
            24/01/02 ! Payee1
                Account

            24/02/03 Payee2
                Account

            24/02/03 ! Mom & Dad
                Account

            24/02/03 Payee1
                Account
            ",
        );

        let mut backend = LedgerBackend::new();
        backend._test_project_files = Some(vec![]);
        backend.parse_document(&source);

        let ranges = backend.pending_transaction_status_ranges(&source)?;

        insta::assert_debug_snapshot!(ranges,
        @r#"
        [
            Range {
                start: Position {
                    line: 1,
                    character: 9,
                },
                end: Position {
                    line: 1,
                    character: 10,
                },
            },
            Range {
                start: Position {
                    line: 7,
                    character: 9,
                },
                end: Position {
                    line: 7,
                    character: 10,
                },
            },
        ]
        "#
        );

        Ok(())
    }

    #[test]
    fn test_node_xact_ranges() {
        let source = textwrap::dedent(
            "
            2023/09/28 Foo
                Bar   $-160.00
                Qux:Fiz:Wi
            ",
        );
        let mut be = LedgerBackend::new();
        be.parse_document(&source);

        // between ar in Bar
        // FIXME this does not work if placed at end of Bar, it matches to the
        // spaces between account and amount ... is that OK?
        let position = &Position {
            line: 2,
            character: 7,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "account",
            Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 7,
                },
            },
        )
        "#,
        );

        // end of Qux
        let position = &Position {
            line: 3,
            character: 7,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "account",
            Range {
                start: Position {
                    line: 3,
                    character: 4,
                },
                end: Position {
                    line: 3,
                    character: 14,
                },
            },
        )
        "#,
        );

        // end of Wi
        let position = &Position {
            line: 3,
            character: 14,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "account",
            Range {
                start: Position {
                    line: 3,
                    character: 4,
                },
                end: Position {
                    line: 3,
                    character: 14,
                },
            },
        )
        "#,
        );

        // middle of Foo
        let position = &Position {
            line: 1,
            character: 12,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "payee",
            Range {
                start: Position {
                    line: 1,
                    character: 11,
                },
                end: Position {
                    line: 1,
                    character: 14,
                },
            },
        )
        "#,
        );
    }

    #[test]
    fn test_node_xact_periodic_ranges() {
        // maintain a space after "weekly"
        let source = vec!["~ weekly ", "    Bar", ""].join("\n");
        let mut be = LedgerBackend::new();
        be.parse_document(&source);

        // after "weekly "
        let position = &Position {
            line: 0,
            character: 8,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "interval",
            Range {
                start: Position {
                    line: 0,
                    character: 2,
                },
                end: Position {
                    line: 0,
                    character: 8,
                },
            },
        )
        "#,
        );
    }

    #[test]
    fn test_node_directive_ranges() {
        let source = textwrap::dedent(
            "
            include foo
            ",
        );
        let mut be = LedgerBackend::new();
        be.parse_document(&source);

        // middle of foo
        let position = &Position {
            line: 1,
            character: 10,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "filename",
            Range {
                start: Position {
                    line: 1,
                    character: 8,
                },
                end: Position {
                    line: 1,
                    character: 11,
                },
            },
        )
        "#,
        );

        // end of foo
        let position = &Position {
            line: 1,
            character: 11,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "filename",
            Range {
                start: Position {
                    line: 1,
                    character: 8,
                },
                end: Position {
                    line: 1,
                    character: 11,
                },
            },
        )
        "#,
        );
    }

    #[test]
    fn test_node_ranges_at_end_of_file() {
        let source = textwrap::dedent(
            "
            2023/09/28 Foo
                Bar

            2023/09/28 Fii
                Buz",
        );
        let mut be = LedgerBackend::new();
        be.parse_document(&source);

        // end of Buz, which is also end of file
        let position = &Position {
            line: 5,
            character: 7,
        };
        insta::assert_debug_snapshot!(get_node_info(&source, &position, &mut be),
        @r#"
        (
            "account",
            Range {
                start: Position {
                    line: 5,
                    character: 4,
                },
                end: Position {
                    line: 5,
                    character: 7,
                },
            },
        )
        "#,
        );
    }

    //
    //
    //
    fn get_completions(
        source: &str,
        position: &Position,
        backend: Option<LedgerBackend>,
    ) -> (LspRange, Vec<LedgerCompletion>) {
        let mut backend = backend.unwrap_or_else(|| {
            let mut be = LedgerBackend::new();
            be._test_project_files = Some(vec![]);
            be.parse_document(&source);
            be
        });

        let mut visited = HashSet::new();
        match backend.completions_for_position("unused in test", &source, &position, &mut visited) {
            Ok(CompletionResult::Some {
                range,
                mut completions,
            }) => {
                completions.sort();
                (range, completions)
            }
            _ => panic!(),
        }
    }

    fn get_node_info(
        source: &str,
        position: &Position,
        backend: &mut LedgerBackend,
    ) -> (String, LspRange) {
        let node = backend.node_at_position(&source, &position).unwrap();
        let range = node.range();

        (
            node.kind().to_string(),
            LspRange {
                start: Position {
                    line: range.start_point.row as u32,
                    character: range.start_point.column as u32,
                },
                end: Position {
                    line: range.end_point.row as u32,
                    character: range.end_point.column as u32,
                },
            },
        )
    }
}
