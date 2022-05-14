[comment]: # (auto_md_to_doc_comments segment start A)

# github_readme_copy

[comment]: # (auto_cargo_toml_to_md start)

**Basic Rust project template for CLI and library, more than just `cargo new hello`**  
***version: 1.0.4 date: 2022-04-21 author: [bestia.dev](bestia.dev) repository: [Github](https://github.com/bestia-dev/github_readme_copy)***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-89-green.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-13-blue.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-36-purple.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-19-yellow.svg)](https://github.com/bestia-dev/cargo-auto/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-30-orange.svg)](https://github.com/bestia-dev/cargo-auto/)

[comment]: # (auto_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/github_readme_copy/blob/main/LICENSE) [![Rust](https://github.com/bestia-dev/github_readme_copy/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/github_readme_copy/)

## Motivation

It looks like google search SEO is really bad for github README.md files.  
Maybe it will work better as HTML files on my own domain. Yes, it does.  
On every README HTML, there is a link to bestia.dev and a link to the github repository.  
I need a utility CLI that copies the README files as they are rendered on github and save them as html files in the directory `copied_readme`.  
Then I will use an `rsync` command to upload the files to my google cloud virtual machine.

## Octocrab

I will use the crate `octocrab` to get a list of my public repos.  
You need to have a [github PAT (personal access token)](https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token) and save it in a environment variable:  

```bash
export GITHUB_TOKEN=ghp_111111111111111111111
```



## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## open-source and free as a beer

My open-source projects are free as a beer (MIT license).

I just love programming.

But I need also to drink. If you find my projects and tutorials helpful,please buy me a beer donating on my [paypal](https://paypal.me/LucianoBestia).

You know the price of a beer in your local bar ;-) So I can drink a free beer for your health :-)

[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
