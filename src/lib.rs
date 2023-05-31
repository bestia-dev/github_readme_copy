// github_readme_copy/src/lib.rs

// You can collapse the long region below using VSCode. It is only the copy of the README.md file, because it gets compiled into docs.

// region: auto_md_to_doc_comments include README.md A //!
//! # github_readme_copy
//!
//! **Copy my public README.md files from GitHub in HTML format**  
//! ***version: 1.0.136 date: 2023-05-31 author: [bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/github_readme_copy)***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-881-green.svg)](https://github.com/bestia-dev/github_readme_copy/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-95-blue.svg)](https://github.com/bestia-dev/github_readme_copy/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-89-purple.svg)](https://github.com/bestia-dev/github_readme_copy/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/github_readme_copy/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-50-orange.svg)](https://github.com/bestia-dev/github_readme_copy/)
//!
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/github_readme_copy/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/github_readme_copy/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/github_readme_copy/) ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/93552555.svg)
//!
//! Hashtags: #rustlang #tutorial  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Motivation
//!
//! It looks like google search SEO is really bad for GitHub README.md files.  
//! Maybe it will work better as HTML files on my own domain. Yes, it does.  
//! On every README HTML, there is a link to bestia.dev and a link to the GitHub repository.  
//! I need a utility CLI that copies the README files as they are rendered on GitHub and save them as html files in the directory `github_readme`.  
//! Then I will use an `rsync` command to upload the files to my google cloud virtual machine.
//!
//! ## Octocrab
//!
//! I will use the crate `octocrab` to get a list of my public repos.  
//! You need to have a [GitHub PAT (personal access token)](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token) and save it in a environment variable:  
//!
//! ```bash
//! export GITHUB_TOKEN=ghp_111111111111111111111
//! ```
//!
//! ## Workspaces with Primary project
//!
//! When a project is a workspace with multiple projects, one of those is the primary project. We want to make a copy of this primary README.md and not of the workspace README.md. We can signal this to `github_readme_copy` with this link:
//!
//! ```html
//! <a href="https://github.com/bestia-dev/cargo_crev_reviews_workspace/tree/main/cargo_crev_reviews">Primary project README.md</a>
//! ```
//!
//! Example: <https://github.com/bestia-dev/cargo_crev_reviews_workspace>
//!
//! ## substack
//!
//! I want to copy also the articles from substack to my domain web page. If anything happens to substack I will have a backup.  
//!
//! ```bash
//! github_readme_copy substack bestiadev
//! ```
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`

// The `lib.rs` does not have any real code. All the code is in modules in separate files.
// The `lib.rs` has just the list of modules, it publishes module's functions or class for the caller
// and it has some global stuff like the Error enum.

// access to modules
mod github_mod;
mod substack_mod;
mod utils_mod;

// `pub use` allows the caller of the lib to access modules functions, structs or all(*)
pub use github_mod::download_readme;
pub use github_mod::github_backup_bash_scripts;
pub use github_mod::upload_github_readme;
pub use github_mod::upload_substack_articles;
pub use substack_mod::substack_download;

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.
use thiserror::Error;

/// all possible library errors for `thiserror`
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("name `{0}` is already uppercase")]
    Uppercase(String),
    #[error("unknown error")]
    Unknown,
}

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";
