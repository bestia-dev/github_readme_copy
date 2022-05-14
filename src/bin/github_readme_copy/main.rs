//! github_readme_copy/src/bin/github_readme_copy/main.rs

// This `main.rs` is the code for the CLI application.
// The build of this project will create the CLI application.
// The `main.rs` has all the stdin and stdout.
// The `lib.rs` must be in/out agnostic. That is the responsibility of the `main.rs`
// This `lib.rs` can be used as dependency crate for other projects.

// The `main.rs` uses the `anyhow` error library.
// The `lib.rs` uses the `thiserror` library.

/// entry point into the bin-executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // read from env variable
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    dbg!(&token);
    list_repos(&token);
}

/// list_repos
fn list_repos(token: &str) {
    github_readme_copy::list_my_public_repos(token);
}
