mod structs;
mod tests;

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
    let primitive_vars_re = Regex::new(r"\{\{(?:\s?+?)([\w\.]+)(?:\s?+?)\}\}").unwrap();
    let array_vars_re = Regex::new(r"\{!(?:\s?+)(.+)(?:\s?+)(.[^!]+)!\}").unwrap();
    let array_var_primitive_re = Regex::new(r"\{\s*(\w+)\s*\}").unwrap();

    let resume_in_json: serde_json::Value = serde_json::from_str(
        &serde_json::to_string(&resume).unwrap()
    ).expect("Cannot parse JSON");

    let primitives_replaced = primitive_vars_re.replace_all(html, |caps: &Captures| {
        let value = json_get(&resume_in_json, caps[1].to_string());

        remove_quotes(&value.to_string()[..])
    });

    let result = array_vars_re.replace_all(&primitives_replaced, |caps: &Captures| {
        let primary_var_name = caps[1].to_string();
        let html = caps[2].to_string();
        let values = json_get(&resume_in_json, primary_var_name);
        let mut replaced_html = String::new();
        for value in values.as_array().unwrap().iter() {
            let replaced = array_var_primitive_re.replace_all(&html, |c: &Captures| {
                let key = c[1].to_string();
                let res = value[key].to_string();
                remove_quotes(&res)
            });
            replaced_html.push_str(&replaced);
        }

        replaced_html
    });

    result.to_string()
}

// Helps to get nested value
pub fn json_get(json: &serde_json::Value, key_str: String) -> serde_json::Value {
    let keys: Vec<String> = key_str.split(".").map(|s| s.to_string()).collect();
    let get_err_msg = |key: &String| {
        format!("Invalid key used in the HTML template: \"{}\"; full path: \"{}\"", key, key_str)
    };
    let mut result: &serde_json::Value = json.get(&keys[0])
        .expect(&get_err_msg(&keys[0]));

    for key in &keys {
        if key == &keys[0] {
            continue;
        }
        let value = result.get(key);
        match value {
            Some(v) => {
                if v.is_string() || v.is_object() || v.is_number() {
                    result = v;
                } else {
                    result = v;
                    break;
                }
            }
            None => panic!(get_err_msg(key)),
        }
    }

    result.to_owned()
}

pub fn remove_quotes(str: &str) -> String {
    str.replace("\"", "")
}
