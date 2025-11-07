// github_readme_copy/src/substack_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

// use crate::LibraryError;
use serde::Deserialize;

use crate::{pos, ResultLogError};

#[derive(Deserialize, Debug)]
struct SubstackArticle {
    canonical_url: String,
    slug: String,
    post_date: String,
}

/// download substack articles from <https://bestiadev.substack.com/archive>
pub fn substack_download(substack_url: &str) -> anyhow::Result<()> {
    let dest_folder = std::path::Path::new("tmp/substack_articles");
    if !dest_folder.exists() {
        panic!("Error: Folder {} does not exist.", dest_folder.to_string_lossy())
    }
    // copy directory structure from template
    std::fs::copy(
        "template_for_substack_articles/bestia_icon.png",
        "tmp/substack_articles/bestia_icon.png",
    )
    .log(pos!())?;
    std::fs::copy("template_for_substack_articles/README.md", "tmp/substack_articles/README.md").log(pos!())?;
    std::fs::create_dir_all("tmp/substack_articles/css").log(pos!())?;
    std::fs::copy(
        "template_for_substack_articles/css/bestia01.css",
        "tmp/substack_articles/css/bestia01.css",
    )
    .log(pos!())?;
    std::fs::copy(
        "template_for_substack_articles/css/bestia01.css",
        "tmp/substack_articles/css/bestia01.css",
    )
    .log(pos!())?;
    std::fs::copy(
        "template_for_substack_articles/css/normalize.css",
        "tmp/substack_articles/css/normalize.css",
    )
    .log(pos!())?;
    std::fs::copy(
        "template_for_substack_articles/css/Roboto-Medium.woff2",
        "tmp/substack_articles/css/Roboto-Medium.woff2",
    )
    .log(pos!())?;

    // get href from <a href="..." class="post-preview-title newsletter">title</a>

    // create a future and then run it in the tokio runtime
    let rt1 = tokio::runtime::Runtime::new().log(pos!())?;
    let future1 = async move { vec_of_public_repos_from_github(substack_url).await };
    let vec_of_article = rt1.block_on(future1).log(pos!())?;
    // rayon uses many threads to download in parallel
    let pool = rayon::ThreadPoolBuilder::new().build().log(pos!())?;
    pool.scope(|scoped| {
        for article in &vec_of_article {
            let canonical_url = article.canonical_url.clone();
            let file_name = article.slug.clone();
            let post_date = article.post_date.clone();
            scoped.spawn(move |_s| {
                println!("Reading {}", &canonical_url);
                let body = reqwest::blocking::get(&canonical_url)
                    .expect("Inside closure.")
                    .text()
                    .expect("Inside closure.");
                let article = get_article(&body).expect("Inside closure.");
                let mut new_html = std::fs::read_to_string("template_for_substack_articles/0_template.txt").expect("Inside closure.");
                insert_title(&mut new_html, &file_name).expect("inside closure");
                insert_original_url(&mut new_html, &canonical_url).expect("inside closure");
                // 2 times insert_original_url
                insert_original_url(&mut new_html, &canonical_url).expect("inside closure");
                insert_post_date(&mut new_html, &post_date).expect("inside closure");
                insert_article(&mut new_html, &article).expect("inside closure");

                let path = dest_folder.join(file_name).with_extension("html");
                if path.exists() {
                    let old_html = std::fs::read_to_string(&path).expect("Inside closure.");
                    if old_html != new_html {
                        println!("Writing {}", path.to_string_lossy());
                        std::fs::write(&path, new_html).expect("Inside closure.");
                    }
                } else {
                    println!("Writing {}", path.to_string_lossy());
                    std::fs::write(&path, new_html).expect("Inside closure.");
                }
            });
        }
    });
    Ok(())
}

/// list urls from all articles from /api/v1/archive
async fn vec_of_public_repos_from_github(substack_url: &str) -> anyhow::Result<Vec<SubstackArticle>> {
    let archive_url = format!("https://{}/api/v1/archive", substack_url);
    println!("    Reading list of articles: {}", &archive_url);
    // return list
    Ok(reqwest::get(archive_url.clone())
        .await
        .log(pos!())?
        .json::<Vec<SubstackArticle>>()
        .await
        .log(pos!())?)
}

fn get_article(body: &str) -> anyhow::Result<String> {
    let pos1 = crate::utils_mod::find_pos_end_data_before_delimiter(body, 0, "<article ")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    let pos2 = crate::utils_mod::find_pos_start_data_after_delimiter(body, 0, "</article>")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    let article = &body[pos1..pos2];

    let article = remove_div_post_footer(article).log(pos!())?;
    let article = remove_div_role_dialog(&article).log(pos!())?;
    let article = remove_div_class_anchor(&article).log(pos!())?;
    let article = remove_a_role_button(&article).log(pos!())?;
    let article = remove_attribute_a_rel_nofollow(&article).log(pos!())?;
    //return article
    remove_dev_class_pencraft(&article).log(pos!())
    //let article = img_src_modify(&article).log(pos!())?;
}

fn insert_title(new_html: &mut String, title: &str) -> anyhow::Result<()> {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "<title>template</title>")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    new_html.replace_range(pos3 + 7..pos3 + 15, title);
    Ok(())
}

fn insert_original_url(new_html: &mut String, canonical_url: &str) -> anyhow::Result<()> {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "canonical_url")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    new_html.replace_range(pos3..pos3 + 13, canonical_url);
    Ok(())
}

fn insert_post_date(new_html: &mut String, post_date: &str) -> anyhow::Result<()> {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "post_date")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    new_html.replace_range(pos3..pos3 + 9, &format!("Article date: {post_date}"));
    Ok(())
}

fn insert_article(new_html: &mut String, article: &str) -> anyhow::Result<()> {
    let pos3 = crate::utils_mod::find_pos_end_data_before_delimiter(&*new_html, 0, "\n</body>")
        .ok_or_else(|| anyhow::anyhow!("delimiter is None"))
        .log(pos!())?;
    new_html.replace_range(pos3..pos3, article);
    Ok(())
}

/// remove <div class="post-footer...
fn remove_div_post_footer(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("div[class]", |el| {
            let href = el.get_attribute("class").unwrap_or("".to_string());
            if href.contains("post-footer") {
                el.remove();
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}

/// remove  <div role="dialog"
fn remove_div_role_dialog(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("div[role]", |el| {
            let href = el.get_attribute("role").unwrap_or("".to_string());
            if href.contains("dialog") {
                el.remove();
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}

// remove <div class="header-anchor-widget offset-top">
fn remove_div_class_anchor(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("div[class]", |el| {
            let href = el.get_attribute("class").unwrap_or("".to_string());
            if href.contains("header-anchor-widget") {
                el.remove();
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}

// remove <a role="button"
fn remove_a_role_button(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("a[role]", |el| {
            let href = el.get_attribute("role").unwrap_or("".to_string());
            if href.contains("button") {
                el.remove();
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}

// remove attribute <a rel="nofollow ugc noopener">
fn remove_attribute_a_rel_nofollow(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("a[rel]", |el| {
            let href = el.get_attribute("rel").unwrap_or("".to_string());
            if href.contains("nofollow") {
                el.remove_attribute("rel")
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}

// remove <div class="pencraft
fn remove_dev_class_pencraft(article: &str) -> anyhow::Result<String> {
    use lol_html::{element, rewrite_str, RewriteStrSettings};
    let element_content_handlers = vec![
        // Rewrite insecure hyperlinks
        element!("div[class]", |el| {
            let href = el.get_attribute("class").unwrap_or("".to_string());
            if href.contains("pencraft") {
                el.remove();
            }
            Ok(())
        }),
    ];

    let output = rewrite_str(
        article,
        RewriteStrSettings {
            element_content_handlers,
            ..RewriteStrSettings::default()
        },
    )
    .log(pos!())?;

    Ok(output)
}
