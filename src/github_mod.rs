// github_readme_copy/src/github_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

use crate::LibraryError;

/// list_my_public_repos
pub fn list_my_public_repos(token: &str) {
    log::warn!("start list_my_public_repos()");
    // create a future and then run it in the tokio runtime
    let future = async move {
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .unwrap();

        let page = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("owner")
            .sort("updated")
            .per_page(100)
            .send()
            .await
            .unwrap();

        let vec_of_repo = octocrab
            .all_pages::<octocrab::models::Repository>(page)
            .await
            .unwrap();

        for repo in vec_of_repo {
            println!("{}", repo.name);
        }
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(future);
}

/// format the hello phrase with uppercase name
/// if it is already uppercase, return error with thiserror
pub fn format_upper_hello_phrase(greet_name: &str) -> Result<String, LibraryError> {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let upper_greet_name = make_uppercase(greet_name);
    if upper_greet_name == greet_name {
        return Err(LibraryError::Uppercase(greet_name.to_string()));
    }

    // return
    Ok(format!("Hello {}!", &upper_greet_name))
}

/// return uppercase
pub fn make_uppercase(greet_name: &str) -> String {
    // return
    greet_name.to_uppercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_format_upper_hello_phrase() {
        assert_eq!(
            format_upper_hello_phrase("abcd").expect("error"),
            "Hello ABCD!"
        );
        assert!(format_upper_hello_phrase("ABCD").is_err());
    }

    #[test]
    pub fn test_make_uppercase() {
        assert_eq!(make_uppercase("abcd"), "ABCD");
        assert_eq!(make_uppercase("1234abcd"), "1234ABCD");
        assert_eq!(make_uppercase("čšž"), "ČŠŽ");
    }
}
