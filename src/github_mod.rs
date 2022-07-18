// github_readme_copy/src/github_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

// se crate::LibraryError;

/// download public readmes
pub fn download_readme(token: &str) {
    let dest_folder = std::path::Path::new("copied_readme");
    // create a future and then run it in the tokio runtime
    let rt1 = tokio::runtime::Runtime::new().unwrap();
    let future1 = async move { vec_of_public_repos_from_github(token).await };
    let vec_of_repo = rt1.block_on(future1);

    // 12 threads to download in parallel
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build()
        .unwrap();
    pool.scope(|scoped| {
        for repo in &vec_of_repo {
            let repo_name = &repo.name;
            scoped.spawn(move |_s| {
                // create a future and then run it in the tokio runtime
                //let measure_instant = std::time::Instant::now();
                let rt2 = tokio::runtime::Runtime::new().unwrap();
                //println!( "Elapsed time tokio::runtime::Runtime::new(): {} ms", measure_instant.elapsed().as_millis() );

                let future2 = async move { get_readme_body(repo).await };
                let (body, title, description) = rt2.block_on(future2);
                //let measure_instant = std::time::Instant::now();
                let article = get_article(&body);
                //println!( "Elapsed time get_article: {} ms", measure_instant.elapsed().as_millis() );
                let mut new_html = std::fs::read_to_string("copied_readme/0_template.txt").unwrap();

                insert_title(&mut new_html, &title);
                insert_description(&mut new_html, &description);
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
            });
        }
    });
    // check if there is some obsolete html
    rename_obsolete_html(dest_folder, &vec_of_repo);
}

// rename obsolete html
fn rename_obsolete_html(
    dest_folder: &std::path::Path,
    vec_of_repo: &Vec<octocrab::models::Repository>,
) {
    for entry in dest_folder.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if entry.file_name().to_string_lossy().ends_with(".html") {
                let mut repo_exists = false;
                for repo in vec_of_repo {
                    if format!("{}.html", &repo.name) == entry.file_name().to_string_lossy() {
                        repo_exists = true;
                        break;
                    }
                }
                if repo_exists == false {
                    // rename the file
                    println!("Obsolete renamed: {}", &entry.file_name().to_string_lossy());
                    std::fs::rename(entry.path(), entry.path().with_extension("obsolete")).unwrap();
                }
            }
        }
    }
}

fn insert_article(new_html: &mut String, article: &str) {
    let pos3 =
        crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "\n</body>").unwrap();
    new_html.replace_range(pos3..pos3, article);
}

fn insert_title(new_html: &mut String, title: &str) {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(
        &*new_html,
        0,
        "<title>template</title>",
    )
    .unwrap();
    new_html.replace_range(pos3 + 7..pos3 + 15, title);
}
fn insert_description(new_html: &mut String, description: &str) {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(
        &*new_html,
        0,
        r#"content="Learning Rust Wasm/Webassembly programming and having fun""#,
    )
    .unwrap();
    new_html.replace_range(pos3 + 9..pos3 + 66, description);
}

fn get_article(body: &str) -> &str {
    let pos1 = crate::utils_mod::find_pos_end_data_before_delimiter(&body, 0, "<article ").unwrap();
    let pos2 =
        crate::utils_mod::find_pos_start_data_after_delimiter(&body, 0, "</article>").unwrap();
    let article = &body[pos1..pos2];
    article
}

fn get_long_title(body: &str) -> &str {
    let pos1 = crate::utils_mod::find_pos_start_data_after_delimiter(&body, 0, "<title>").unwrap();
    let pos2 = crate::utils_mod::find_pos_end_data_before_delimiter(&body, 0, "</title>").unwrap();
    let title = &body[pos1..pos2];
    title
}

fn get_github_description<'a>(body: &'a str, title: &str) -> &'a str {
    let pos1 = crate::utils_mod::find_pos_start_data_after_delimiter(
        &body,
        0,
        r#"</h1>
<p dir="auto"><strong>"#,
    )
    .expect(&format!("not found github description start for {title}"));
    let pos2 = crate::utils_mod::find_pos_end_data_before_delimiter(&body, 0, "</strong><br>")
        .expect(&format!("not found github description end for {title}"));
    let github_description = &body[pos1..pos2];
    github_description
}

/// get the right readme body
/// if there is a link to >Primary project README.md<, use that instead, for example cargo_crev_reviews_workspace
async fn get_readme_body(repo: &octocrab::models::Repository) -> (String, String, String) {
    let repo_url = repo.html_url.as_ref().unwrap();
    println!("    Reading {}", repo_url);
    // open the html
    let body = reqwest::get(repo_url.clone())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // get title and description
    // They are already HTML encoded, because they come from a HTML
    // find and parse: <title>GitHub - bestia-dev/github_readme_copy: Copy my public README.md files from Github in HTML format</title>
    let title = get_long_title(&body);
    let mut spl = title.split(": ");
    let title = spl
        .next()
        .unwrap()
        .trim_start_matches("GitHub - bestia-dev/")
        .to_string();
    let description = spl.next().unwrap().to_string();

    // check if the description of the project and the github description is the same
    let github_description = get_github_description(&body, &title);
    if github_description != description {
        println!("");
        println!("    description different:");
        println!("    {title}");

        println!("    {description}");
        println!("    {github_description}");
        println!("");
    }

    // find the magic link "Primary project README.md" it must be header2
    let pos1 = body.find(r#"">Primary project README.md</a></h2>"#);
    match pos1 {
        None => return (body, title, description),
        Some(pos1) => {
            // extract the link
            let delim2 = r#"<a href=""#;
            let pos2 = body[..pos1].rfind(delim2).expect("The html {} has the phrase >Primary project README.md<, but before that there is no <a href=");
            let pos3 = pos2 + delim2.len();
            let link_url = &body[pos3..pos1];
            println!("    Primary project: Reading {}", repo_url);
            let body = reqwest::get(link_url.clone())
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            return (body, title, description);
        }
    }
}

/// only public repos
async fn vec_of_public_repos_from_github(token: &str) -> Vec<octocrab::models::Repository> {
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

/// private and public repos
async fn vec_of_private_and_public_repos_from_github(
    token: &str,
) -> Vec<octocrab::models::Repository> {
    let octocrab = octocrab::Octocrab::builder()
        .personal_token(token.to_string())
        .build()
        .unwrap();
    let page = octocrab
        .current()
        .list_repos_for_authenticated_user()
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

/// create bash script for backup of all Github repositories
pub fn github_backup_bash_scripts(token: &str) {
    let dest_folder = std::path::Path::new("bash_script_for_backup");
    // create a future and then run it in the tokio runtime
    let rt1 = tokio::runtime::Runtime::new().unwrap();
    let future1 = async move { vec_of_private_and_public_repos_from_github(token).await };
    let vec_of_repo = rt1.block_on(future1);

    let num_of_repo = format!("{}", vec_of_repo.len());
    let path_base = r#"c:\Users\Luciano\Dropbox\BestiaDev\github_backup"#;
    let mut pull_script = String::from(&format!(
        r#":: pull_all.cmd
:: script to pull all the changes from github into local folder github_backup

:: num of repositories: {num_of_repo}

ECHO OFF

"#
    ));
    let mut push_script = String::from(&format!(
        r#":: pull_all.cmd
:: script to push all the changes from local folder github_backup to github

:: num of repositories: {num_of_repo}

ECHO OFF

"#
    ));

    for repo in &vec_of_repo {
        let repo_name = &repo.name;
        pull_script.push_str(&format!(
            r#"cd {path_base}\{repo_name}\
echo %cd%
 git pull
"#
        ));

        push_script.push_str(&format!(
            r#"cd {path_base}\{repo_name}\
echo %cd%
 git commit -a -m "2022-07-17" 
 git push
"#
        ));
    }
    pull_script.push_str(&format!(
        r#"
cd {path_base}\
"#
    ));
    push_script.push_str(&format!(
        r#"
cd {path_base}\
"#
    ));
    let path = dest_folder
        .join("pull_all_for_backup")
        .with_extension("cmd");
    std::fs::write(&path, pull_script).unwrap();
    let path = dest_folder
        .join("push_all_for_backup")
        .with_extension("cmd");
    std::fs::write(&path, push_script).unwrap();
}

#[cfg(test)]
mod test {
    //use super::*;
}
