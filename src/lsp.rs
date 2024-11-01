use crate::backend::{LedgerBackend, LedgerCompletion};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::{LspService, Server};

pub async fn run_server() {
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

#[derive(Debug)]
pub struct LspState {
    // see https://github.com/ebkalderon/nix-language-server/blob/master/src/backend.rs#L14-L23
    pub sources: HashMap<String, String>,
    pub completions: HashMap<String, Vec<LedgerCompletion>>,
}

#[derive(Debug)]
pub struct Lsp {
    pub client: Client,
    pub state: Mutex<LspState>,
    pub backend: LedgerBackend,
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
