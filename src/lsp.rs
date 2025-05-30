use crate::backend::{CompletionResult, LedgerBackend, LedgerCompletion, TransactionStatus};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tower_lsp::{LspService, Server};

pub async fn run_server() {
    _run_server(tokio::io::stdin(), tokio::io::stdout()).await;
}

async fn _run_server<I, O>(read: I, write: O)
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    let (service, socket) = LspService::new(|client| Lsp {
        client,
        state: Mutex::new(LspState {
            backend: LedgerBackend::new(),
            config: Config::default(),
            sources: HashMap::new(),
        }),
    });
    Server::new(read, write, socket).serve(service).await;
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
            "[initialize] initializing {} {} ({} @ {})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
            env!("GIT_HASH"),
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
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
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

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        log_debug!(self, "[code_action] {params:?}");
        let start_time = std::time::Instant::now();

        let mut state = self.state.lock().await;
        log_debug!(
            self,
            "[code_action] acquired lock @ {:?}",
            start_time.elapsed()
        );
        let pathname = params.text_document.uri.path();
        let contents = match state.sources.get(pathname) {
            Some(contents) => contents.clone(),
            None => return Ok(None),
        };

        let mut actions = Vec::new();

        match state
            .backend
            .transaction_at_position_status(&contents, &params.range.start)
        {
            Ok(Some(status)) => {
                let make_pending_edit = |range| {
                    (
                        "pending (!)",
                        TextEdit {
                            range,
                            new_text: " !".to_string(),
                        },
                    )
                };
                let make_cleared_edit = |range| {
                    (
                        "cleared (*)",
                        TextEdit {
                            range,
                            new_text: " *".to_string(),
                        },
                    )
                };
                let make_not_cleared_edit = |range| {
                    (
                        "not cleared",
                        TextEdit {
                            range,
                            new_text: "".to_string(),
                        },
                    )
                };
                let make_code_action = |(label, edit)| {
                    let mut changes = HashMap::new();
                    changes.insert(params.text_document.uri.clone(), vec![edit]);

                    CodeActionOrCommand::CodeAction(CodeAction {
                        title: format!("Mark transaction as {label}"),
                        kind: None,
                        diagnostics: None,
                        edit: Some(WorkspaceEdit {
                            changes: Some(changes),
                            document_changes: None,
                            change_annotations: None,
                        }),
                        command: None,
                        is_preferred: Some(false),
                        disabled: None,
                        data: None,
                    })
                };

                match status {
                    TransactionStatus::NotCleared(pos) => {
                        let range = Range {
                            start: pos,
                            end: pos,
                        };

                        actions.push(make_code_action(make_pending_edit(range)));
                        actions.push(make_code_action(make_cleared_edit(range)));
                    }
                    TransactionStatus::Pending(range) => {
                        actions.push(make_code_action(make_cleared_edit(range)));
                        actions.push(make_code_action(make_not_cleared_edit(range)));
                    }
                    TransactionStatus::Cleared(range) => {
                        actions.push(make_code_action(make_pending_edit(range)));
                        actions.push(make_code_action(make_not_cleared_edit(range)));
                    }
                };
            }
            Ok(None) => {}
            Err(err) => {
                log!(self, ERROR, "[code_action] {err}");
                return Ok(None);
            }
        };

        match state.backend.pending_transaction_status_ranges(&contents) {
            Ok(pending_ranges) if !pending_ranges.is_empty() => {
                let pending_edits = pending_ranges
                    .into_iter()
                    .map(|range| TextEdit {
                        range,
                        new_text: "*".to_string(),
                    })
                    .collect();
                let mut changes = HashMap::new();
                changes.insert(params.text_document.uri.clone(), pending_edits);

                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                    title: "Mark all pending transactions cleared".to_string(),
                    kind: None,
                    diagnostics: None,
                    edit: Some(WorkspaceEdit {
                        changes: Some(changes),
                        document_changes: None,
                        change_annotations: None,
                    }),
                    command: None,
                    is_preferred: Some(false),
                    disabled: None,
                    data: None,
                }))
            }
            Ok(_) => {}
            Err(err) => {
                log!(self, ERROR, "[code_action] {err}");
                return Ok(None);
            }
        };

        log!(self, "[code_action:response] @ {:?}", start_time.elapsed());
        Ok(Some(actions))
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

#[cfg(test)]
mod test {
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
    use tower_lsp::{jsonrpc, lsp_types};

    use super::*;

    #[test_log::test(tokio::test)]
    async fn initialize() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;

        let request = jsonrpc::Request::build("initialize")
            .id(1)
            .params(serde_json::json!({"capabilities":{}}))
            .finish();

        let response = context
            .request::<lsp_types::InitializeResult>(&request)
            .await?;

        insta::assert_debug_snapshot!(response.capabilities.code_action_provider,
            @r#"
            Some(
                Simple(
                    true,
                ),
            )
            "#
        );

        insta::assert_debug_snapshot!(response.capabilities.completion_provider,
            @r#"
            Some(
                CompletionOptions {
                    resolve_provider: Some(
                        false,
                    ),
                    trigger_characters: Some(
                        [
                            "@",
                        ],
                    ),
                    all_commit_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                    completion_item: None,
                },
            )
            "#
        );

        insta::assert_debug_snapshot!(response.capabilities.document_formatting_provider,
            @r#"
            Some(
                Left(
                    true,
                ),
            )
            "#
        );

        insta::assert_debug_snapshot!(response.capabilities.definition_provider,
            @r#"
            Some(
                Left(
                    true,
                ),
            )
            "#
        );

        insta::assert_debug_snapshot!(response.capabilities.text_document_sync,
            @r#"
            Some(
                Kind(
                    Full,
                ),
            )
            "#
        );

        insta::assert_debug_snapshot!(response.capabilities.workspace,
            @r#"
            Some(
                WorkspaceServerCapabilities {
                    workspace_folders: Some(
                        WorkspaceFoldersServerCapabilities {
                            supported: Some(
                                true,
                            ),
                            change_notifications: Some(
                                Left(
                                    true,
                                ),
                            ),
                        },
                    ),
                    file_operations: None,
                },
            )
            "#
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn initialize_with_initialization_options() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;

        let request = jsonrpc::Request::build("initialize")
            .id(1)
            .params(serde_json::json!({
                "capabilities":{},
                "initializationOptions":{ "formatting": false }
            }))
            .finish();

        let response = context
            .request::<lsp_types::InitializeResult>(&request)
            .await?;

        insta::assert_debug_snapshot!(response.capabilities.document_formatting_provider, @r"None");

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn code_actions_with_pending_xacts() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

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
        context.prep_document(&source).await?;

        let actions = context.code_action(1, 10).await?.unwrap();

        let actions = actions
            .iter()
            .map(|a: &CodeActionOrCommand| match a {
                CodeActionOrCommand::Command(_) => todo!(),
                CodeActionOrCommand::CodeAction(action) => (
                    &action.title,
                    action
                        .edit
                        .as_ref()
                        .unwrap()
                        .changes
                        .as_ref()
                        .unwrap()
                        .values()
                        .collect::<Vec<_>>(),
                ),
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(actions,
            @r#"
            [
                (
                    "Mark transaction as pending (!)",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                },
                                new_text: " !",
                            },
                        ],
                    ],
                ),
                (
                    "Mark transaction as cleared (*)",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                },
                                new_text: " *",
                            },
                        ],
                    ],
                ),
                (
                    "Mark all pending transactions cleared",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 4,
                                        character: 9,
                                    },
                                    end: Position {
                                        line: 4,
                                        character: 10,
                                    },
                                },
                                new_text: "*",
                            },
                        ],
                    ],
                ),
            ]
            "#
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn code_actions_without_pending_xacts() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = textwrap::dedent(
            "
            24/01/02 Payee1
                Account

            24/02/03 * Payee2
                Account
            ",
        );
        context.prep_document(&source).await?;

        let actions = context.code_action(5, 10).await?.unwrap();

        let actions = actions
            .iter()
            .map(|a: &CodeActionOrCommand| match a {
                CodeActionOrCommand::Command(_) => todo!(),
                CodeActionOrCommand::CodeAction(action) => (
                    &action.title,
                    action
                        .edit
                        .as_ref()
                        .unwrap()
                        .changes
                        .as_ref()
                        .unwrap()
                        .values()
                        .collect::<Vec<_>>(),
                ),
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(actions,
            @r#"
            [
                (
                    "Mark transaction as pending (!)",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 4,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 4,
                                        character: 10,
                                    },
                                },
                                new_text: " !",
                            },
                        ],
                    ],
                ),
                (
                    "Mark transaction as not cleared",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 4,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 4,
                                        character: 10,
                                    },
                                },
                                new_text: "",
                            },
                        ],
                    ],
                ),
            ]
            "#
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn code_actions_with_effective_date() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = textwrap::dedent(
            "
            2024/01/02=2025/03/04 Payee1
                Account
                Account
            ",
        );
        context.prep_document(&source).await?;

        let actions = context.code_action(2, 10).await?.unwrap();

        let actions = actions
            .iter()
            .map(|a: &CodeActionOrCommand| match a {
                CodeActionOrCommand::Command(_) => todo!(),
                CodeActionOrCommand::CodeAction(action) => (
                    &action.title,
                    action
                        .edit
                        .as_ref()
                        .unwrap()
                        .changes
                        .as_ref()
                        .unwrap()
                        .values()
                        .collect::<Vec<_>>(),
                ),
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(actions,
            @r#"
                [
                    (
                        "Mark transaction as pending (!)",
                        [
                            [
                                TextEdit {
                                    range: Range {
                                        start: Position {
                                            line: 1,
                                            character: 21,
                                        },
                                        end: Position {
                                            line: 1,
                                            character: 21,
                                        },
                                    },
                                    new_text: " !",
                                },
                            ],
                        ],
                    ),
                    (
                        "Mark transaction as cleared (*)",
                        [
                            [
                                TextEdit {
                                    range: Range {
                                        start: Position {
                                            line: 1,
                                            character: 21,
                                        },
                                        end: Position {
                                            line: 1,
                                            character: 21,
                                        },
                                    },
                                    new_text: " *",
                                },
                            ],
                        ],
                    ),
                ]
                "#
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn code_actions_bug_with_maybe_invalid_xacts() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = vec![
            textwrap::dedent(
                "
                24/01/02 Payee1
                    Account",
            ),
            // a line w/ 4 spaces, like we just hit <enter> to add another account
            "    ".to_string(),
            // actual blank line between above and below xacts
            textwrap::dedent(
                "
                24/01/03 Payee2
                    Account2
                ",
            ),
        ]
        .join("\n");
        context.prep_document(&source).await?;
        // ask for actions at pos 3:4 (end of new "empty" account line)
        let actions = context.code_action(3, 4).await?.unwrap();

        let actions = actions
            .iter()
            .map(|a: &CodeActionOrCommand| match a {
                CodeActionOrCommand::Command(_) => todo!(),
                CodeActionOrCommand::CodeAction(action) => (
                    &action.title,
                    action
                        .edit
                        .as_ref()
                        .unwrap()
                        .changes
                        .as_ref()
                        .unwrap()
                        .values()
                        .collect::<Vec<_>>(),
                ),
            })
            .collect::<Vec<_>>();
        insta::assert_debug_snapshot!(actions,
            @r#"
            [
                (
                    "Mark transaction as pending (!)",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                },
                                new_text: " !",
                            },
                        ],
                    ],
                ),
                (
                    "Mark transaction as cleared (*)",
                    [
                        [
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                    end: Position {
                                        line: 1,
                                        character: 8,
                                    },
                                },
                                new_text: " *",
                            },
                        ],
                    ],
                ),
            ]
            "#
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn completions() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = textwrap::dedent(
            "
            24/01/02 Pay
                Account1

            24/02/03 * Payee2
                Acc
            ",
        );
        context.prep_document(&source).await?;

        {
            let completions = match context.completion(1, 12).await?.unwrap() {
                CompletionResponse::Array(completions) => completions,
                CompletionResponse::List(_) => unreachable!(),
            };

            let completions = completions
                .iter()
                .map(|item: &CompletionItem| {
                    (&item.label, &item.detail, item.text_edit.as_ref().unwrap())
                })
                .collect::<Vec<_>>();
            insta::assert_debug_snapshot!(completions,
                @r#"
                [
                    (
                        "Payee2",
                        Some(
                            "Payee",
                        ),
                        Edit(
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 1,
                                        character: 9,
                                    },
                                    end: Position {
                                        line: 1,
                                        character: 12,
                                    },
                                },
                                new_text: "Payee2",
                            },
                        ),
                    ),
                ]
                "#
            );
        }

        {
            let completions = match context.completion(5, 7).await?.unwrap() {
                CompletionResponse::Array(completions) => completions,
                CompletionResponse::List(_) => unreachable!(),
            };

            let completions = completions
                .iter()
                .map(|item: &CompletionItem| {
                    (&item.label, &item.detail, item.text_edit.as_ref().unwrap())
                })
                .collect::<Vec<_>>();
            insta::assert_debug_snapshot!(completions,
                @r#"
                [
                    (
                        "Account1",
                        Some(
                            "Account",
                        ),
                        Edit(
                            TextEdit {
                                range: Range {
                                    start: Position {
                                        line: 5,
                                        character: 4,
                                    },
                                    end: Position {
                                        line: 5,
                                        character: 7,
                                    },
                                },
                                new_text: "Account1",
                            },
                        ),
                    ),
                ]
                "#
            );
        }

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn completions_from_invalid_document_no_accounts() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = textwrap::dedent(
            "
            24/01/02 Pay
            ",
        );
        context.prep_document(&source).await?;

        {
            let completions = match context.completion(1, 12).await?.unwrap() {
                CompletionResponse::Array(completions) => completions,
                CompletionResponse::List(_) => unreachable!(),
            };

            let completions = completions
                .iter()
                .map(|item: &CompletionItem| {
                    (&item.label, &item.detail, item.text_edit.as_ref().unwrap())
                })
                .collect::<Vec<_>>();
            insta::assert_debug_snapshot!(completions,
                @r#"
                []
                "#
            );
        }

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn completions_from_invalid_document_no_amounts() -> anyhow::Result<()> {
        let mut context = TestContext::new().await?;
        context.initialize().await?;

        let source = textwrap::dedent(
            "
            24/01/02 Pay
                Account
                Account
            ",
        );
        context.prep_document(&source).await?;

        {
            let completions = match context.completion(1, 12).await?.unwrap() {
                CompletionResponse::Array(completions) => completions,
                CompletionResponse::List(_) => unreachable!(),
            };

            let completions = completions
                .iter()
                .map(|item: &CompletionItem| {
                    (&item.label, &item.detail, item.text_edit.as_ref().unwrap())
                })
                .collect::<Vec<_>>();
            insta::assert_debug_snapshot!(completions,
                @r#"
                []
                "#
            );
        }

        Ok(())
    }

    struct TestContext {
        pub request_tx: UnboundedSender<String>,
        pub response_rx: UnboundedReceiver<String>,
        pub _server: tokio::task::JoinHandle<()>,
        pub _client: tokio::task::JoinHandle<()>,
    }

    impl TestContext {
        pub async fn new() -> anyhow::Result<Self> {
            let (request_tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let (tx, mut client_response_rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let (client_tx, response_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

            let async_in = AsyncIn(rx);
            let async_out = AsyncOut(tx);

            let server = tokio::spawn(async move { _run_server(async_in, async_out).await });

            let client = tokio::spawn(async move {
                loop {
                    let Some(response) = client_response_rx.recv().await else {
                        continue;
                    };
                    if client_tx.send(response).is_err() {
                        tracing::error!("Failed to pass client response");
                    }
                }
            });

            Ok(Self {
                request_tx,
                response_rx,
                _server: server,
                _client: client,
            })
        }

        // pub async fn send_all(&mut self, messages: &[&str]) -> anyhow::Result<()> {
        //     for message in messages {
        //         self.send(&jsonrpc::Request::from_str(message)?).await?;
        //     }
        //     Ok(())
        // }

        pub async fn send(&mut self, request: &jsonrpc::Request) -> anyhow::Result<()> {
            self.request_tx
                .send(encode_message(None, &serde_json::to_string(request)?))?;
            Ok(())
        }

        pub async fn recv<R: std::fmt::Debug + serde::de::DeserializeOwned>(
            &mut self,
        ) -> anyhow::Result<R> {
            // TODO split response for single messages
            loop {
                let response = self
                    .response_rx
                    .recv()
                    .await
                    .ok_or_else(|| anyhow::anyhow!("empty response"))?;
                // decode response
                let payload = response.split('\n').last().unwrap_or_default();

                tracing::debug!("recv: {payload}");

                // skip log messages
                if payload.contains("window/logMessage") {
                    continue;
                }

                // try parsing as a notification
                let response = serde_json::from_str::<jsonrpc::Request>(payload)?;
                match response.into_parts() {
                    (_method, _id, Some(params)) => return Ok(serde_json::from_value(params)?),
                    _ => {
                        tracing::debug!("could not parse repsonse as notification");
                    }
                };

                // try parsing as a result response
                let response = serde_json::from_str::<jsonrpc::Response>(payload)?;
                let (_id, result) = response.into_parts();
                return Ok(serde_json::from_value(result?)?);
            }
        }

        pub async fn request<R: std::fmt::Debug + serde::de::DeserializeOwned>(
            &mut self,
            request: &jsonrpc::Request,
        ) -> anyhow::Result<R> {
            self.send(request).await?;
            self.recv().await
        }

        pub async fn initialize(&mut self) -> anyhow::Result<()> {
            let request = jsonrpc::Request::build("initialize")
                .id(1)
                .params(serde_json::json!({"capabilities":{}}))
                .finish();

            let _ = self
                .request::<lsp_types::InitializeResult>(&request)
                .await?;

            Ok(())
        }

        pub async fn prep_document(&mut self, content: &str) -> anyhow::Result<()> {
            let request = jsonrpc::Request::build("textDocument/didOpen")
                .params(serde_json::json!({"textDocument":{
                    "uri": "file:///foo.ledger",
                    "text": content,
                    "version": 0,
                    "languageId": "ledger"
                }}))
                .finish();

            let _ = self.send(&request).await?;

            Ok(())
        }

        pub async fn code_action(
            &mut self,
            line: u8,
            col: u8,
        ) -> anyhow::Result<Option<CodeActionResponse>> {
            let request = jsonrpc::Request::build("textDocument/codeAction")
                .id(3)
                .params(serde_json::json!({
                    "range":{
                        "start": { "line": line, "character": col },
                        "end":   { "line": line, "character": col }
                    },
                    "context": { "diagnostics": [] },
                    "textDocument":{
                        "uri": "file:///foo.ledger",
                        "text": "not used",
                        "version": 0,
                        "languageId": "ledger"
                    }
                }))
                .finish();

            self.request::<Option<CodeActionResponse>>(&request).await
        }

        pub async fn completion(
            &mut self,
            line: u8,
            col: u8,
        ) -> anyhow::Result<Option<CompletionResponse>> {
            let request = jsonrpc::Request::build("textDocument/completion")
                .id(3)
                .params(serde_json::json!({
                    "textDocument": {
                        "uri": "file:///foo.ledger",
                    },
                    "position": { "line": line, "character": col }
                }))
                .finish();

            self.request::<Option<CompletionResponse>>(&request).await
        }
    }

    fn encode_message(content_type: Option<&str>, message: &str) -> String {
        let content_type = content_type
            .map(|ty| format!("\r\nContent-Type: {ty}"))
            .unwrap_or_default();

        format!(
            "Content-Length: {}{}\r\n\r\n{}",
            message.len(),
            content_type,
            message
        )
    }

    pub struct AsyncIn(UnboundedReceiver<String>);
    pub struct AsyncOut(UnboundedSender<String>);

    impl AsyncRead for AsyncIn {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            let rx = self.get_mut();
            match rx.0.poll_recv(cx) {
                Poll::Ready(Some(v)) => {
                    // tracing::debug!("read value: {:?}", v);
                    buf.put_slice(v.as_bytes());
                    Poll::Ready(Ok(()))
                }
                _ => Poll::Pending,
            }
        }
    }

    impl AsyncWrite for AsyncOut {
        fn poll_write(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            let tx = self.get_mut();
            let value = String::from_utf8(buf.to_vec()).unwrap();
            // tracing::debug!("write value: {value:?}");
            let _ = tx.0.send(value);
            Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }

        fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }
}
