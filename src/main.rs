use ledger_parser::{Serializer, SerializerSettings};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{env, fs};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::Parser;
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    match (env::args().nth(1), env::args().nth(2)) {
        (Some(arg), Some(file)) if arg == "--debug" => {
            let source = contents_of_path(&file);
            let print_completions = true;

            let be = LedgerBackend::new();
            let mut visited = HashSet::new();
            dump_debug(
                "tree-sitter",
                be.completions(&file, &source, &mut visited),
                print_completions,
            );

            return;
        }
        (_, _) => {}
    }

    // simple_logging::log_to_file("ledger-lsp.log", log::LevelFilter::max())
    //     .expect("Could not init logging");
    log::info!("[main] ledger-lsp starting");

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|client| Lsp {
        client,
        state: Mutex::new(LspState {
            sources: HashMap::new(),
            completions: HashMap::new(),
        }),
        backend: LedgerBackend::new(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum LedgerCompletion {
    Account(String),
    Directive(String),
    File(String),
    Payee(String),
    Period(String),
    Tag(String),
}

#[derive(Debug)]
struct LedgerBackend {
    _test_included_content: Option<String>,
    _test_project_files: Option<Vec<String>>,
}

impl LedgerBackend {
    fn new() -> Self {
        Self {
            _test_included_content: None,
            _test_project_files: None,
        }
    }

    fn completions(
        &self,
        buffer_path: &str,
        content: &str,
        visited: &mut HashSet<String>,
    ) -> Vec<LedgerCompletion> {
        let mut completions = HashSet::new();

        let current_dir = match Path::new(buffer_path).parent() {
            Some(dir) => dir,
            None => {
                log::error!("[diagnostics] Buffer has no parent dir? {buffer_path}");
                // TODO ??
                return Vec::new();
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

                let included_content = self
                    ._test_included_content
                    .as_ref()
                    .map_or_else(|| contents_of_path(filename), |content| content.to_string());

                self.completions(filename, &included_content, visited)
                    .into_iter()
                    .for_each(|c| {
                        completions.insert(c);
                    })
            }
        }

        completions.into_iter().collect()
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

    fn diagnostics(buffer_path: &str, content: &str) -> Vec<Diagnostic> {
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
                    let dir = match Path::new(buffer_path).parent() {
                        Some(dir) => dir,
                        None => {
                            log::error!("[diagnostics] Buffer has no parent dir? {buffer_path}");
                            // TODO ??
                            return None;
                        }
                    };
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

    fn format(content: &str) -> String {
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
                String::from_utf8(buf).expect("TODO")
            }
            Err(err) => {
                log::error!("[format] Unable to parse ledger");
                log::error!("[format] {err}");
                content.to_owned()
            }
        }
    }
}

#[derive(Debug)]
struct LspState {
    // see https://github.com/ebkalderon/nix-language-server/blob/master/src/backend.rs#L14-L23
    sources: HashMap<String, String>,
    completions: HashMap<String, Vec<LedgerCompletion>>,
}

#[derive(Debug)]
struct Lsp {
    client: Client,
    state: Mutex<LspState>,
    backend: LedgerBackend,
}

#[tower_lsp::async_trait]
impl LanguageServer for Lsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!("[initialize] {params:?}");

        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    // trigger_characters: None,
                    trigger_characters: Some(vec!["@".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                execute_command_provider: None,
                // execute_command_provider: Some(ExecuteCommandOptions {
                //     commands: vec!["dummy.do_something".to_string()],
                //     work_done_progress_options: Default::default(),
                // }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        log::info!("[initialized] {params:?}");

        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        log::info!("[shutdown]");
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        log::info!("[did_change_workspace_folders]");
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        log::info!("[did_change_configuration]");
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        log::info!("[did_change_watched_files]");
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        log::info!("[execute_command]");
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        log::info!("[did_open] {params:?}");
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;

        // on open, cache the file contents, generate initial completions, and
        // run dianostics
        let mut state = self.state.lock().await;
        state.sources.insert(
            params.text_document.uri.path().to_owned(),
            params.text_document.text.clone(),
        );
        let mut visited = HashSet::new();

        state.completions.insert(
            params.text_document.uri.path().to_owned(),
            self.backend.completions(
                params.text_document.uri.path(),
                &params.text_document.text,
                &mut visited,
            ),
        );

        self.client
            .publish_diagnostics(
                params.text_document.uri.clone(),
                LedgerBackend::diagnostics(
                    params.text_document.uri.path(),
                    &params.text_document.text,
                ),
                None,
            )
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        log::info!("[did_change] {params:?}");
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;

        // on update, only cache the file contents and don't touch the
        // completions or diagnostics (because the buffer may be
        // dirty/incomplete/incorrect)
        let mut state = self.state.lock().await;
        state.sources.insert(
            params.text_document.uri.path().to_owned(),
            match params.content_changes.get(0) {
                Some(content) => content.text.clone(),
                None => String::new(),
            },
        );
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        log::info!("[did_save] {params:?}");
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;

        // on save, regenerate the completions and diagnostics, but don't cache
        // the file contents (params don't have access to updated buffer contents)
        // TODO figure out how to send TextDocumentSaveRegistrationOptions{include_text: Some(true)}
        // ... then we could update both
        let mut state = self.state.lock().await;
        if let Some(content) = state.sources.get(params.text_document.uri.path()).cloned() {
            let mut visited = HashSet::new();
            state.completions.insert(
                params.text_document.uri.path().to_owned(),
                self.backend
                    .completions(params.text_document.uri.path(), &content, &mut visited),
            );

            self.client
                .publish_diagnostics(
                    params.text_document.uri.clone(),
                    LedgerBackend::diagnostics(params.text_document.uri.path(), &content),
                    None,
                )
                .await;
        }
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        log::info!("[did_close]");

        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log::info!("[completion] {params:?}");

        // let contents = contents_of_path(params.text_document_position.text_document.uri.path());
        let state = self.state.lock().await;
        let contents = match state
            .sources
            .get(params.text_document_position.text_document.uri.path())
        {
            Some(contents) => contents,
            None => return Ok(None),
        };
        let completions = match state
            .completions
            .get(params.text_document_position.text_document.uri.path())
        {
            Some(contents) => contents,
            None => return Ok(None),
        };

        #[derive(Default)]
        struct CompletionsToInclude {
            accounts: bool,
            directives: bool,
            files: bool,
            payees: bool,
            periods: bool,
            tags: bool,
        }

        let include = {
            let line = params.text_document_position.position.line as usize;
            let line = contents.split('\n').nth(line).unwrap_or("");

            match (line.chars().nth(0), line.trim().chars().nth(0)) {
                // posting comment/note
                (Some(char1), Some(';' | '#' | '%' | '|' | '*')) if char1.is_whitespace() => {
                    CompletionsToInclude {
                        tags: true,
                        ..CompletionsToInclude::default()
                    }
                }

                // posting account or subdirective
                (Some(char1), _) if char1.is_whitespace() => CompletionsToInclude {
                    accounts: true,
                    directives: true,
                    ..CompletionsToInclude::default()
                },

                // transaction date
                (Some(char1), _) if char1.is_numeric() => CompletionsToInclude {
                    payees: true,
                    ..CompletionsToInclude::default()
                },

                // periodic transaction
                (Some(char1), _) if char1 == '~' => CompletionsToInclude {
                    periods: true,
                    ..CompletionsToInclude::default()
                },

                // include directive
                (Some('i'), _) if line.starts_with("include") => CompletionsToInclude {
                    files: true,
                    ..CompletionsToInclude::default()
                },

                (_, _) => CompletionsToInclude {
                    directives: true,
                    ..CompletionsToInclude::default()
                },
            }
        };

        let completions = completions
            .into_iter()
            .filter_map(|i| match i {
                LedgerCompletion::Account(account) if include.accounts => Some(
                    CompletionItem::new_simple(account.clone(), "Account".to_string()),
                ),
                LedgerCompletion::Directive(directive) if include.directives => Some(
                    CompletionItem::new_simple(directive.clone(), "Directive".to_string()),
                ),
                LedgerCompletion::File(filename) if include.files => Some(
                    CompletionItem::new_simple(filename.clone(), "File".to_string()),
                ),
                LedgerCompletion::Payee(payee) if include.payees => Some(
                    CompletionItem::new_simple(payee.clone(), "Payee".to_string()),
                ),
                LedgerCompletion::Period(period) if include.periods => Some(
                    CompletionItem::new_simple(period.clone(), "Period".to_string()),
                ),
                LedgerCompletion::Tag(tag) if include.tags => {
                    Some(CompletionItem::new_simple(tag.clone(), "Tag".to_string()))
                }
                LedgerCompletion::Account(_)
                | LedgerCompletion::Directive(_)
                | LedgerCompletion::File(_)
                | LedgerCompletion::Payee(_)
                | LedgerCompletion::Period(_)
                | LedgerCompletion::Tag(_) => None,
            })
            .collect();

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        log::info!("[formatting] {params:?}");

        let state = self.state.lock().await;
        let source = match state.sources.get(params.text_document.uri.path()) {
            Some(source) => source,
            None => return Ok(None),
        };

        let formatted = LedgerBackend::format(&source);

        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(u32::MAX, u32::MAX),
            },
            new_text: formatted,
        }]))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        log::info!("[goto_definition] {params:?}");

        let state = self.state.lock().await;
        let buffer_path = params
            .text_document_position_params
            .text_document
            .uri
            .path();
        let source = match state.sources.get(buffer_path) {
            Some(source) => source,
            None => return Ok(None),
        };

        let line_num = params.text_document_position_params.position.line as usize;
        let line = source.split('\n').nth(line_num).unwrap_or("");

        let path = match line.split_once(' ') {
            Some((maybe_include, maybe_path)) if maybe_include == "include" => {
                let quotes: &[_] = &['"', '\''];
                maybe_path.trim().trim_matches(quotes)
            }
            None | Some((_, _)) => return Ok(None),
        };

        let path_start_offset = line.find(path).unwrap() as u32;
        let path_len = path.len() as u32;
        let origin_selection_range = Some(Range {
            start: Position {
                line: params.text_document_position_params.position.line,
                character: path_start_offset,
            },
            end: Position {
                line: params.text_document_position_params.position.line,
                character: path_start_offset + path_len,
            },
        });

        let url = {
            let path = Path::new(path);
            let path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                let dir = match Path::new(buffer_path).parent() {
                    Some(dir) => dir,
                    None => {
                        log::error!("[goto_definition] Buffer has no parent dir? {buffer_path}");
                        return Ok(None);
                    }
                };
                let path = dir.join(path);
                match path.canonicalize() {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!(
                            "[goto_definition] Counld not canonicalize {}",
                            path.to_string_lossy()
                        );
                        log::error!("[goto_definition] {err}");
                        return Ok(None);
                    }
                }
            };

            match Url::from_file_path(&path) {
                Ok(url) => url,
                Err(()) => {
                    log::error!(
                        "[goto_definition] Unable to build url for {}",
                        path.to_string_lossy()
                    );
                    return Ok(None);
                }
            }
        };

        Ok(Some(GotoDefinitionResponse::Link(vec![LocationLink {
            origin_selection_range,
            target_uri: url,
            target_range: Range::default(),
            target_selection_range: Range::default(),
        }])))
    }
}

fn dump_debug(kind: &str, completions: Vec<LedgerCompletion>, print_completions: bool) {
    println!("[{kind}] {} total", completions.len());
    let mut accounts = Vec::new();
    let mut directives = Vec::new();
    let mut files = Vec::new();
    let mut payees = Vec::new();
    let mut periods = Vec::new();
    let mut tags = Vec::new();
    completions.iter().for_each(|c| match c {
        LedgerCompletion::Account(account) => accounts.push(account),
        LedgerCompletion::Directive(directive) => directives.push(directive),
        LedgerCompletion::File(filename) => files.push(filename),
        LedgerCompletion::Payee(payee) => payees.push(payee),
        LedgerCompletion::Period(period) => periods.push(period),
        LedgerCompletion::Tag(tag) => tags.push(tag),
    });
    println!("[{kind}] {} accounts", accounts.len());
    println!("[{kind}] {} directives", directives.len());
    println!("[{kind}] {} files", files.len());
    println!("[{kind}] {} payees", payees.len());
    println!("[{kind}] {} periods", periods.len());
    println!("[{kind}] {} tags", tags.len());

    if print_completions {
        dbg!(accounts);
        dbg!(directives);
        dbg!(files);
        dbg!(payees);
        dbg!(periods);
        dbg!(tags);
    }
}

/// path must be canonicalize-able; either canonical on it's own, or valid
/// relative to cwd
fn contents_of_path(path: &str) -> String {
    let p = Path::new(path);

    let p = match p.canonicalize() {
        Ok(p) => p,
        Err(err) => {
            log::error!("[completions_for_path] {err}");
            return String::new();
        }
    };

    match fs::read_to_string(p) {
        Ok(contents) => contents,
        Err(err) => {
            log::error!("[completions_for_path] Unable to open file '{path}'");
            log::error!("[completions_for_path] {err}");
            return String::new();
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
    let mut completions = be.completions("unused in test", &source, &mut visited);
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
    let mut completions = be.completions("unused in test", &source, &mut visited);
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
    let mut completions = be.completions("unused in test", "", &mut visited);
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

    let mut completions = be.completions("unused in test", &source, &mut visited);

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

    insta::assert_snapshot!(LedgerBackend::format(&source),
    @r"
    2023/09/28 (743) Check Withdrawal
        ; Memo: CHK#743
        SVFCU:Personal                          $-160.00
        SVFCU:Personal                           $-16.00
        Expenses:Uncategorized

    ",
    );
}
