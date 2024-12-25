use std::fmt::Display;

pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    /// Create a new bookmark.
    pub fn new(name: String, url: String, tags: Vec<String>) -> Self {
        Self { name, url, tags }
    }

    /// Return the bookmark's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the bookmark's URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Return the bookmark's tags.
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Sets the bookmark's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the bookmark's URL.
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    /// Sets the bookmark's tags.
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    /// Checks if the bookmark matches a fuzzy query.
    pub fn matches(&self, query: &str) -> bool {
        self.to_string()
            .to_lowercase()
            .contains(&query.to_lowercase())
    }
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} [{}]", self.name, self.url, self.tags.join(", "))
    }
}
