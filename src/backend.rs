use ledger_parser::{Serializer, SerializerSettings};
use std::collections::HashSet;
use std::path::Path;
use tower_lsp::lsp_types::*;
use tree_sitter::Parser;
use walkdir::WalkDir;

use crate::contents_of_path;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LedgerCompletion {
    Account(String),
    Directive(String),
    File(String),
    Payee(String),
    Period(String),
    Tag(String),
}

#[derive(Debug)]
pub struct LedgerBackend {
    _test_included_content: Option<String>,
    _test_project_files: Option<Vec<String>>,
}

impl LedgerBackend {
    pub fn new() -> Self {
        Self {
            _test_included_content: None,
            _test_project_files: None,
        }
    }

    pub fn completions(
        &self,
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

        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_ledger::language())
            .expect("loading Ledger tree-sitter grammar");
        let tree = parser.parse(content, None).unwrap();

        let query = tree_sitter::Query::new(
            tree_sitter_ledger::language(),
            "(payee) @payee (account) @account (note) @note (filename) @filename",
        )
        .expect("creating tree-sitter query");

        let mut cursor = tree_sitter::QueryCursor::new();
        let source = content.as_bytes();
        for m in cursor.matches(&query, tree.root_node(), source) {
            // (payee) @payee
            for n in m.nodes_for_capture_index(0) {
                let payee = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                    .expect("converting bytes back to text")
                    .to_string();
                completions.insert(LedgerCompletion::Payee(payee));
            }

            // (account) @account
            for n in m.nodes_for_capture_index(1) {
                let account = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                    .expect("converting bytes back to text")
                    .to_string();
                completions.insert(LedgerCompletion::Account(account));
            }

            // (note) @note
            for n in m.nodes_for_capture_index(2) {
                let note = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                    .expect("converting bytes back to text");
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
                let filename = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                    .expect("converting bytes back to text");

                let path = Path::new(filename);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    current_dir.join(path)
                };
                let filename = path.as_os_str().to_str().unwrap_or(filename);

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
        match ledger_parser::parse(content) {
            Ok(ledger) => {
                let mut buf = Vec::new();
                let mut settings = SerializerSettings::default();
                settings.transaction_date_format = "%Y/%m/%d".to_owned();
                settings.indent_posting = Some(" ".repeat(4));
                settings.indent_amount = Some(" ".repeat(2));
                settings.indent_comment = Some(" ".to_owned());
                settings.align_postings = true;
                settings.posting_comments_sameline = true;
                settings.sort_transactions = true;
                settings.condense_empty_lines = true;
                settings.insert_empty_lines = true;
                ledger.write(&mut buf, &settings).expect("TODO");
                Ok(String::from_utf8(buf).expect("TODO"))
            }
            Err(err) => Err(format!("[format] Unable to parse ledger: {err}")),
        }
    }
}

#[test]
fn test_completions() {
    let be = LedgerBackend {
        _test_included_content: None,
        _test_project_files: Some(vec![]),
    };
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
    let be = LedgerBackend {
        _test_included_content: None,
        _test_project_files: Some(vec![]),
    };
    let source = textwrap::dedent(
        "
    24/01/02 Payee1
        ; Tag1: foo
        ; Tag2: bar
        Account1  $1
        Account2
    ",
    );
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
    let be = LedgerBackend {
        _test_included_content: None,
        _test_project_files: Some(
            vec!["foo.ledger", "bar.yaml", "baz/qux.ledger"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        ),
    };
    let mut visited = HashSet::new();
    let mut completions = be.completions("unused in test", "", &mut visited).unwrap();
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

    let be = LedgerBackend {
        _test_included_content: Some(included),
        _test_project_files: Some(vec![]),
    };
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
        2023/09/28 (743) Check Withdrawal   ; Memo: CHK#743
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
