use std::fs;
use std::io::prelude::*;
use rust_embed::RustEmbed;

use crate::parser;

const JSON_FILE_NAME: &str = "cv.json";
const OUTPUT_HTML_FILE_NAME: &str = "cv.html";

#[derive(RustEmbed)]
#[folder = "src/assets"]
struct Asset;

pub fn init() {
    let command = match std::env::args().nth(1) {
        Some(cmd) => cmd,
        None => "help".to_string(),
    };
    match command.as_str() {
        "init" => create_json(),
        "build" => build_template(),
        _ => help(),
    }
}

fn help() {
    println!(r#"
    Available commands:
    - init -- create a default resume JSON template
    - build -- generate a HTML file based on the cv.json template
    "#);
}

fn create_json() {
    let mut file = match fs::OpenOptions::new().write(true)
        .create_new(true)
        .open(JSON_FILE_NAME) {
            Ok(f) => f,
            Err(_err) => panic!("Cannot create a JSON file. Probably it already exists.")
        };
    println!("File created.");
    let template = Asset::get("cv_template.json")
        .expect("Cannot read the CV template file");
    file.write(&template)
        .expect("Cannot init a CV template");
    
}

fn build_template() {
    let theme = std::env::args().nth(2)
        .expect("No theme provided");
    let resume = parser::extract_resume(JSON_FILE_NAME);
    let theme_path = format!("themes/{}/index.html", theme);
    let html_raw = Asset::get(&theme_path)
        .expect("Cannot find html file for the theme",);
    let html_str = match std::str::from_utf8(html_raw.as_ref()) {
        Ok(str) => str,
        Err(e) => panic!(e),
    };
    let result = parser::replace_html_vars(html_str, resume);

    fs::write(OUTPUT_HTML_FILE_NAME, result)
        .expect("Cannot write the result to html file");
}
