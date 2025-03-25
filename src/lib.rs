// github_readme_copy/src/lib.rs

// You can collapse the long region below using VSCode. It is only the copy of the README.md file, because it gets compiled into docs.

#![doc=include_str!("../README.md")]

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`

// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// access to modules
mod encrypt_decrypt_with_ssh_key_mod;
mod github_mod;
mod substack_mod;
mod utils_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use encrypt_decrypt_with_ssh_key_mod::github_api_token_with_oauth2_mod::get_github_secret_token;
pub use encrypt_decrypt_with_ssh_key_mod::github_api_token_with_oauth2_mod::GithubApiConfig;
pub use encrypt_decrypt_with_ssh_key_mod::github_api_token_with_oauth2_mod::GITHUB_API_CONFIG;
pub use github_mod::download_readme;
pub use github_mod::github_backup_bash_scripts;
pub use github_mod::upload_github_readme;
pub use github_mod::upload_substack_articles;
pub use substack_mod::substack_download;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// All possible library errors for `thiserror`.
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("name `{0}` is already uppercase")]
    Uppercase(String),
    #[error("unknown error")]
    Unknown,
}

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
#[allow(dead_code)]
pub const BLUE: &str = "\x1b[34m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants
