use std::fs;
use std::io::prelude::*;
use rust_embed::RustEmbed;
use std::path::Path;

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
        "build" => cmd_build(),
        "build-from" => cmd_build_from(),
        _ => help(),
    }
}

fn help() {
    println!(r#"
    Available commands:
    - init
        Create a default resume JSON template. The output will be "cv.json" file where you should put your information.
    - build <theme>
        Generate a HTML file based on the cv.json file. The output will be "cv.html" file.
        See the documentation for the list of available themes. Example of use: qcv build simple
    - build-from <custom_html_path>
        Generate a HTML file based on the cv.json BUT with your custom HTML theme.
        Example: qcv build-from my_theme.html
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

fn cmd_build_from() {
    let theme_path = std::env::args().nth(2)
        .expect("No theme path provided");
    let html = fs::read_to_string(theme_path)
        .expect("Cannot find the theme");
    build_from_template(&html);
}

fn cmd_build() {
    let theme = std::env::args().nth(2)
        .expect("No theme name provided");
    let theme_path = format!("themes/{}/index.html", theme);
    let html_raw = Asset::get(&theme_path)
        .expect("Cannot find html file for the theme",);
    let html = match std::str::from_utf8(html_raw.as_ref()) {
        Ok(str) => str,
        Err(e) => panic!(e),
    };
    build_from_template(html);
}

fn build_from_template(html: &str) {
    let resume = parser::extract_resume(JSON_FILE_NAME);
    let result = parser::replace_html_vars(html, resume);
    let write_file = || {
        fs::write(OUTPUT_HTML_FILE_NAME, result)
            .expect("Cannot write the result to html file");
    };

    let is_html_exists = Path::new(OUTPUT_HTML_FILE_NAME).exists();
    if is_html_exists {
        match fs::remove_file(OUTPUT_HTML_FILE_NAME) {
            Ok(()) => write_file(),
            Err(_e) => panic!("Cannot rewrite the HTML file"),
        };
    } else {
        write_file();
    }
}
