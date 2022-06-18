//! github_readme_copy/src/bin/github_readme_copy/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

use github_readme_copy::{RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("download") => {
            // read from env variable
            match std::env::var("GITHUB_TOKEN") {
                Err(_err) => println!(
                        "{RED}Error: env variable GITHUB_TOKEN not found. 
Get your personal github token from https://github.com/settings/tokens.
Before run, store it in local session env variable (put a space before the command, to avoid the bash history):
 export GITHUB_TOKEN=*****{RESET}"
                    ),
                Ok(token) => download_readme(&token),
            }
        }
        Some("upload") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(upload_url) => upload_readme(upload_url),
            None => println!("{RED}Error: Missing arguments `upload_url`.{RESET}"),
        },
        _ => println!("{RED}Error: Unrecognized arguments. Try `github_readme_copy --help`{RESET}"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"      
{YELLOW}Welcome to github_readme_copy
This program will download all your public README.md from Github in html format
and upload these html files to your web server.
This is useful, because SEO works really bad on Github READMEs.{RESET}

github_readme_copy --help

{YELLOW}Before download, store in env variable your personal token: export GITHUB_TOKEN=*****
Get your personal github token from https://github.com/settings/tokens.{RESET}

github_readme_copy download

{YELLOW}Before upload over SSH, use ssh-agent and ssh-add 
to add the passphrase for the SSH connection to the web server.{RESET}

github_readme_copy upload username@server:folder/
"#
    );
}

/// download from github using your personal github token inside the env variable
fn download_readme(token: &str) {
    github_readme_copy::download_readme(token);
}

/// upload over SSH
fn upload_readme(upload_url: &str) {
    github_readme_copy::upload_readme(upload_url);
}
