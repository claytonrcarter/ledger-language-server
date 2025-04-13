// https://stackoverflow.com/a/44407625
use std::process::Command;
fn main() {
    let git_hash: String = match Command::new("git").args(["rev-parse", "HEAD"]).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.chars().take(8).collect(),
            Err(_) => "<unknown commit>".to_string(),
        },
        Err(_) => "<unknown commit>".to_string(),
    };
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
}
