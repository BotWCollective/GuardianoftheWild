use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    tags: HashMap<String, String>,
    username: String,
}

impl User {
    pub fn parse(tags: &str, username: &str) -> User {
        let tags = tags
            .split(';')
            .map(|p| {
                let p = p.split_once('=').unwrap_or(("", ""));
                (p.0.to_string(), p.1.to_string())
            })
            .collect::<HashMap<String, String>>();
        Self {
            tags,
            username: username.to_string(),
        }
    }
    pub fn tag(&self, k: &str) -> Option<&String> {
        self.tags.get(k)
    }
    pub fn username(&self) -> &String {
        &self.username
    }
}
