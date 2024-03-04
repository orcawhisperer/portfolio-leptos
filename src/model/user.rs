use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    pub title: String,
    pub duration: String,
    pub responsibilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Experience {
    pub company: String,
    pub company_url: String,
    pub logo: String,
    pub location: String,
    pub positions: Vec<Position>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub title: String,
    pub category: String,
    pub proficiency: String,
    pub proficiency_level: u8,
    pub color: String,
    pub icon: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Certification {
    pub title: String,
    pub provider: String,
    pub date: String,
    pub link: String,
    pub badge: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contact {
    pub contact_type: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub title: String,
    pub about: String,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub certifications: Vec<Certification>,
    pub contacts: Vec<Contact>,
}

impl User {
    pub fn new() -> User {
        User {
            name: "".to_string(),
            title: "".to_string(),
            about: "".to_string(),
            skills: vec![],
            experience: vec![],
            certifications: vec![],
            contacts: vec![],
        }
    }
}
