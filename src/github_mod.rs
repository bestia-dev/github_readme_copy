// github_readme_copy/src/github_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

// se crate::LibraryError;

/// download public readmes
pub fn download_readme(token: &str) {
    // TODO: I could use rayon to download more then one html in parallel
    // create a future and then run it in the tokio runtime
    let future = async move {
        let dest_folder = std::path::Path::new("copied_readme");
        let vec_of_repo = vec_of_repos_from_github(token).await;

        for repo in &vec_of_repo {
            let repo_name = &repo.name;
            let body = get_readme_body(repo).await;

            let article = get_article(&body);
            let mut new_html = std::fs::read_to_string("copied_readme/0_template.txt").unwrap();

            insert_article(&mut new_html, article);

            let path = dest_folder.join(repo_name).with_extension("html");
            if path.exists() {
                let old_html = std::fs::read_to_string(&path).unwrap();
                if old_html != new_html {
                    std::fs::write(&path, new_html).unwrap();
                }
            } else {
                println!("Writing {}", path.to_string_lossy());
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

fn insert_article(new_html: &mut String, article: &str) {
    let pos3 =
        crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "\n</body>").unwrap();
    new_html.replace_range(pos3..pos3, article);
}

fn get_article(body: &str) -> &str {
    let pos1 = crate::utils_mod::find_pos_end_data_before_delimiter(&body, 0, "<article ").unwrap();
    let pos2 =
        crate::utils_mod::find_pos_start_data_after_delimiter(&body, 0, "</article>").unwrap();
    let article = &body[pos1..pos2];
    article
}

/// get the right readme body
/// if there is a link to >Primary project README.md<, use that instead, for example cargo_crev_reviews_workspace
async fn get_readme_body(repo: &octocrab::models::Repository) -> String {
    let repo_url = repo.html_url.as_ref().unwrap();
    println!("Reading {}", repo_url);
    // open the html
    let body = reqwest::get(repo_url.clone())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // find the magic link "Primary project README.md" it must be header2
    let pos1 = body.find(r#"">Primary project README.md</a></h2>"#);
    match pos1 {
        None => return body,
        Some(pos1) => {
            // extract the link
            let delim2 = r#"<a href=""#;
            let pos2 = body[..pos1].rfind(delim2).expect("The html {} has the phrase >Primary project README.md<, but before that there is no <a href=");
            let pos3 = pos2 + delim2.len();
            let link_url = &body[pos3..pos1];
            println!("Primary project: Reading {}", repo_url);
            let body = reqwest::get(link_url.clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return body;
        }
    }
}

/// only public repos
async fn vec_of_repos_from_github(token: &str) -> Vec<octocrab::models::Repository> {
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
    vec_of_repo
}

pub fn upload_readme(upload_url: &str) {
    let source_folder = std::path::Path::new("copied_readme");
    // easy upload with rsync over SSH
    println!(
        "\nUploading from {}/ to server {}...\n",
        source_folder.to_string_lossy(),
        upload_url
    );
    // the SSh key must be already ssh-add into the ssh-agent
    // rsync -e ssh -avz --delete-after copied_readme luciano_bestia@bestia.dev:/var/www/bestia.dev/docs/
    let mut rsync = std::process::Command::new("rsync");
    rsync
        .arg("-avz")
        .arg("--delete-after")
        .arg("--progress")
        .arg("-e ssh") // tells rsync which port to use
        // path must end with / to signal we want to copy the content and not the directory
        .arg(&format!("{}/", source_folder.to_string_lossy()))
        // path must end with / to signal we want to copy the content and not the directory
        .arg(upload_url);

    rsync.status().expect("rsync failed to execute");
}

#[cfg(test)]
mod test {
    //use super::*;
}
