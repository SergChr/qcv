mod structs;

use std::fs;
use std::io::prelude::*;
use structs::Resume;
use regex::{Regex, Captures};

pub fn extract_resume(path: &str) -> Resume {
    let mut file = fs::File::open(path)
        .expect("Unable to read the JSON file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read the JSON file");
    let resume: Resume = serde_json::from_str(&content)
        .expect("Invalid JSON format");

    resume
}

pub fn replace_html_vars(html: &str, resume: Resume) -> String {
    let vars_re = Regex::new(r"\{\{(?:\s?+?)(\w+.+)(?:\s?+?)\}\}").unwrap();
    let json: serde_json::Value = serde_json::from_str(
        &serde_json::to_string(&resume).unwrap()
    ).expect("Cannot parse JSON");
    let result = vars_re.replace_all(html, |caps: &Captures| {
        let value = json_get(&json, caps[1].to_string());
        remove_quotes(&value.to_string()[..])
    });

    result.to_string()
}

// Helps to get nested value
fn json_get(json: &serde_json::Value, key_str: String) -> &serde_json::Value {
    let keys: Vec<String> = key_str.split(".").map(|s| s.to_string()).collect();
    let mut result: &serde_json::Value = json.get(&keys[0]).unwrap();

    for key in &keys {
        if key == &keys[0] {
            continue;
        }
        result = result.get(key).unwrap();
    }

    result
}

fn remove_quotes(str: &str) -> String {
    str.replace("\"", "")
}
