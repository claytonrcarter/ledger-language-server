use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};

use crate::backend::{LedgerBackend, LedgerCompletion};

mod backend;
mod lsp;

#[tokio::main]
async fn main() {
    match (env::args().nth(1), env::args().nth(2)) {
        (Some(arg), Some(file)) if arg == "--debug" => {
            let source = contents_of_path(&file).unwrap();
            let print_completions = true;

            let be = LedgerBackend::new();
            let mut visited = HashSet::new();
            dump_debug(
                "tree-sitter",
                be.completions(&file, &source, &mut visited).unwrap(),
                print_completions,
            );

            return;
        }
        (_, _) => {}
    }

    lsp::run_server().await;
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
pub fn contents_of_path(path: &str) -> Result<String, String> {
    let p = Path::new(path)
        .canonicalize()
        .map_err(|err| format!("[contents_of_path] {err}"))?;

    fs::read_to_string(p).map_err(|err| {
        format!("[contents_of_path] Unable to open file '{path}'\n[contents_of_path] {err}")
    })
}
