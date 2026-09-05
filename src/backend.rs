use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tower_lsp::lsp_types::Range as LspRange;
use tower_lsp::lsp_types::*;
use tree_sitter::{Language, Node, Parser, Point, QueryMatch, Range, Tree};
use type_sitter::StreamingIterator;
use walkdir::WalkDir;

use crate::lsp::CheckLevel;
use crate::{backend_format, contents_of_path};

// Match `tag: value` and `:tag:`; ledger tags don't contain whitespace, and value tags
// must start the (trimmed) line but do not need to have a value.
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(r"(?<just_tag>(^| ):\S+:( |$))|(?<tag_with_value>^\S+:( |$))").unwrap()
});

fn substring(source: &[u8], start_byte: usize, end_byte: usize) -> Result<String> {
    Ok(
        std::str::from_utf8(&source[start_byte..end_byte.min(source.len())])?
            .trim()
            .to_string(),
    )
}

/// Get the start/end indices of a word that may be at `index`.
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

pub enum Tag {
    JustTag(String),
    WithValue(String),
}

impl Tag {
    pub fn name(&self) -> String {
        match self {
            Tag::JustTag(tag) => tag.clone(),
            Tag::WithValue(tag) => tag.clone(),
        }
    }
}

fn tags_from_note(note: &str) -> Vec<Tag> {
    // trim leading whitespace and comment chars
    // https://ledger-cli.org/doc/ledger3.html#Commenting-on-your-Journal
    let trimmed = note.trim_start_matches([' ', '\t', ';', '#', '%', '|', '*']);

    log::debug!("note content: {note:?}");
    log::debug!("trimmed: {trimmed:?}");

    let captures = TAG_RE.captures(trimmed);
    log::debug!("captures: {captures:?}");

    match captures {
        Some(captures) if captures.name("just_tag").is_some() => captures["just_tag"]
            .trim()
            .split(':')
            .filter(|tag| !tag.is_empty())
            .map(|tag| Tag::JustTag(tag.to_string()))
            .collect(),
        Some(captures) if captures.name("tag_with_value").is_some() => {
            vec![Tag::WithValue(
                captures["tag_with_value"]
                    .trim_end_matches([' ', ':'])
                    .to_string(),
            )]
        }
        Some(captures) => {
            log::error!("tag regex failure; this should be unreachable: {captures:?}");
            Vec::new()
        }
        None => Vec::new(),
    }
}

#[derive(Debug)]
pub enum LocationBasedResult<T> {
    /// Results were found for the node at this location.
    Some {
        /// Document range of the matching node.
        range: LspRange,
        /// Results for the matching node.
        results: Vec<T>,
    },

    /// No results were found for the node at this location.
    None,

    /// There is no node at this location.
    NoNode(String),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct LedgerLocation {
    pub file: PathBuf,
    pub range: LedgerRange,
}

#[derive(Debug, Eq, PartialEq)]
pub struct LedgerRange(pub LspRange);

impl Hash for LedgerRange {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.start.character.hash(state);
        self.0.start.line.hash(state);
        self.0.end.character.hash(state);
        self.0.end.line.hash(state);
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct LedgerHover(pub Hover);

impl Hash for LedgerHover {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let HoverContents::Scalar(MarkedString::String(ref s)) = self.0.contents {
            // we only use HoverContents::Scalar(MarkedString::String())
            s.hash(state)
        } else {
            #[cfg(debug_assertions)]
            unreachable!(
                "[unreachable] unexpected HoverContents variant {:?}",
                self.0.contents
            );
            #[cfg(not(debug_assertions))]
            log::warn!(
                "[unreachable] unexpected HoverContents variant {:?}",
                self.0.contents
            );
        }
    }
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

pub struct DiagnosticsParams {
    pub check_level: CheckLevel,
    pub check_payees: bool,
}

#[derive(Clone)]
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
                if parent.id() == node.id() {
                    // weird loop! bug? why would the parent node id == the
                    // child node id?
                    return Ok(None);
                }
                node = parent;
            } else {
                // dbg!(position, node.kind(), node.range());
                return Ok(None);
            }
        }
        // dbg!(position, node.kind(), node.range(), node.to_sexp());

        let mut date_node = None;
        let mut status_node = None;
        let mut code_or_payee_node = None;

        let mut cursor = node.walk();
        for node in node.named_children(&mut cursor) {
            if node.kind() == "date" || node.kind() == "effective_date" {
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

    /// Get completions relevant to `position` in the the `content` document.
    ///
    /// `buffer_path` and `visited` are used to track included documents
    pub fn completions_for_position(
        &mut self,
        buffer_path: &Path,
        content: &str,
        position: &Position,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<LocationBasedResult<LedgerCompletion>> {
        let mut completions: HashSet<LedgerCompletion> = HashSet::new();

        let node = match self.node_at_position(content, position) {
            Some(node) => node,
            None => {
                return Ok(LocationBasedResult::NoNode(format!(
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

        log::debug!("{:?} Node: {} {:?}", position, node.kind(), node.range());
        log::debug!("line: {line_content:?}");
        log::debug!("posn: {}^", " ".repeat(position.character as usize));

        match node.kind() {
            "account" => self.filter_nodes(
                &mut completions,
                buffer_path,
                "(account) @account",
                1,
                content,
                &|_node, account, _, _, _| {
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

            "payee" => self.filter_nodes(
                &mut completions,
                buffer_path,
                "(payee) @payee",
                1,
                content,
                &|_node, payee, _, _, _| {
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

                self.filter_nodes(
                    &mut completions,
                    buffer_path,
                    "(note) @note",
                    1,
                    content,
                    &|_node, note, _, _, _| {
                        if note == current_node_content {
                            // don't include current node content
                            return Vec::new();
                        }

                        tags_from_note(&note)
                            .iter()
                            .map(|tag| match tag {
                                Tag::JustTag(tag) => format!(":{tag}:"),
                                Tag::WithValue(tag) => format!("{tag}: "),
                            })
                            .map(LedgerCompletion::Tag)
                            .collect()
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

            _ => return Ok(LocationBasedResult::None),
        };

        // remove trailing newline from range to replace
        if range.end_point.column == 0 && range.start_point.row != range.end_point.row {
            range.end_byte -= 1;
            range.end_point.row -= 1;
            range.end_point.column = content.lines().nth(range.end_point.row).unwrap_or("").len();
        }

        Ok(LocationBasedResult::Some {
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
            results: completions.into_iter().collect(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn filter_nodes<F, I, T>(
        &mut self,
        results: &mut HashSet<T>,
        buffer_path: &Path,
        query: &str,
        primary_capture_index: u32,
        buffer_contents: &str,
        filter_fn: &F,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<()>
    where
        // first capture node, node content, file name, file content, matches
        F: Fn(&Node, String, &Path, &str, &QueryMatch) -> I,
        I: IntoIterator<Item = T>,
        T: Hash + Eq,
    {
        let current_dir = match buffer_path.parent() {
            Some(dir) => dir,
            None => {
                // TODO ??
                return Err(anyhow!(
                    "[completions] Buffer has no parent dir? {}",
                    buffer_path.display()
                ));
            }
        };

        let tree = match self.trees_cache.get(buffer_contents) {
            Some(tree) => tree.clone(),
            None => {
                // self.parse_document(content);
                // self.trees_cache.get(content).unwrap().clone()
                return Err(anyhow!(
                    "no tree found for contents of file '{}'",
                    buffer_path.display()
                ));
            }
        };

        let ts_query = tree_sitter::Query::new(
            match self.parser()?.language() {
                Some(ref language) => language,
                None => bail!("getting tree-sitter language"),
            },
            format!("(filename) @filename {query}").as_str(),
        )?;
        let mut cursor = tree_sitter::QueryCursor::new();

        let source = buffer_contents.as_bytes();
        let mut matches = cursor.matches(&ts_query, tree.root_node(), source);
        while let Some(m) = matches.next() {
            // query as passed in
            for node in m.nodes_for_capture_index(primary_capture_index) {
                let capture_text = substring(source, node.start_byte(), node.end_byte())?;
                results.extend(filter_fn(
                    &node,
                    capture_text,
                    buffer_path,
                    buffer_contents,
                    m,
                ));
            }

            // (filename) @filename
            for n in m.nodes_for_capture_index(0) {
                let filename = substring(source, n.start_byte(), n.end_byte())?;

                let path = Path::new(&filename);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    current_dir.join(path)
                };

                if visited.contains(&path) {
                    continue;
                } else {
                    visited.insert(path.clone());
                }

                let included_content = self
                    ._test_included_content
                    .as_ref()
                    .map_or_else(
                        || contents_of_path(&path),
                        |content| Ok(content.to_string()),
                    )
                    .unwrap_or_else(|_| String::new());

                self.parse_document(&included_content);

                self.filter_nodes(
                    results,
                    &path,
                    query,
                    primary_capture_index,
                    &included_content,
                    filter_fn,
                    visited,
                )?;
            }
        }

        Ok(())
    }

    fn completions_insert_project_files(
        &self,
        completions: &mut HashSet<LedgerCompletion>,
        buffer_path: &Path,
    ) -> Result<()> {
        let current_dir = match buffer_path.parent() {
            Some(dir) => dir,
            None => {
                // TODO ??
                return Err(anyhow!(
                    "[completions] Buffer has no parent dir? {}",
                    buffer_path.display()
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

    pub fn diagnostics(
        &mut self,
        buffer_path: &Path,
        content: &str,
        params: &DiagnosticsParams,
    ) -> Vec<Diagnostic> {
        self.diagnostics_elided_amounts(buffer_path, content)
            .into_iter()
            .chain(self.diagnostics_undefined_values(buffer_path, content, params))
            .chain(self.diagnostics_includes(buffer_path, content))
            .collect()
    }

    pub fn diagnostics_undefined_values(
        &mut self,
        buffer_path: &Path,
        content: &str,
        params: &DiagnosticsParams,
    ) -> Vec<Diagnostic> {
        let DiagnosticsParams {
            check_level,
            check_payees,
        } = params;

        #[derive(Debug, Eq, PartialEq, Hash)]
        enum DefinedValue {
            Account(String),
            Commodity(String),
            Payee(String),
            Tag(String),
        }

        let severity = match check_level {
            CheckLevel::Off => return Vec::new(),
            CheckLevel::Strict => DiagnosticSeverity::WARNING,
            CheckLevel::Pedantic => DiagnosticSeverity::ERROR,
        };

        // collect defined accounts and payees
        let (defined_values, mut visited) = {
            let mut defined_values: HashSet<DefinedValue> = HashSet::new();
            let mut visited: HashSet<PathBuf> = HashSet::new();
            let query = "
            (directive
               	[
                	(account_directive (account) @account
                        (account_subdirective (alias_subdirective) @account_alias)*
                    )
                    (payee_directive (payee) @payee
                        (payee_subdirective (alias_subdirective) @payee_alias)*
                    )
                    (commodity_directive (commodity) @commodity
                        (commodity_subdirective (alias_subdirective) @commodity_alias)*
                    )
                    (tag_directive) @tag
                ]
            ) @directive
            ";

            let Ok(()) = self.filter_nodes(
                &mut defined_values,
                buffer_path,
                query,
                8, // 1-based index of @directive
                content,
                &|_node, _node_content, _buffer_path, buffer_contents, matches| {
                    #[allow(clippy::type_complexity)]
                    let captures: [(fn(String) -> DefinedValue, &str); 7] = [
                        // must be in same order as captures, above
                        (DefinedValue::Account, ""),
                        (DefinedValue::Account, "alias "),
                        (DefinedValue::Payee, ""),
                        (DefinedValue::Payee, "alias "),
                        (DefinedValue::Commodity, ""),
                        (DefinedValue::Commodity, "alias "),
                        (DefinedValue::Tag, "tag "),
                    ];

                    (1u32..)
                        .zip(captures)
                        .flat_map(|(i, (make, prefix))| {
                            matches.nodes_for_capture_index(i).filter_map(move |node| {
                                substring(
                                    buffer_contents.as_bytes(),
                                    node.start_byte(),
                                    node.end_byte(),
                                )
                                .ok()
                                .and_then(|s| {
                                    if !prefix.is_empty() {
                                        s.strip_prefix(prefix).map(str::trim).map(str::to_string)
                                    } else {
                                        Some(s)
                                    }
                                })
                                .map(make)
                            })
                        })
                        .collect::<Vec<_>>()
                },
                &mut visited,
            ) else {
                return Vec::new();
            };

            (defined_values, visited)
        };

        #[derive(Eq, PartialEq, Hash)]
        struct TempDiagnostic((Range, String));
        let mut diagnostics: HashSet<TempDiagnostic> = HashSet::new();
        let query = "
            (plain_xact
              (payee)? @payee
              [
                (posting
             	  (account) @account
                  (amount (commodity) @commodity)?
                  (note)? @note
                )
               	(note) @note
              ]
            ) @xact
            ";

        let Ok(()) = self.filter_nodes(
            &mut diagnostics,
            buffer_path,
            query,
            5, // 1-based index of @xact, not counting duplicates
            content,
            &|_node, _node_content, _buffer_path, buffer_contents, matches| {
                #[allow(clippy::type_complexity)]
                let captures: [(&str, fn(String) -> DefinedValue, bool); 4] = [
                    // must be in same order as captures, above; do not need to
                    // match capture names
                    ("payee", DefinedValue::Payee, *check_payees),
                    ("account", DefinedValue::Account, true),
                    ("commodity", DefinedValue::Commodity, true),
                    ("tag", DefinedValue::Tag, true),
                ];

                let defined_values = &defined_values;
                (1u32..)
                    .zip(captures)
                    .filter(|(_, (_, _, should_report))| *should_report)
                    .flat_map(|(i, (name, make, _))| {
                        matches.nodes_for_capture_index(i).flat_map(move |node| {
                            let value = match substring(
                                buffer_contents.as_bytes(),
                                node.start_byte(),
                                node.end_byte(),
                            ) {
                                Ok(s) => s,
                                Err(_) => return Vec::new(),
                            };

                            let values = if node.kind() == "note" {
                                tags_from_note(&value).iter().map(|t| t.name()).collect()
                            } else if node.kind() == "account" {
                                vec![value.trim_matches(['[', ']', '(', ')']).to_string()]
                            } else {
                                vec![value]
                            };

                            values
                                .iter()
                                .filter_map(|value| {
                                    if defined_values.contains(&make(value.clone())) {
                                        None
                                    } else {
                                        Some(TempDiagnostic((
                                            // FIXME: tags should only highlight the tag, not the whole note
                                            node.range(),
                                            format!("Undefined {name}: {value}"),
                                        )))
                                    }
                                })
                                .collect()
                        })
                    })
                    .collect::<Vec<_>>()
            },
            // HACK: reusing visited prevents filter_nodes() from crawling into
            // (and diagnosing) included files
            &mut visited,
        ) else {
            return Vec::new();
        };

        diagnostics
            .into_iter()
            .map(|TempDiagnostic((range, message))| {
                let mut diag = Diagnostic::new_simple(lsp_range_from_ts_range(range), message);
                // TODO: provide config to change severity, none if off, warning if strict, error if pedantic
                diag.severity = Some(severity);
                diag.source = Some("ledger-ls".to_string());
                diag
            })
            .collect()
    }

    pub fn diagnostics_elided_amounts(
        &mut self,
        buffer_path: &Path,
        content: &str,
    ) -> Vec<Diagnostic> {
        #[derive(Eq, PartialEq, Hash)]
        struct TempDiagnostic((Range, String));
        let mut diagnostics: HashSet<TempDiagnostic> = HashSet::new();
        let mut visited: HashSet<PathBuf> = HashSet::new();
        let query = "(plain_xact) @xact";

        // TODO: this is prob crawling included files, too; but should it not be?
        let Ok(()) = self.filter_nodes(
            &mut diagnostics,
            buffer_path,
            query,
            1,
            content,
            &|node, _node_content, _buffer_path, _buffer_contents, _matches| {
                let mut cursor = node.walk();
                let count = node
                    .named_children(&mut cursor)
                    .filter(|child| child.kind() == "posting")
                    .filter(|posting| {
                        let mut cursor = node.walk();
                        let mut children = posting.named_children(&mut cursor);
                        children
                            .find(|child| {
                                child.kind() == "amount" || child.kind() == "balance_assertion"
                            })
                            .is_none()
                    })
                    .count();

                if count > 1 {
                    Some(TempDiagnostic((
                        node.range(),
                        format!("Only 1 elided amount allowed per transaction. Found {count}."),
                    )))
                } else {
                    None
                }
            },
            &mut visited,
        ) else {
            return Vec::new();
        };

        diagnostics
            .into_iter()
            .map(|TempDiagnostic((range, message))| {
                let mut diag = Diagnostic::new_simple(lsp_range_from_ts_range(range), message);
                diag.severity = Some(DiagnosticSeverity::ERROR);
                diag.source = Some("ledger-ls".to_string());
                diag
            })
            .collect()
    }

    fn diagnostics_includes(&self, buffer_path: &Path, content: &str) -> Vec<Diagnostic> {
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
                        let dir = buffer_path.parent()?;
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

    pub fn references_for_position(
        &mut self,
        buffer_path: &Path,
        content: &str,
        position: &Position,
        include_declaration: bool,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<LocationBasedResult<LedgerLocation>> {
        let mut locations: HashSet<LedgerLocation> = HashSet::new();

        let node = match self.node_at_position(content, position) {
            Some(node) => node,
            None => {
                return Ok(LocationBasedResult::NoNode(format!(
                    "No node found at position {position:?}"
                )));
            }
        };
        let current_node_content = substring(
            content.as_bytes(),
            node.range().start_byte,
            node.range().end_byte,
        )?;
        let range = node.range();

        let line_content = content.lines().nth(position.line as usize).unwrap_or("");

        log::debug!("{:?} Node: {} {:?}", position, node.kind(), node.range());
        log::debug!("line: {line_content:?}");
        log::debug!("posn: {}^", " ".repeat(position.character as usize));

        let query = match node.kind() {
            // Accounts mostly show up in postings and `account`/`A` directives
            // (declarations), but can also be used in `bucket` directives and
            // timeclock journals.
            //
            // This matches all accounts when including decls, or only accounts
            // within postings if excluding. This is a simple and effective
            // approach, but it comes at the cost of not really supporting
            // bucket/timeclock.
            "account" if include_declaration => "(account) @account",
            "account" => "(posting (account) @account)",

            // Same as above, but payee is only used in fewer places.
            "payee" if include_declaration => "(payee) @payee",
            "payee" => "(plain_xact (payee) @payee)",

            _ => return Ok(LocationBasedResult::None),
        };

        self.filter_nodes(
            &mut locations,
            buffer_path,
            query,
            1,
            content,
            &|node, node_content, buffer_path, _, _| {
                if node_content == current_node_content {
                    Some(LedgerLocation {
                        file: buffer_path.to_path_buf(),
                        range: LedgerRange(lsp_range_from_ts_range(node.range())),
                    })
                } else {
                    // don't include current node content
                    None
                }
            },
            visited,
        )?;

        Ok(LocationBasedResult::Some {
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
            results: locations.into_iter().collect(),
        })
    }

    pub fn hovers_for_position(
        &mut self,
        buffer_path: &Path,
        content: &str,
        position: &Position,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<LocationBasedResult<LedgerHover>> {
        let mut hovers: HashSet<LedgerHover> = HashSet::new();

        let node = match self.node_at_position(content, position) {
            Some(node) => node,
            None => {
                return Ok(LocationBasedResult::NoNode(format!(
                    "No node found at position {position:?}"
                )));
            }
        };
        let current_node_content = substring(
            content.as_bytes(),
            node.range().start_byte,
            node.range().end_byte,
        )?;
        let range = node.range();

        let line_content = content.lines().nth(position.line as usize).unwrap_or("");

        log::debug!("{:?} Node: {} {:?}", position, node.kind(), node.range());
        log::debug!("line: {line_content:?}");
        log::debug!("posn: {}^", " ".repeat(position.character as usize));
        log::debug!("current_node_content: {current_node_content}");

        match node.kind() {
            "account" => self.filter_nodes(
                &mut hovers,
                buffer_path,
                // sibling order is important in tree-sitter queries, so we need
                // to support notes both before and after aliases; Ledger only
                // supports 1 note/account, but it supports multiple aliases.
                "
                (account_directive
                    (account) @account
                    (account_subdirective (alias_subdirective) @alias)*
                    (account_subdirective (note_subdirective) @note)?
                    (account_subdirective (alias_subdirective) @alias)*
                )
                ",
                1,
                content,
                &|_node, account, _buffer_path, buffer_contents, matches| {
                    // capture indices:
                    //  1 => @account
                    //  2 => @alias (NOTE: includes 2nd @alias; tree-sitter combines them into a single capture)
                    //  3 => @note

                    let capture_contents = |i, prefix| {
                        matches
                            .nodes_for_capture_index(i)
                            .filter_map(|node_node| {
                                substring(
                                    buffer_contents.as_bytes(),
                                    node_node.start_byte(),
                                    node_node.end_byte(),
                                )
                                .ok()
                                .and_then(|s| {
                                    s.strip_prefix(prefix).map(str::trim).map(str::to_string)
                                })
                            })
                            .collect::<Vec<_>>()
                    };

                    let get_note = || capture_contents(3, "note ").join(" ").trim().to_string();
                    let get_aliases = || capture_contents(2, "alias ");

                    log::debug!("account: {account}");

                    let is_match_and_alias = if account == current_node_content {
                        Some(false)
                    } else {
                        let aliases = get_aliases();
                        log::debug!("aliases: {aliases:?}");
                        aliases.contains(&current_node_content).then_some(true)
                    };

                    is_match_and_alias
                        .map(|is_alias| {
                            if is_alias {
                                format!("aliased from `{current_node_content}`")
                            } else {
                                String::new()
                            }
                        })
                        .map(|alias_content| {
                            let note_content = get_note();
                            log::debug!("note_content: {note_content:?}");
                            log::debug!("alias_content: {alias_content:?}");
                            let hover_content = format!(
                                "`{account}`{hr}{note}{br}{alias_content}",
                                hr = if !note_content.is_empty() || !alias_content.is_empty() {
                                    "\n***"
                                } else {
                                    ""
                                },
                                note = if !note_content.is_empty() {
                                    format!("\n*{note_content}*")
                                } else {
                                    String::new()
                                },
                                br = match (note_content.is_empty(), alias_content.is_empty()) {
                                    (true, true) => "",
                                    (true, false) => "\n",
                                    (false, true) => "",
                                    (false, false) => "  \n",
                                },
                            );
                            LedgerHover(Hover {
                                contents: HoverContents::Scalar(MarkedString::String(
                                    hover_content,
                                )),
                                range: None,
                            })
                        })
                },
                visited,
            )?,

            "commodity" => self.filter_nodes(
                &mut hovers,
                buffer_path,
                // sibling order is important; see account query, above
                "
                (commodity_directive
                    (commodity) @commodity
                    (commodity_subdirective (alias_subdirective) @alias)*
                    (commodity_subdirective (note_subdirective) @note)?
                    (commodity_subdirective (alias_subdirective) @alias)*
                )
                ",
                1,
                content,
                &|_node, commodity, _buffer_path, buffer_contents, matches| {
                    // capture indices:
                    //  1 => @commodity
                    //  2 => @alias (NOTE: includes 2nd @alias)
                    //  3 => @note

                    let capture_contents = |i, prefix| {
                        matches
                            .nodes_for_capture_index(i)
                            .filter_map(|node_node| {
                                substring(
                                    buffer_contents.as_bytes(),
                                    node_node.start_byte(),
                                    node_node.end_byte(),
                                )
                                .ok()
                                .and_then(|s| {
                                    s.strip_prefix(prefix).map(str::trim).map(str::to_string)
                                })
                            })
                            .collect::<Vec<_>>()
                    };

                    let get_note = || capture_contents(3, "note ").join(" ").trim().to_string();
                    let get_aliases = || capture_contents(2, "alias ");

                    log::debug!("commodity: {commodity}");

                    let is_match_and_alias = if commodity == current_node_content {
                        Some(false)
                    } else {
                        let aliases = get_aliases();
                        log::debug!("aliases: {aliases:?}");
                        aliases.contains(&current_node_content).then_some(true)
                    };

                    is_match_and_alias
                        .map(|is_alias| {
                            if is_alias {
                                format!("aliased from `{current_node_content}`")
                            } else {
                                String::new()
                            }
                        })
                        .map(|alias_content| {
                            let note_content = get_note();
                            log::debug!("note_content: {note_content:?}");
                            log::debug!("alias_content: {alias_content:?}");
                            let hover_content = format!(
                                "`{commodity}`{hr}{note}{br}{alias_content}",
                                hr = if !note_content.is_empty() || !alias_content.is_empty() {
                                    "\n***"
                                } else {
                                    ""
                                },
                                note = if !note_content.is_empty() {
                                    format!("\n*{note_content}*")
                                } else {
                                    String::new()
                                },
                                br = match (note_content.is_empty(), alias_content.is_empty()) {
                                    (true, true) => "",
                                    (true, false) => "\n",
                                    (false, true) => "",
                                    (false, false) => "  \n",
                                },
                            );
                            LedgerHover(Hover {
                                contents: HoverContents::Scalar(MarkedString::String(
                                    hover_content,
                                )),
                                range: None,
                            })
                        })
                },
                visited,
            )?,

            "payee" => self.filter_nodes(
                &mut hovers,
                buffer_path,
                "
                (payee_directive
                    (payee) @payee
                    (payee_subdirective (alias_subdirective) @alias)*
                )
                ",
                1, // 1-based index of @payee
                content,
                &|_node, payee, _buffer_path, buffer_contents, matches| {
                    // capture indices:
                    //  1 => @payee
                    //  2 => @alias

                    let capture_contents = |i, prefix| {
                        matches
                            .nodes_for_capture_index(i)
                            .filter_map(|node_node| {
                                substring(
                                    buffer_contents.as_bytes(),
                                    node_node.start_byte(),
                                    node_node.end_byte(),
                                )
                                .ok()
                                .and_then(|s| {
                                    s.strip_prefix(prefix).map(str::trim).map(str::to_string)
                                })
                            })
                            .collect::<Vec<_>>()
                    };

                    let get_aliases = || capture_contents(2, "alias ");

                    log::debug!("payee: {payee}");

                    let is_match_and_alias = if payee == current_node_content {
                        Some(false)
                    } else {
                        let aliases = get_aliases();
                        log::debug!("aliases: {aliases:?}");
                        aliases.contains(&current_node_content).then_some(true)
                    };

                    is_match_and_alias
                        .map(|is_alias| {
                            if is_alias {
                                format!("`{payee}`\n***\naliased from `{current_node_content}`")
                            } else {
                                format!("`{payee}`")
                            }
                        })
                        .map(|hover_content| {
                            LedgerHover(Hover {
                                contents: HoverContents::Scalar(MarkedString::String(
                                    hover_content,
                                )),
                                range: None,
                            })
                        })
                },
                visited,
            )?,

            // TODO: support commodities
            _ => return Ok(LocationBasedResult::None),
        };

        Ok(LocationBasedResult::Some {
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
            results: hovers.into_iter().collect(),
        })
    }

    /// Get the smallest named node at the given position.
    fn node_at_position(&mut self, content: &str, position: &Position) -> Option<Node<'_>> {
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

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_diagnostics_elided_amounts() {
        let source = textwrap::dedent(
            "
            account Account
            payee Payee

            2024/01/02 Payee
                Account  1 ; 1 elided amount
                Account

            2024/01/02 Payee
                Account  ; 2 elided amounts
                Account

            2024/01/02 Payee
                Account  = 1 ; balance assertion counts as amount
                Account
            ",
        );

        let diagnostics = get_diagnostics(
            &source,
            DiagnosticsParams {
                check_level: CheckLevel::Strict,
                check_payees: false,
            },
            None,
        );

        insta::assert_debug_snapshot!(diagnostics,
            @r#"
        [
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 8,
                        character: 0,
                    },
                    end: Position {
                        line: 11,
                        character: 0,
                    },
                },
                severity: Some(
                    Error,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Only 1 elided amount allowed per transaction. Found 2.",
                related_information: None,
                tags: None,
                data: None,
            },
        ]
        "#
        );
    }

    #[test]
    fn test_diagnostics_undefined_values() {
        let source = textwrap::dedent(
            "
            account Account1

            2024/01/02 Payee1
                Account1    $1
                (Account1)  1
                [Account1]  1
                Account2    1
                (Account2)  1
                [Account2]  1
            ",
        );

        let diagnostics = get_diagnostics(
            &source,
            DiagnosticsParams {
                check_level: CheckLevel::Strict,
                check_payees: false,
            },
            None,
        );

        insta::assert_debug_snapshot!(diagnostics,
            @r#"
        [
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 4,
                        character: 16,
                    },
                    end: Position {
                        line: 4,
                        character: 17,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined commodity: $",
                related_information: None,
                tags: None,
                data: None,
            },
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 7,
                        character: 4,
                    },
                    end: Position {
                        line: 7,
                        character: 12,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined account: Account2",
                related_information: None,
                tags: None,
                data: None,
            },
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 8,
                        character: 4,
                    },
                    end: Position {
                        line: 8,
                        character: 14,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined account: Account2",
                related_information: None,
                tags: None,
                data: None,
            },
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 9,
                        character: 4,
                    },
                    end: Position {
                        line: 9,
                        character: 14,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined account: Account2",
                related_information: None,
                tags: None,
                data: None,
            },
        ]
        "#
        );
    }

    #[test]
    fn test_diagnostics_undefined_values_with_aliases() {
        let source = textwrap::dedent(
            "
            account Account
                alias Account1

            commodity USD
                alias $

            payee Payee
                alias Payee1

            2024/01/02 Payee1
                Account1    $1
                Account
            ",
        );

        // Payee1, $ and Account1 are all aliases; should produce no diagnostics

        let diagnostics = get_diagnostics(
            &source,
            DiagnosticsParams {
                check_level: CheckLevel::Strict,
                check_payees: true,
            },
            None,
        );

        insta::assert_debug_snapshot!(diagnostics, @r"[]");
    }

    #[test]
    fn test_diagnostics_undefined_tags() {
        let source = textwrap::dedent(
            "
            account Account
            tag Foo
            tag Bar

            2024/01/02 Payee
                ; Foo: value
                Account  1 ; :Qux:
                ; Zip: value
                Account  1 ; :Bar:
                ; :Yurt:
                Account
            ",
        );

        let diagnostics = get_diagnostics(
            &source,
            DiagnosticsParams {
                check_level: CheckLevel::Strict,
                check_payees: false,
            },
            None,
        );

        insta::assert_debug_snapshot!(diagnostics,
            @r#"
        [
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 7,
                        character: 15,
                    },
                    end: Position {
                        line: 7,
                        character: 22,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined tag: Qux",
                related_information: None,
                tags: None,
                data: None,
            },
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 8,
                        character: 4,
                    },
                    end: Position {
                        line: 8,
                        character: 16,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined tag: Zip",
                related_information: None,
                tags: None,
                data: None,
            },
            Diagnostic {
                range: Range {
                    start: Position {
                        line: 10,
                        character: 4,
                    },
                    end: Position {
                        line: 10,
                        character: 12,
                    },
                },
                severity: Some(
                    Warning,
                ),
                code: None,
                code_description: None,
                source: Some(
                    "ledger-ls",
                ),
                message: "Undefined tag: Yurt",
                related_information: None,
                tags: None,
                data: None,
            },
        ]
        "#
        );
    }

    #[test]
    fn test_completions_payees() {
        let source = textwrap::dedent(
            "
            2024/01/02 Payee1
                Account

            2024/02/03 Payee2
                Account

            2024/02/03 Mom & Dad
                Account
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 1,
                character: 12,
            },
            None,
        );

        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 1,
                    character: 11,
                },
                end: Position {
                    line: 1,
                    character: 17,
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
            2024/01/02 Payee1
                Account1  $1
                Account2

            2024/02/03 Payee2
                Account2  $2
                Account3

            2024/02/03 Mom & Dad
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

    /// Test that `tag: value` style tags are offered as completions.
    #[test]
    fn test_completions_tags_with_value() {
        init_logging();

        let source = textwrap::dedent(
            "
            2024/01/02 Payee
                ; Tag1: foo
                ; Tag2: bar
                ;
                ; T
                Account
            ",
        );

        // blank line, after the ;
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
                    "Tag1: ",
                ),
                Tag(
                    "Tag2: ",
                ),
            ],
        )
        "#
        );

        // Tag2, after the T
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
                    "Tag1: ",
                ),
            ],
        )
        "#
        );

        // just the T, after the T
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
                    "Tag1: ",
                ),
                Tag(
                    "Tag2: ",
                ),
            ],
        )
        "#
        );
    }

    /// Test that `:tag:` style tags are offered as completions.
    #[test]
    fn test_completions_tags() {
        init_logging();

        let source = textwrap::dedent(
            "
            2024/01/02 Payee
                ; :Tag1:
                ; :Tag2:
                ;
                ; :
                ; :T
                Account
            ",
        );

        // just :, after the :
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
                    ":Tag1:",
                ),
                Tag(
                    ":Tag2:",
                ),
            ],
        )
        "#
        );

        // :Tag2:, after the T
        let completions = get_completions(
            &source,
            &Position {
                line: 3,
                character: 8,
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
                    character: 11,
                },
            },
            [
                Tag(
                    ":Tag1:",
                ),
            ],
        )
        "#
        );

        // just the :T, after the T
        let completions = get_completions(
            &source,
            &Position {
                line: 6,
                character: 8,
            },
            None,
        );
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 6,
                    character: 6,
                },
                end: Position {
                    line: 6,
                    character: 8,
                },
            },
            [
                Tag(
                    ":Tag1:",
                ),
                Tag(
                    ":Tag2:",
                ),
            ],
        )
        "#
        );
    }

    #[test]
    fn test_completions_tags_are_deduped() {
        init_logging();

        let source = textwrap::dedent(
            "
            2024/01/02 Payee
                ; :
                ; Tag1: with value
                ; Tag1:
                ; :Tag2:
                ; :Tag3:Tag2:
                Account  $1
                Account
            ",
        );

        let completions = get_completions(
            &source,
            &Position {
                line: 2,
                character: 6,
            },
            None,
        );
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 2,
                    character: 6,
                },
                end: Position {
                    line: 2,
                    character: 6,
                },
            },
            [
                Tag(
                    ":Tag2:",
                ),
                Tag(
                    ":Tag3:",
                ),
                Tag(
                    "Tag1: ",
                ),
            ],
        )
        "#
        );
    }

    /// Confirm that our tag matching code aligns with ledger's; ie, that we
    /// aren't missing anything nor including anything extra.
    #[test]
    fn test_completions_tags_align_with_ledger() {
        init_logging();

        // Paste this into a ledger file and run `ledger -f <file> tags` to
        // display which are supported. Note that `Tag9:Tag10:` looks like a
        // bug, but is valid according to ledger.
        let source = textwrap::dedent(
            "
            2024/01/02 Payee
                ; :
                ;
                ; :Tag1:
                ; :Tag2:Tag3:
                ; NotTag4:NotTag5:NotTag6
                ; :NotTag7:NotTag8
                ; Tag9:Tag10:
                ; NotTag11 :Tag12:Tag13: NotTag14
                ; http://example.com:80
                ; :Tag15: :NotATag16:
                ;
                ; Tag30: Value30
                ; Tag31:
                ; NotA Tag32: Value32
                ; NotATag33:Value33
                ; http://example.com
                Account  $1
                Account
            ",
        );

        // on then line with just :, after the :
        let completions = get_completions(
            &source,
            &Position {
                line: 2,
                character: 7,
            },
            None,
        );
        insta::assert_debug_snapshot!(completions,
        @r#"
        (
            Range {
                start: Position {
                    line: 2,
                    character: 6,
                },
                end: Position {
                    line: 2,
                    character: 7,
                },
            },
            [
                Tag(
                    ":Tag12:",
                ),
                Tag(
                    ":Tag13:",
                ),
                Tag(
                    ":Tag15:",
                ),
                Tag(
                    ":Tag1:",
                ),
                Tag(
                    ":Tag2:",
                ),
                Tag(
                    ":Tag3:",
                ),
                Tag(
                    "Tag30: ",
                ),
                Tag(
                    "Tag31: ",
                ),
                Tag(
                    "Tag9:Tag10: ",
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
            2024/01/02 IncludedPayee
                IncludedAccount
            ",
        );
        let source = textwrap::dedent(
            "
            include foo.ledger

            2024/01/02 Payee
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
                    character: 11,
                },
                end: Position {
                    line: 3,
                    character: 16,
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
    fn test_references_payees() {
        let source = textwrap::dedent(
            "
            2024/01/02 Payee1
                Account1

            2024/02/03 Payee2
                Account2

            2024/02/03 Payee1
                Account1
            ",
        );

        let completions = get_references(
            &source,
            &Position {
                line: 1,
                character: 12,
            },
            true,
            None,
        );

        insta::assert_debug_snapshot!(completions,
        @r###"
        (
            Range {
                start: Position {
                    line: 1,
                    character: 11,
                },
                end: Position {
                    line: 1,
                    character: 17,
                },
            },
            [
                LedgerLocation {
                    file: "unused in test",
                    range: LedgerRange(
                        Range {
                            start: Position {
                                line: 1,
                                character: 11,
                            },
                            end: Position {
                                line: 1,
                                character: 17,
                            },
                        },
                    ),
                },
                LedgerLocation {
                    file: "unused in test",
                    range: LedgerRange(
                        Range {
                            start: Position {
                                line: 7,
                                character: 11,
                            },
                            end: Position {
                                line: 7,
                                character: 17,
                            },
                        },
                    ),
                },
            ],
        )
        "###
        );
    }

    #[test]
    fn test_references_accounts() {
        let source = textwrap::dedent(
            "
            account Account1

            2024/01/02 Payee1
                Account1

            2024/02/03 Payee2
                Account2

            2024/02/03 Payee1
                Account1
            ",
        );

        {
            // references for Account1 WITH declaration
            let completions = get_references(
                &source,
                &Position {
                    line: 4,
                    character: 5,
                },
                true,
                None,
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 8,
                                },
                                end: Position {
                                    line: 1,
                                    character: 16,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
                                    character: 12,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 4,
                                },
                                end: Position {
                                    line: 10,
                                    character: 12,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }

        {
            // references for Account1 WITHOUT declaration
            let completions = get_references(
                &source,
                &Position {
                    line: 4,
                    character: 5,
                },
                false,
                None,
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
                                    character: 12,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 4,
                                },
                                end: Position {
                                    line: 10,
                                    character: 12,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }
    }

    #[test]
    fn test_references_from_included_files() {
        let included = textwrap::dedent(
            "
            2024/01/02 Payee1
                Account1
            ",
        );
        let source = textwrap::dedent(
            "
            include foo.ledger

            2024/01/02 Payee1
                Account1

            2024/02/03 Payee2
                Account2

            2024/02/03 Payee1
                Account1
            ",
        );

        let be = {
            let mut be = LedgerBackend::new();
            be._test_included_content = Some(included.clone());
            be._test_project_files = Some(vec![]);
            be.parse_document(&source);
            be.parse_document(&included);
            be
        };

        {
            // references for Account1, in first transaction
            let completions = get_references(
                &source,
                &Position {
                    line: 4,
                    character: 5,
                },
                true,
                Some(be.clone()),
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerLocation {
                        file: "foo.ledger",
                        range: LedgerRange(
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
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
                                    character: 12,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 4,
                                },
                                end: Position {
                                    line: 10,
                                    character: 12,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }

        {
            // references for Payee1, in third transaction
            let completions = get_references(
                &source,
                &Position {
                    line: 9,
                    character: 15,
                },
                true,
                Some(be),
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 9,
                        character: 11,
                    },
                    end: Position {
                        line: 9,
                        character: 17,
                    },
                },
                [
                    LedgerLocation {
                        file: "foo.ledger",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 11,
                                },
                                end: Position {
                                    line: 1,
                                    character: 17,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 11,
                                },
                                end: Position {
                                    line: 3,
                                    character: 17,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 11,
                                },
                                end: Position {
                                    line: 9,
                                    character: 17,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }
    }

    #[test]
    fn test_references_include_declaration() {
        let source = textwrap::dedent(
            "
            account Account1

            2024/01/02 Payee1
                Account1
            ",
        );

        {
            // refs for Account1 WITH declaration
            let completions = get_references(
                &source,
                &Position {
                    line: 4,
                    character: 5,
                },
                true,
                None,
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 8,
                                },
                                end: Position {
                                    line: 1,
                                    character: 16,
                                },
                            },
                        ),
                    },
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
                                    character: 12,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }

        {
            // refs for Account1 WITHOUT declaration
            let completions = get_references(
                &source,
                &Position {
                    line: 4,
                    character: 5,
                },
                false,
                None,
            );

            insta::assert_debug_snapshot!(completions,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerLocation {
                        file: "unused in test",
                        range: LedgerRange(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
                                    character: 12,
                                },
                            },
                        ),
                    },
                ],
            )
            "###
            );
        }
    }

    #[test]
    fn test_hover_accounts() {
        init_logging();

        {
            // account without alias or note
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    account Account1

                    2024/01/02 Payee1
                        Account1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 4,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 4,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // account with note but no alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    account Account1
                        note This is account 1

                    2024/01/02 Payee1
                        Account1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 5,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 5,
                        character: 4,
                    },
                    end: Position {
                        line: 5,
                        character: 12,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\n*This is account 1*",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // account with alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    account Account1
                        alias Act1

                    2024/01/02 Payee1
                        Act1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 5,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 5,
                        character: 4,
                    },
                    end: Position {
                        line: 5,
                        character: 8,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\naliased from `Act1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // account with note before alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    account Account1
                        note This is account 1
                        alias Act1

                    2024/01/02 Payee1
                        Act1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 6,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 4,
                    },
                    end: Position {
                        line: 6,
                        character: 8,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\n*This is account 1*  \naliased from `Act1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // account with note after alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    account Account1
                        alias Act1
                        note This is account 1

                    2024/01/02 Payee1
                        Act1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 6,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 4,
                    },
                    end: Position {
                        line: 6,
                        character: 8,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\n*This is account 1*  \naliased from `Act1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // account with multiple aliases but no note
            let source = textwrap::dedent(
                "
                account Account1
                    alias Act1
                    alias Acct1

                2024/01/02 Payee1
                    Act1   $1
                    Acct1  $1
                    Other
                ",
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 6,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 4,
                    },
                    end: Position {
                        line: 6,
                        character: 8,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\naliased from `Act1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 7,
                    character: 5,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 7,
                        character: 4,
                    },
                    end: Position {
                        line: 7,
                        character: 9,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Account1`\n***\naliased from `Acct1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }
    }

    #[test]
    fn test_hover_commodities() {
        init_logging();

        {
            // commodity without alias or note
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    commodity $

                    2024/01/02 Payee1
                        Account1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 4,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 14,
                    },
                    end: Position {
                        line: 4,
                        character: 15,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with note but no alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    commodity $
                        note This is USD

                    2024/01/02 Payee1
                        Account1  $1
                        Other
                    ",
                ),
                &Position {
                    line: 5,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 5,
                        character: 14,
                    },
                    end: Position {
                        line: 5,
                        character: 15,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\n*This is USD*",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    commodity $
                        alias USD

                    2024/01/02 Payee1
                        Account1  USD1
                        Other
                    ",
                ),
                &Position {
                    line: 5,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 5,
                        character: 14,
                    },
                    end: Position {
                        line: 5,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\naliased from `USD`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with note before alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    commodity $
                        note US dollars
                        alias USD

                    2024/01/02 Payee1
                        Account1  USD1
                        Other
                    ",
                ),
                &Position {
                    line: 6,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 14,
                    },
                    end: Position {
                        line: 6,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\n*US dollars*  \naliased from `USD`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with note after alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    commodity $
                        alias USD
                        note US dollars

                    2024/01/02 Payee1
                        Account1  USD1
                        Other
                    ",
                ),
                &Position {
                    line: 6,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 14,
                    },
                    end: Position {
                        line: 6,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\n*US dollars*  \naliased from `USD`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with multiple aliases but no note
            let source = textwrap::dedent(
                "
                commodity $
                    alias USD
                    alias Dollars

                2024/01/02 Payee1
                    Account1  USD1
                    Account1  Dollars 1
                    Other
                ",
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 6,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 6,
                        character: 14,
                    },
                    end: Position {
                        line: 6,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\naliased from `USD`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 7,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 7,
                        character: 14,
                    },
                    end: Position {
                        line: 7,
                        character: 21,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\naliased from `Dollars`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // commodity with multiple aliases and a note
            let source = textwrap::dedent(
                "
                commodity $
                    alias USD
                    note This is a note
                    alias Dollars

                2024/01/02 Payee1
                    Account1  USD1
                    Account1  Dollars 1
                    Other
                ",
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 7,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 7,
                        character: 14,
                    },
                    end: Position {
                        line: 7,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\n*This is a note*  \naliased from `USD`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 8,
                    character: 14,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 8,
                        character: 14,
                    },
                    end: Position {
                        line: 8,
                        character: 21,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`$`\n***\n*This is a note*  \naliased from `Dollars`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }
    }

    #[test]
    fn test_hover_payees() {
        init_logging();

        {
            // payee without alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    payee Payee

                    2024/01/02 Payee
                        Account  $1
                        Account
                    ",
                ),
                &Position {
                    line: 3,
                    character: 12,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 3,
                        character: 11,
                    },
                    end: Position {
                        line: 3,
                        character: 16,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Payee`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // payee with alias
            let hovers = get_hovers(
                &textwrap::dedent(
                    "
                    payee Payee
                        alias Payee1

                    2024/01/02 Payee1
                        Account  $1
                        Account
                    ",
                ),
                &Position {
                    line: 4,
                    character: 12,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 4,
                        character: 11,
                    },
                    end: Position {
                        line: 4,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Payee`\n***\naliased from `Payee1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }

        {
            // payee with multiple aliases
            let source = textwrap::dedent(
                "
                payee Payee
                    alias Payee1
                    alias Payee2

                2024/01/02 Payee1
                    Account  $1
                    Account

                2024/01/02 Payee2
                    Account  $1
                    Account
                ",
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 5,
                    character: 12,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 5,
                        character: 11,
                    },
                    end: Position {
                        line: 5,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Payee`\n***\naliased from `Payee1`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );

            let hovers = get_hovers(
                &source,
                &Position {
                    line: 9,
                    character: 12,
                },
                None,
            );

            insta::assert_debug_snapshot!(hovers,
            @r###"
            (
                Range {
                    start: Position {
                        line: 9,
                        character: 11,
                    },
                    end: Position {
                        line: 9,
                        character: 17,
                    },
                },
                [
                    LedgerHover(
                        Hover {
                            contents: Scalar(
                                String(
                                    "`Payee`\n***\naliased from `Payee2`",
                                ),
                            ),
                            range: None,
                        },
                    ),
                ],
            )
            "###
            );
        }
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
            2024/01/02 Payee1
                Account

            2024/02/03 ! Payee2
                Account

            2024/02/03 * Mom & Dad
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
                    character: 10,
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
                        character: 10,
                    },
                    end: Position {
                        line: 4,
                        character: 12,
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
                        character: 10,
                    },
                    end: Position {
                        line: 7,
                        character: 12,
                    },
                },
            ),
        )
        "#
        );

        Ok(())
    }

    #[test]
    fn test_transaction_status_bug_maybe_invalid_xact() -> Result<()> {
        let source = vec![
            textwrap::dedent(
                "
                2024/01/02 Payee1
                    Account",
            ),
            // a line w/ 4 spaces, like we just hit <enter> to add another account
            "    ".to_string(),
            // actual blank line between above and below xacts
            textwrap::dedent(
                "
                2024/01/03 Payee2
                    Account2
                ",
            ),
        ]
        .join("\n");
        let mut backend = LedgerBackend::new();
        backend._test_project_files = Some(vec![]);
        backend.parse_document(&source);

        let status = backend.transaction_at_position_status(
            &source,
            // cursor at end of "    " line
            &Position {
                line: 3,
                character: 4,
            },
        )?;

        insta::assert_debug_snapshot!(status,
        @r#"
        Some(
            NotCleared(
                Position {
                    line: 1,
                    character: 10,
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
            2024/01/02 ! Payee1
                Account

            2024/02/03 Payee2
                Account

            2024/02/03 ! Mom & Dad
                Account

            2024/02/03 Payee1
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
                    character: 11,
                },
                end: Position {
                    line: 1,
                    character: 12,
                },
            },
            Range {
                start: Position {
                    line: 7,
                    character: 11,
                },
                end: Position {
                    line: 7,
                    character: 12,
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
    fn get_diagnostics(
        source: &str,
        params: DiagnosticsParams,
        backend: Option<LedgerBackend>,
    ) -> Vec<Diagnostic> {
        let mut backend = backend.unwrap_or_else(|| {
            let mut be = LedgerBackend::new();
            be._test_project_files = Some(vec![]);
            be.parse_document(&source);
            be
        });

        let mut diagnostics = backend.diagnostics(Path::new("unused in test"), &source, &params);
        diagnostics.sort_by(|a, b| a.range.start.cmp(&b.range.start));
        diagnostics
    }

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
        match backend.completions_for_position(
            Path::new("unused in test"),
            &source,
            &position,
            &mut visited,
        ) {
            Ok(LocationBasedResult::Some {
                range,
                results: mut completions,
            }) => {
                completions.sort();
                (range, completions)
            }
            _ => panic!(),
        }
    }

    fn get_references(
        source: &str,
        position: &Position,
        include_declaration: bool,
        backend: Option<LedgerBackend>,
    ) -> (LspRange, Vec<LedgerLocation>) {
        let mut backend = backend.unwrap_or_else(|| {
            let mut be = LedgerBackend::new();
            be._test_project_files = Some(vec![]);
            be.parse_document(&source);
            be
        });

        let mut visited = HashSet::new();
        match backend.references_for_position(
            Path::new("unused in test"),
            &source,
            &position,
            include_declaration,
            &mut visited,
        ) {
            Ok(LocationBasedResult::Some { range, mut results }) => {
                results.sort_by_key(|r| r.range.0.start.line);
                (range, results)
            }
            _ => panic!(),
        }
    }

    fn get_hovers(
        source: &str,
        position: &Position,
        backend: Option<LedgerBackend>,
    ) -> (LspRange, Vec<LedgerHover>) {
        let mut backend = backend.unwrap_or_else(|| {
            let mut be = LedgerBackend::new();
            be._test_project_files = Some(vec![]);
            be.parse_document(&source);
            be
        });

        let mut visited = HashSet::new();
        match backend.hovers_for_position(
            Path::new("unused in test"),
            &source,
            &position,
            &mut visited,
        ) {
            Ok(LocationBasedResult::Some { range, results }) => (range, results),
            Ok(LocationBasedResult::NoNode(s)) => panic!("no node: {s}"),
            Ok(LocationBasedResult::None) => panic!("no results at position {position:?}"),
            Err(err) => panic!("error: {err}"),
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
