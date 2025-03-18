use crate::backend::{CompletionResult, LedgerBackend, LedgerCompletion};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::{LspService, Server};

pub async fn run_server() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|client| Lsp {
        client,
        state: Mutex::new(LspState {
            backend: LedgerBackend::new(),
            config: Config::default(),
            sources: HashMap::new(),
        }),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}

pub struct LspState {
    pub backend: LedgerBackend,

    pub config: Config,

    // see https://github.com/ebkalderon/nix-language-server/blob/master/src/backend.rs#L14-L23
    /// Mapping of path names to file contents.
    pub sources: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Config {
    pub format: bool,
    pub format_sort_transactions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            format: true,
            format_sort_transactions: true,
        }
    }
}

pub struct Lsp {
    pub client: Client,
    pub state: Mutex<LspState>,
}

macro_rules! log {
    // log!(self, LEVEL, "format {args} and {}", such)
    // where level is LOG, INFO, WARNING, ERROR
    ($self:ident, $lvl:ident, $($arg:tt)+) => ({
        $self.client
            .log_message(MessageType::$lvl, format!($($arg)+))
            .await;
    });

    // log!(self, "format {args} and {}", such)
    ($self:ident, $($arg:tt)+) => ({
        $self.client
            .log_message(MessageType::LOG, format!($($arg)+))
            .await;
    });
}

macro_rules! log_debug {
    // log!(self, LEVEL, "format {args} and {}", such)
    // where level is LOG, INFO, WARNING, ERROR
    ($self:ident, $lvl:ident, $($arg:tt)+) => ({
        #[cfg(debug_assertions)]
        $self.client
            .log_message(MessageType::$lvl, format!($($arg)+))
            .await;
    });

    // log!(self, "format {args} and {}", such)
    ($self:ident, $($arg:tt)+) => ({
        #[cfg(debug_assertions)]
        $self.client
            .log_message(MessageType::LOG, format!($($arg)+))
            .await;
    });
}

#[tower_lsp::async_trait]
impl LanguageServer for Lsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log!(
            self,
            "[initialize] initializing {} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        log_debug!(self, "[initialize] {params:#?}");
        log_debug!(
            self,
            "[initialize:initialization_options] {:#?}",
            params.initialization_options
        );

        let mut state = self.state.lock().await;
        if let Some(ref opts) = params.initialization_options {
            match opts.get("formatting") {
                Some(Value::Bool(should_format)) => {
                    state.config.format = *should_format;
                }
                Some(_) => {
                    log!(self, WARNING, "[initialize:config] unrecognized value for lsp setting 'formatting'. Expected one of `true` or `false`.");
                }
                None => {}
            }

            match opts.get("sort_transactions") {
                Some(Value::Bool(should_sort)) => {
                    state.config.format_sort_transactions = *should_sort;
                }
                Some(_) => {
                    log!(self, WARNING, "[initialize:config] unrecognized value for lsp setting 'sort_transactions'. Expected one of `true` or `false`.");
                }
                None => {}
            }
        }
        log_debug!(self, "[initialize:config] {:#?}", state.config);

        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    // trigger_characters: None,
                    trigger_characters: Some(vec!["@".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                document_formatting_provider: Some(OneOf::Left(true))
                    .filter(|_| state.config.format),
                definition_provider: Some(OneOf::Left(true)),
                execute_command_provider: None,
                // execute_command_provider: Some(ExecuteCommandOptions {
                //     commands: vec!["dummy.do_something".to_string()],
                //     work_done_progress_options: Default::default(),
                // }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),

                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        log_debug!(self, "[initialized] {_params:?}");
    }

    async fn shutdown(&self) -> Result<()> {
        log_debug!(self, "[shutdown]");
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _params: DidChangeWorkspaceFoldersParams) {
        log_debug!(self, "[did_change_workspace_folders] {_params:?}");
    }

    async fn did_change_configuration(&self, _params: DidChangeConfigurationParams) {
        log_debug!(self, "[did_change_configuration] {_params:?}");
    }

    async fn did_change_watched_files(&self, _params: DidChangeWatchedFilesParams) {
        log_debug!(self, "[did_change_watched_files] {_params:?}");
    }

    async fn execute_command(&self, _params: ExecuteCommandParams) -> Result<Option<Value>> {
        log_debug!(self, "[execute_command] {_params:?}");

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        {
            let mut p = params.clone();
            p.text_document.text = "...trimmed...".to_string();
            log_debug!(self, "[did_open] {p:?}");
        }

        // on open, cache the file contents, generate initial completions, and
        // run dianostics
        let mut state = self.state.lock().await;
        state.sources.insert(
            params.text_document.uri.path().to_owned(),
            params.text_document.text.clone(),
        );
        state.backend.parse_document(&params.text_document.text);

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
        {
            let mut p = params.clone();
            p.content_changes = p
                .content_changes
                .into_iter()
                .map(|mut c| {
                    c.text = "...trimmed...".to_string();
                    c
                })
                .collect();
            log_debug!(self, "[did_change] {p:?}");
        }

        // on update, only cache the file contents and don't touch the
        // completions or diagnostics (because the buffer may be
        // dirty/incomplete/incorrect)
        let mut state = self.state.lock().await;
        let content = match params.content_changes.first() {
            Some(content) => content.text.clone(),
            None => String::new(),
        };
        state
            .sources
            .insert(params.text_document.uri.path().to_owned(), content.clone());
        state.backend.parse_document(&content);
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        {
            let mut p = params.clone();
            p.text = Some("...trimmed...".to_string());
            log_debug!(self, "[did_save] {p:?}");
        }

        // on save, regenerate the completions and diagnostics, but don't cache
        // the file contents (params don't have access to updated buffer contents)
        // TODO figure out how to send TextDocumentSaveRegistrationOptions{include_text: Some(true)}
        // ... then we could update both
        let state = self.state.lock().await;
        if let Some(content) = state.sources.get(params.text_document.uri.path()).cloned() {
            self.client
                .publish_diagnostics(
                    params.text_document.uri.clone(),
                    LedgerBackend::diagnostics(params.text_document.uri.path(), &content),
                    None,
                )
                .await;
        }
    }

    async fn did_close(&self, _params: DidCloseTextDocumentParams) {
        log_debug!(self, "[did_close] {_params:?}");
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        log_debug!(self, "[completion] {params:?}");
        let start_time = std::time::Instant::now();

        // let contents = contents_of_path(params.text_document_position.text_document.uri.path());
        let mut state = self.state.lock().await;
        log_debug!(
            self,
            "[completion] acquired lock @ {:?}",
            start_time.elapsed()
        );
        let pathname = params.text_document_position.text_document.uri.path();
        let contents = match state.sources.get(pathname) {
            Some(contents) => contents.clone(),
            None => return Ok(None),
        };

        let mut visited = HashSet::new();
        let (range, completions) = match state.backend.completions_for_position(
            pathname,
            &contents,
            &params.text_document_position.position,
            &mut visited,
        ) {
            Ok(CompletionResult::Some { range, completions }) => (range, completions),
            Ok(CompletionResult::None) => {
                log_debug!(
                    self,
                    INFO,
                    "[completion] no completions for position {:?}",
                    &params.text_document_position.position
                );
                return Ok(None);
            }
            Ok(CompletionResult::NoNode(_report)) => {
                log_debug!(self, INFO, "[completion] {_report}");
                return Ok(None);
            }
            Err(err) => {
                log!(self, ERROR, "[completion] {err}");
                return Ok(None);
            }
        };

        let create_completion = |label: &str, detail: &str| {
            let mut completion = CompletionItem::new_simple(label.to_string(), detail.to_string());
            completion.text_edit = Some(CompletionTextEdit::Edit(TextEdit {
                range,
                new_text: label.to_string(),
            }));
            completion
        };

        let completions: Vec<CompletionItem> = completions
            .iter()
            .map(|i| match i {
                // FIXME there is no need to recompute the range of the current node
                // for each completion; it should be calculated once and reued
                LedgerCompletion::Account(account) => create_completion(account, "Account"),

                LedgerCompletion::Directive(directive) => create_completion(directive, "Directive"),

                LedgerCompletion::File(filename) => create_completion(filename, "File"),

                LedgerCompletion::Payee(payee) => create_completion(payee, "Payee"),

                LedgerCompletion::Period(period) => create_completion(period, "Period"),

                LedgerCompletion::PeriodSnippet(period) => {
                    let mut completion =
                        CompletionItem::new_simple(period.label.clone(), "Period".to_string());
                    completion.insert_text = Some(period.snippet.clone());
                    completion.insert_text_format = Some(InsertTextFormat::SNIPPET);
                    completion.text_edit = Some(CompletionTextEdit::Edit(TextEdit {
                        range,
                        new_text: period.snippet.clone(),
                    }));
                    completion
                }

                LedgerCompletion::Tag(tag) => create_completion(tag, "Tag"),
            })
            .collect();

        log!(
            self,
            "[completion:response] {} completions @ {:?}",
            completions.len(),
            start_time.elapsed()
        );
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        log_debug!(self, "[formatting] {params:?}");
        let _start_time = std::time::Instant::now();

        let state = self.state.lock().await;
        if !state.config.format {
            // we already told the client that we don't "support" formatting,
            // but perhaps they're asking anyway?
            log!(
                self,
                "[formatting:response] not formatting: disabled by user settings",
            );
            return Ok(None);
        }

        let source = match state.sources.get(params.text_document.uri.path()) {
            Some(source) => source,
            None => return Ok(None),
        };

        let formatted = match LedgerBackend::format(source, state.config.format_sort_transactions) {
            Ok(formatted) => formatted,
            Err(err) => {
                log!(self, ERROR, "{err}");
                source.clone()
            }
        };

        log_debug!(
            self,
            "[formatting:response] done in {:?}",
            _start_time.elapsed()
        );
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
        log_debug!(self, "[goto_definition] {params:?}");

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
            Some(("include", maybe_path)) => {
                let quotes: &[_] = &['"', '\''];
                maybe_path.trim().trim_matches(quotes)
            }
            None | Some((_, _)) => return Ok(None),
        };

        let path_start_offset = line.find(path).unwrap_or(0) as u32;
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
                        log!(
                            self,
                            ERROR,
                            "[goto_definition] Buffer has no parent dir? {buffer_path}"
                        );
                        return Ok(None);
                    }
                };
                let path = dir.join(path);
                match path.canonicalize() {
                    Ok(path) => path,
                    Err(err) => {
                        log!(
                            self,
                            ERROR,
                            "[goto_definition] Counld not canonicalize {}",
                            path.to_string_lossy()
                        );
                        log!(self, ERROR, "[goto_definition] {err}");
                        return Ok(None);
                    }
                }
            };

            match Url::from_file_path(&path) {
                Ok(url) => url,
                Err(()) => {
                    log!(
                        self,
                        ERROR,
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
