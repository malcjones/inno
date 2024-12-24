use std::fmt::Display;

pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    /// Create a new bookmark
    pub fn new(name: String, url: String, tags: Vec<String>) -> Self {
        Self { name, url, tags }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} [{}]", self.name, self.url, self.tags.join(", "))
    }
}