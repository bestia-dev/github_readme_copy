// github_readme_copy/src/github_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

use crate::LibraryError;

/// list_my_public_repos
pub fn list_my_public_repos(token: &str) {
    // create a future and then run it in the tokio runtime
    let future = async move {
        let dest_folder = std::path::Path::new("copied_readme");

        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token.to_string())
            .build()
            .unwrap();

        let page = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("public")
            .sort("full_name")
            .per_page(100)
            .send()
            .await
            .unwrap();

        let vec_of_repo = octocrab
            .all_pages::<octocrab::models::Repository>(page)
            .await
            .unwrap();

        for repo in &vec_of_repo {
            let repo_name = &repo.name;
            let repo_url = repo.html_url.as_ref().unwrap();
            println!("{}", &repo_url);
            // open the html and extract the `article` element
            let body = reqwest::get(repo_url.clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let pos1 = crate::utils_mod::find_pos_end_data_before_delimiter(&body, 0, "<article ")
                .unwrap();
            let pos2 =
                crate::utils_mod::find_pos_start_data_after_delimiter(&body, 0, "</article>")
                    .unwrap();
            let article = &body[pos1..pos2];
            let mut new_html = std::fs::read_to_string("copied_readme/0_template.txt").unwrap();

            let pos3 =
                crate::utils_mod::find_pos_end_data_before_delimiter(&new_html, 0, "\n</body>")
                    .unwrap();
            new_html.replace_range(pos3..pos3, article);

            let path = dest_folder.join(repo_name).with_extension("html");
            if path.exists() {
                let old_html = std::fs::read_to_string(&path).unwrap();
                if old_html != new_html {
                    std::fs::write(&path, new_html).unwrap();
                }
            } else {
                std::fs::write(&path, new_html).unwrap();
            }
        }
        // check if there is some obsolete html
        for entry in dest_folder.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if entry.file_name().to_string_lossy().ends_with(".html") {
                    let mut repo_exists = false;
                    for repo in &vec_of_repo {                       
                        if format!("{}.html", &repo.name) == entry.file_name().to_string_lossy() {
                            repo_exists = true;
                            break;
                        }
                    }
                    if repo_exists == false {
                        // rename the file
                        println!("Obsolete renamed: {}", &entry.file_name().to_string_lossy());
                        std::fs::rename(entry.path(), entry.path().with_extension("obsolete"))
                            .unwrap();
                    }
                }
            }
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
