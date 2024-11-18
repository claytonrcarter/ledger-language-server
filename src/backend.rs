use std::collections::{HashMap, HashSet};
use std::path::Path;
use tower_lsp::lsp_types::*;
use tree_sitter::{Language, Parser, Point, Tree};
use type_sitter::StreamingIterator;
use walkdir::WalkDir;

use crate::{backend_format, contents_of_path};

fn substring(source: &[u8], start_byte: usize, end_byte: usize) -> String {
    std::str::from_utf8(&source[start_byte..end_byte.min(source.len())])
        .expect("converting bytes back to text")
        .to_string()
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LedgerCompletion {
    Account(String),
    Directive(String),
    File(String),
    Payee(String),
    Period(String),
    Tag(String),
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

    fn parser(&self) -> Parser {
        let mut parser = Parser::new();
        let language = Language::new(tree_sitter_ledger::LANGUAGE);
        parser
            .set_language(&language)
            .expect("loading Ledger tree-sitter grammar");
        parser
    }

    /// Parse an input document (source code) and save the parsed Tree for use
    /// later. If the document has already been cached, no new parsing is done.
    pub fn parse_document<'a>(&mut self, content: &'a str) {
        if !self.trees_cache.contains_key(content) {
            self.trees_cache.insert(
                content.to_string(),
                self.parser().parse(content, None).unwrap(),
            );
        }
    }

    pub fn completions(
        &mut self,
        buffer_path: &str,
        content: &str,
        visited: &mut HashSet<String>,
    ) -> Result<Vec<LedgerCompletion>, String> {
        let mut completions = HashSet::new();

        let current_dir = match Path::new(buffer_path).parent() {
            Some(dir) => dir,
            None => {
                // TODO ??
                return Err(format!(
                    "[diagnostics] Buffer has no parent dir? {buffer_path}"
                ));
            }
        };

        // "is empty" is a proxy for "this is the first call" to completions,
        // not a recursive call to included files
        if visited.is_empty() {
            self.completions_insert_directives(&mut completions);
            self.completions_insert_periods(&mut completions);

            // only crawl files once, from the dir containing the buffer
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
        }

        let tree = match self.trees_cache.get(content) {
            Some(tree) => tree.clone(),
            None => {
                return Err(format!(
                    "no tree found for contents of file '{buffer_path}'"
                ))
            }
        };

        let query = tree_sitter::Query::new(
            &self.parser().language().unwrap(),
            "(payee) @payee (account) @account (note) @note (filename) @filename",
        )
        .expect("creating tree-sitter query");
        let mut cursor = tree_sitter::QueryCursor::new();

        let source = content.as_bytes();
        let mut matches = cursor.matches(&query, tree.root_node(), source);
        while let Some(m) = matches.next() {
            // (payee) @payee
            for n in m.nodes_for_capture_index(0) {
                let payee = substring(source, n.start_byte(), n.end_byte());
                completions.insert(LedgerCompletion::Payee(payee));
            }

            // (account) @account
            for n in m.nodes_for_capture_index(1) {
                let account = substring(source, n.start_byte(), n.end_byte());
                std::str::from_utf8(&source[n.start_byte()..n.end_byte().min(source.len())])
                    .expect("converting bytes back to text")
                    .to_string();
                completions.insert(LedgerCompletion::Account(account));
            }

            // (note) @note
            for n in m.nodes_for_capture_index(2) {
                let note = substring(source, n.start_byte(), n.end_byte());
                note.split('\n')
                    .filter_map(|l| {
                        match l
                            // https://ledger-cli.org/doc/ledger3.html#Commenting-on-your-Journal
                            .trim_start_matches([' ', '\t', ';', '#', '%', '|', '*'])
                            .split_once(": ")
                        {
                            Some((tag, _)) if !tag.contains(' ') => Some(tag.to_owned()),
                            Some(_) | None => None,
                        }
                    })
                    .for_each(|tag| {
                        completions.insert(LedgerCompletion::Tag(tag));
                    });
            }

            // (filename) @filename
            for n in m.nodes_for_capture_index(3) {
                let filename = substring(source, n.start_byte(), n.end_byte());

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

                let included_content = self._test_included_content.as_ref().map_or_else(
                    || contents_of_path(filename),
                    |content| Ok(content.to_string()),
                )?;

                self.completions(filename, &included_content, visited)
                    // ignore errors for included files; surface those via
                    // diagnostics instead
                    .unwrap_or_default()
                    .into_iter()
                    .for_each(|c| {
                        completions.insert(c);
                    })
            }
        }

        Ok(completions.into_iter().collect())
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
        // https://ledger-cli.org/doc/ledger3.html#Command-Directives
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
    }

    pub fn diagnostics(buffer_path: &str, content: &str) -> Vec<Diagnostic> {
        content
            .split('\n')
            .enumerate()
            .filter_map(|(i, line)| {
                let path = match line.trim().split_once(' ') {
                    Some((maybe_include, maybe_path)) if maybe_include == "include" => {
                        let quotes: &[_] = &['"', '\''];
                        maybe_path.trim().trim_matches(quotes)
                    }
                    None | Some((_, _)) => return None,
                };

                let path_start_offset = line.find(path).unwrap() as u32;
                let path_len = path.len() as u32;

                Some((
                    path,
                    Range {
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
                let path = Path::new(path);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    // FIXME what is parent() is None?
                    let dir = Path::new(buffer_path).parent()?;
                    dir.join(path)
                };

                if path.exists() {
                    None
                } else {
                    Some(Diagnostic::new_simple(range, "does not exist".to_owned()))
                }
            })
            .collect()
    }

    pub fn format(content: &str) -> Result<String, String> {
        backend_format::format(content).map_err(|_err| "TODO convert io::Error to ???".to_string())
    }

    pub fn node_range_at_position(
        &self,
        content: &str,
        kind: &LedgerCompletion,
        position: &Position,
    ) -> Option<Range> {
        let tree = self.trees_cache.get(content)?;

        let point = Point {
            row: position.line as usize,
            column: position.character as usize,
        };
        let mut cursor = tree.walk();

        // descend to smallest node @ point
        while cursor.goto_first_child_for_point(point).is_some() {}

        // seek to first named node; if the current (unnamed) node starts at
        // point, then the point/cursor could be at the "end" of the previous
        // node
        while !cursor.node().is_named() {
            if cursor.node().range().start_point == point {
                cursor.goto_previous_sibling();
            } else {
                cursor.goto_parent();
            }
        }

        let node_kind = cursor.node().kind();
        match kind {
            LedgerCompletion::Account(_) if node_kind == "account" => {}
            LedgerCompletion::Payee(_) if node_kind == "payee" => {}
            LedgerCompletion::Account(_)
            | LedgerCompletion::Payee(_)
            | LedgerCompletion::Directive(_)
            | LedgerCompletion::File(_)
            | LedgerCompletion::Period(_)
            | LedgerCompletion::Tag(_) => return None,
        }

        let range = cursor.node().range();
        Some(Range {
            start: Position {
                line: range.start_point.row as u32,
                character: range.start_point.column as u32,
            },
            end: Position {
                line: range.end_point.row as u32,
                character: range.end_point.column as u32,
            },
        })
    }
}

#[test]
fn test_completions() {
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

    let mut be = LedgerBackend::new();
    be._test_project_files = Some(vec![]);
    be.parse_document(&source);

    let mut visited = HashSet::new();
    let mut completions = be
        .completions("unused in test", &source, &mut visited)
        .unwrap();
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Account(
            "Account1",
        ),
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
        Payee(
            "Mom & Dad",
        ),
        Payee(
            "Payee1",
        ),
        Payee(
            "Payee2",
        ),
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
    ]
    "#
    );
}

#[test]
fn test_completions_tags() {
    let source = textwrap::dedent(
        "
    24/01/02 Payee1
        ; Tag1: foo
        ; Tag2: bar
        Account1  $1
        Account2
    ",
    );

    let mut be = LedgerBackend::new();
    be._test_project_files = Some(vec![]);
    be.parse_document(&source);

    let mut visited = HashSet::new();

    let mut completions = be
        .completions("unused in test", &source, &mut visited)
        .unwrap();
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Account(
            "Account1",
        ),
        Account(
            "Account2",
        ),
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
        Payee(
            "Payee1",
        ),
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
        Tag(
            "Tag1",
        ),
        Tag(
            "Tag2",
        ),
    ]
    "#
    );
}

#[test]
fn test_completions_files() {
    let source = "";

    let mut be = LedgerBackend::new();
    be._test_project_files = Some(
        vec!["foo.ledger", "bar.yaml", "baz/qux.ledger"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    );
    be.parse_document(source);

    let mut visited = HashSet::new();
    let mut completions = be
        .completions("unused in test", source, &mut visited)
        .unwrap();
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
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
        File(
            "baz/qux.ledger",
        ),
        File(
            "foo.ledger",
        ),
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
    ]
    "#
    );
}

#[test]
fn test_completions_from_included_files() {
    let source = "include foo.ledger";
    let included = textwrap::dedent(
        "
        24/01/02 Payee1
            Account1  $1
            Account2
        ",
    );

    let mut be = LedgerBackend::new();
    be._test_included_content = Some(included.clone());
    be._test_project_files = Some(vec![]);
    be.parse_document(source);
    be.parse_document(&included);

    let mut visited = HashSet::new();

    let mut completions = be
        .completions("unused in test", &source, &mut visited)
        .unwrap();

    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Account(
            "Account1",
        ),
        Account(
            "Account2",
        ),
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
        Payee(
            "Payee1",
        ),
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
    ]
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

    insta::assert_snapshot!(LedgerBackend::format(&source).unwrap(),
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
fn test_account_range() {
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
    let range = be.node_range_at_position(
        &source,
        &LedgerCompletion::Account(String::new()),
        &Position {
            line: 2,
            character: 7,
        },
    );
    insta::assert_debug_snapshot!(range,
    @r"
    Some(
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
    ",
    );

    // end of Qux
    let range = be.node_range_at_position(
        &source,
        &LedgerCompletion::Account(String::new()),
        &Position {
            line: 3,
            character: 7,
        },
    );
    insta::assert_debug_snapshot!(range,
    @r"
    Some(
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
    ",
    );

    // end of Wi
    let range = be.node_range_at_position(
        &source,
        &LedgerCompletion::Account(String::new()),
        &Position {
            line: 3,
            character: 14,
        },
    );
    insta::assert_debug_snapshot!(range,
    @r"
    Some(
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
    ",
    );

    // middle of Foo (payee, not account)
    let range = be.node_range_at_position(
        &source,
        &LedgerCompletion::Account(String::new()),
        &Position {
            line: 1,
            character: 12,
        },
    );
    insta::assert_debug_snapshot!(range,
    @r"
    None
    ",
    );

    // middle of Foo (payee)
    let range = be.node_range_at_position(
        &source,
        &LedgerCompletion::Payee(String::new()),
        &Position {
            line: 1,
            character: 12,
        },
    );
    insta::assert_debug_snapshot!(range,
    @r"
    Some(
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
    ",
    );
}
