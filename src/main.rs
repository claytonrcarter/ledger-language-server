use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};

use anyhow::Result;

use crate::backend::LedgerBackend;

mod backend;
mod backend_format;
mod backend_nodes;
mod lsp;

#[tokio::main]
async fn main() {
    match (
        env::args().nth(1),
        env::args().nth(2),
        env::args().nth(3),
        env::args().nth(4),
    ) {
        (Some(arg), Some(file), Some(line), Some(column)) if arg == "--debug" => {
            #[allow(clippy::unwrap_used)]
            let (file, source) = {
                let file = Path::new(&file).canonicalize().unwrap();
                let file = file.as_os_str().to_str().unwrap().to_string();
                let source = contents_of_path(&file).unwrap();
                (file, source)
            };

            let mut be = LedgerBackend::new();
            be.parse_document(&source);

            let mut visited = HashSet::new();
            #[allow(clippy::unwrap_used)]
            let completions = be
                .completions_for_position(
                    &file,
                    &source,
                    &tower_lsp::lsp_types::Position {
                        line: line.parse().unwrap(),
                        character: column.parse().unwrap(),
                    },
                    &mut visited,
                )
                .unwrap();
            println!("completions:\n{completions:?}");

            return;
        }
        (Some(arg), _, _, _) if arg == "lsp" => lsp::run_server().await,
        (_, _, _, _) => {
            eprintln!("Usage: ledger-language-server lsp => run the LSP server using stdin/stdout");
        }
    }
}

/// path must be canonicalize-able; either canonical on it's own, or valid
/// relative to cwd
pub fn contents_of_path(path: &str) -> Result<String> {
    let p = Path::new(path)
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("[contents_of_path] {err}"))?;

    fs::read_to_string(p).map_err(|err| {
        anyhow::anyhow!("[contents_of_path] Unable to open file '{path}'\n[contents_of_path] {err}")
    })
}
