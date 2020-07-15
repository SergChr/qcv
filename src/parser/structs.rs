use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Resume {
    pub basics: Basics,
    pub work: Vec<Work>,
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
    pub skills: Vec<String>,
    pub languages: Vec<Language>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub summary: String,
    pub location: Location,
    pub profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub country: String,
    pub address: String,
    pub city: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub network: String,
    pub username: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Work {
    pub company: String,
    pub position: String,
    pub website: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Education {
    pub institution: String,
    pub location: String,
    pub area: String,
    pub study_type: String,
    pub start_date: String,
    pub end_date: String,
    pub courses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    pub language: String,
    pub level: String,
}
