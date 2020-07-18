mod structs;
mod tests;

use regex::{Captures, Regex};
use std::fs;
use std::io::prelude::*;
use structs::Resume;

pub fn extract_resume(path: &str) -> Resume {
    let mut file = fs::File::open(path).expect("Unable to read the JSON file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read the JSON file");
    let resume: Resume = serde_json::from_str(&content).expect("Invalid JSON format");

    resume
}

pub fn replace_html_vars(html: &str, resume: Resume) -> String {
    let primitive_vars_re = Regex::new(r"\{\{(?:\s?+?)([\w\.]+)(?:\s?+?)\}\}").unwrap();
    let array_vars_re = Regex::new(r"\{!(?:\s?+)(.+)(?:\s?+)(.[^!]+)!\}").unwrap();
    let array_var_primitive_re = Regex::new(r"\{\s*(\w+)\s*\}").unwrap();

    // Could we use the initial parsed JSON instead?
    let resume_in_json: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&resume).unwrap()).expect("Cannot parse JSON");

    let primitives_replaced = primitive_vars_re.replace_all(html, |caps: &Captures| {
        let value = json_get(&resume_in_json, caps.get(1).unwrap().as_str());

        remove_quotes(&value.as_str().expect("Primitive"))
    });

    let result = array_vars_re.replace_all(&primitives_replaced, |caps: &Captures| {
        let primary_var_name = caps.get(1).unwrap().as_str();
        let html = caps.get(2).unwrap().as_str();
        let values = json_get(&resume_in_json, primary_var_name);
        let mut replaced_html = String::new();
        for value in values.as_array().unwrap().iter() {
            let replaced = array_var_primitive_re.replace_all(&html, |c: &Captures| {
                let key = c.get(1).unwrap().as_str();
                let res = value.get(key).unwrap().as_str().expect("Primitive");
                remove_quotes(res)
            });
            replaced_html.push_str(&replaced);
        }

        replaced_html
    });

    result.into_owned()
}

// Helps to get nested value
pub fn json_get(json: &serde_json::Value, key_str: &str) -> serde_json::Value {
    use serde_json::Value;
    let keys: Vec<&str> = key_str.split(".").collect();
    eprintln!("{:?}", keys);
    let get_err_msg = |key: &str| {
        format!(
            "Invalid key used in the HTML template: \"{}\"; full path: \"{}\"",
            key, key_str
        )
    };
    let mut result: &serde_json::Value = json.get(keys[0]).expect(&get_err_msg(keys[0]));

    for key in keys.iter().skip(1) {
        let value = result.get(key);
        match value {
            Some(v) => match v {
                Value::String(_) | Value::Object(_) | Value::Number(_) => {
                    result = v;
                }
                _ => {
                    result = v;
                    break;
                }
            },
            None => panic!(get_err_msg(key)),
        }
    }

    result.to_owned()
}

pub fn remove_quotes(s: &str) -> String {
    s.replace("\"", "")
}
