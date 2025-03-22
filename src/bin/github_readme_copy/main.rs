//! github_readme_copy/src/bin/github_readme_copy/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

use github_readme_copy::{GREEN, RED, RESET, YELLOW};

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();
    github_api_config_initialize();
    // super simple argument parsing. There are crates that can parse more complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("download") => {
            // get the authentication over OAuth2
            let secret_token = github_readme_copy::get_github_secret_token().unwrap();
            download_readme(&secret_token);
        }
        Some("upload") => match std::env::args().nth(2).as_deref() {
            // second argument
            // TODO: check if ssh-agent work and has some ssh keys
            Some(upload_url) => upload_github_readme_and_substack_articles(upload_url),
            None => println!("{RED}Error: Missing arguments `upload_url`.{RESET}"),
        },
        Some("substack") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(substack_url) => substack_download(substack_url),
            None => println!("{RED}Error: Missing arguments `substack_url`.{RESET}"),
        },
        Some("github_backup_bash_scripts") => {
            // get the authentication over OAuth2
            let secret_token = github_readme_copy::get_github_secret_token().unwrap();
            github_backup_bash_scripts(&secret_token);
        }
        _ => println!("{RED}Error: Unrecognized arguments. Try `github_readme_copy --help`{RESET}"),
    }
}

fn github_api_config_initialize() {
    use github_readme_copy::GITHUB_API_CONFIG;
    if GITHUB_API_CONFIG.get().is_some() {
        return;
    }

    let github_api_config_json = std::fs::read_to_string("github_api_config.json").unwrap();
    let github_api_config: github_readme_copy::GithubApiConfig = serde_json::from_str(&github_api_config_json).unwrap();
    let _ = GITHUB_API_CONFIG.set(github_api_config);
}

/// print help
fn print_help() {
    println!(
        r#"      
    {YELLOW}Welcome to github_readme_copy
    This program will download all your public README.md from GitHub in html format
    and upload these html files to your web server.
    This is useful, because SEO works really bad on GitHub READMEs.{RESET}
    {YELLOW}Before upload over SSH, use ssh-agent and ssh-add 
    to add the passphrase for the SSH connection to the web server.{RESET}

{GREEN}github_readme_copy --help{RESET}
{GREEN}github_readme_copy download{RESET}
{GREEN}github_readme_copy upload username@server:folder/{RESET}
{GREEN}github_readme_copy substack substack_url{RESET}
{GREEN}github_readme_copy github_backup_bash_scripts{RESET}

    {YELLOW}© 2022 bestia.dev  MIT License github.com/bestia-dev/github_readme_copy{RESET}
"#
    );
}

/// download from GitHub using your personal GitHub token inside the env variable
fn download_readme(secret_token: &secrecy::SecretString) {
    github_readme_copy::download_readme(secret_token);
}

/// upload over SSH
fn upload_github_readme_and_substack_articles(upload_url: &str) {
    github_readme_copy::upload_github_readme(upload_url);
    github_readme_copy::upload_substack_articles(upload_url);
}

/// download from substack
fn substack_download(substack_url: &str) {
    github_readme_copy::substack_download(substack_url);
}

/// create bash scripts for GitHub backup using your personal GitHub token inside the env variable
fn github_backup_bash_scripts(secret_token: &secrecy::SecretString) {
    github_readme_copy::github_backup_bash_scripts(secret_token);
}
