use ledger_parser::{LedgerItem, Serializer, SerializerSettings};
use log::LevelFilter;
use regex::Regex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{env, fs};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::Parser;

#[tokio::main]
async fn main() {
    if let Some(arg) = env::args().nth(1) {
        if arg == "--debug" {
            let source = contents_of_path("/Users/crcarter/Documents/Bookkeeping/ledger/personal-archive/clayton-2023-09-05-2023-12-03.ledger");
            let print_completions = false;

            let be = LedgerBackend::Regex;
            dump_debug("regex", be.completions(&source), print_completions);

            let be = LedgerBackend::Parse;
            dump_debug("parse", be.completions(&source), print_completions);

            let be = LedgerBackend::TreeSitter;
            dump_debug("tree-sitter", be.completions(&source), print_completions);
        }

        return;
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
        backend: LedgerBackend::TreeSitter,
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum LedgerCompletion {
    Payee(String),
    Account(String),
}

#[derive(Debug)]
enum LedgerBackend {
    /// A backend that actually parses actual ledger parser to extract data from
    /// the document. This backend is the most correct, but the least reliable
    /// because it will fail if the document is not (more or less) syntactically
    /// correct.
    #[allow(dead_code)]
    Parse,

    /// A backend that extracts data from the document using regexs. This backend
    /// is the most prone to false positives due to imperfect regex patterns, but
    /// it is the most reliable because it operated line by line instead of on the
    /// document as a whole.
    Regex,

    /// A backend that uses tree-sitter to parse and query a document. tree-sitter
    /// is error tolerant and its queries are quite specific, so may represent the
    /// best of both worlds: correct extraction of symbols in specific positions
    /// *and* reliable parsing of incomplete/incorrect documents
    #[allow(dead_code)]
    TreeSitter,
}

impl LedgerBackend {
    fn completions(&self, content: &str) -> Vec<LedgerCompletion> {
        let mut completions = HashSet::new();

        match self {
            LedgerBackend::Parse => {
                let ledger = match ledger_parser::parse(content) {
                    Ok(ledger) => ledger,
                    Err(err) => {
                        log::error!("[completions_for_path] Unable to parse ledger");
                        log::error!("[completions_for_path] {err}");
                        return Vec::new();
                    }
                };

                for item in ledger.items {
                    let transaction = match item {
                        LedgerItem::Transaction(transaction) => transaction,
                        LedgerItem::EmptyLine
                        | LedgerItem::LineComment(_)
                        | LedgerItem::CommodityPrice(_)
                        | LedgerItem::Include(_)
                        | _ => continue,
                    };
                    if let Some(description) = transaction.description {
                        completions.insert(LedgerCompletion::Payee(description));
                    }

                    for posting in transaction.postings {
                        completions.insert(LedgerCompletion::Account(posting.account));
                    }
                }
            }
            LedgerBackend::Regex => {
                // date=date (code) payee ; comment
                let payee_re = "(?m)^\\d[^ ]+( \\(.+\\))? (.+)(;|$)";
                // TODO compile this once (lazy static?)
                let re = Regex::new(payee_re).expect("compile regex");
                for caps in re.captures_iter(content) {
                    completions.insert(LedgerCompletion::Payee(caps[2].trim().to_string()));
                }

                // <indent>account  ... or <indent>account\n
                // also handle (ie ignore) <indent><comment>
                let account_re = "(?m)^ +([^;\\n]+)";
                let re = Regex::new(account_re).expect("compile regex");
                for caps in re.captures_iter(content) {
                    let account = caps[1].trim().split("  ").next().unwrap();
                    if !account.is_empty() {
                        completions.insert(LedgerCompletion::Account(account.to_string()));
                    }
                }
            }
            LedgerBackend::TreeSitter => {
                let mut parser = Parser::new();
                parser
                    .set_language(tree_sitter_ledger::language())
                    .expect("loading Ledger tree-sitter grammar");
                let tree = parser.parse(content, None).unwrap();

                let query = tree_sitter::Query::new(
                    tree_sitter_ledger::language(),
                    "(payee) @payee (account) @account",
                )
                .expect("creating tree-sitter query");

                let mut cursor = tree_sitter::QueryCursor::new();
                let source = content.as_bytes();
                for m in cursor.matches(&query, tree.root_node(), source) {
                    for n in m.nodes_for_capture_index(0) {
                        let payee = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                            .expect("converting bytes back to text")
                            .to_string();
                        completions.insert(LedgerCompletion::Payee(payee));
                    }
                    for n in m.nodes_for_capture_index(1) {
                        let account = std::str::from_utf8(&source[n.start_byte()..n.end_byte()])
                            .expect("converting bytes back to text")
                            .to_string();
                        completions.insert(LedgerCompletion::Account(account));
                    }
                }
            }
        }

        completions.into_iter().collect()
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
        state.completions.insert(
            params.text_document.uri.path().to_owned(),
            self.backend.completions(&params.text_document.text),
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
            state.completions.insert(
                params.text_document.uri.path().to_owned(),
                self.backend.completions(&content),
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

        let (include_payees, include_accounts) = {
            let line = params.text_document_position.position.line as usize;
            let line = contents.split('\n').nth(line).unwrap_or("");

            match line.chars().nth(0) {
                Some(char) => (char.is_numeric(), char.is_whitespace()),
                None => (false, false),
            }
        };

        let completions = completions
            .into_iter()
            .filter_map(|i| match i {
                LedgerCompletion::Payee(payee) if include_payees => Some(
                    CompletionItem::new_simple(payee.clone(), "Payee".to_string()),
                ),
                LedgerCompletion::Account(account) if include_accounts => Some(
                    CompletionItem::new_simple(account.clone(), "Account".to_string()),
                ),
                LedgerCompletion::Payee(_) | LedgerCompletion::Account(_) => None,
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
    let (payees, accounts): (Vec<_>, Vec<_>) = completions.iter().partition(|c| match c {
        LedgerCompletion::Payee(_) => true,
        LedgerCompletion::Account(_) => false,
    });
    println!("[{kind}] {} payees", payees.len());
    println!("[{kind}] {} accounts", accounts.len());

    if print_completions {
        dbg!(payees);
        dbg!(accounts);
    }
}

fn contents_of_path(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            log::error!("[completions_for_path] Unable to open file");
            log::error!("[completions_for_path] {err}");
            return String::new();
        }
    }
}

#[test]
fn test_completions_by_regex() {
    let be = LedgerBackend::Regex;
    let source = textwrap::dedent(
        "
    24/01/02 Payee1
        Account1  $1
        Account2

    24/02/03 Payee2
        Account2  $2
        Account3
    ",
    );
    let mut completions = be.completions(&source);
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Payee(
            "Payee1",
        ),
        Payee(
            "Payee2",
        ),
        Account(
            "Account1",
        ),
        Account(
            "Account2",
        ),
        Account(
            "Account3",
        ),
    ]
    "#
    );
}

#[test]
fn test_completions_by_parse() {
    let be = LedgerBackend::Parse;
    let source = textwrap::dedent(
        "
    2024/01/02 Payee1
        Account1  $1
        Account2

    2024/02/03 Payee2
        Account2  $2
        Account3
    ",
    );
    let mut completions = be.completions(&source);
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Payee(
            "Payee1",
        ),
        Payee(
            "Payee2",
        ),
        Account(
            "Account1",
        ),
        Account(
            "Account2",
        ),
        Account(
            "Account3",
        ),
    ]
    "#
    );
}

#[test]
fn test_completions_by_treesitter() {
    let be = LedgerBackend::TreeSitter;
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
    let mut completions = be.completions(&source);
    completions.sort();
    insta::assert_debug_snapshot!(completions,
    @r#"
    [
        Payee(
            "Mom & Dad",
        ),
        Payee(
            "Payee1",
        ),
        Payee(
            "Payee2",
        ),
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
