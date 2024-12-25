use thiserror::Error;

use std::{fmt::Display, str::FromStr};

use super::Bookmark;

/// An entry in the store.
pub enum Entry {
    Bookmark(Bookmark),
    Comment(String),
    Empty,
}

impl Entry {
    /// If this `Entry` is a `Bookmark`, extract it and consume `self`.
    pub fn into_bookmark(self) -> Option<Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None,
        }
    }

    /// Returns a reference to the bookmark if this `Entry` is a bookmark.
    pub fn bookmark(&self) -> Option<&Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None,
        }
    }

    /// Returns a mutable reference to the bookmark if this `Entry` is a bookmark.
    pub fn bookmark_mut(&mut self) -> Option<&mut Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::Bookmark(bookmark) => write!(f, "{}", bookmark),
            Entry::Comment(comment) => write!(f, "{}", comment),
            Entry::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Error, Debug)]
pub enum EntryParseError {
    #[error("Missing ':' separator in bookmark line: '{0}'")]
    MissingSeparator(String),
    #[error("Bookmark name is empty")]
    EmptyName,
    #[error("Bookmark URL is empty")]
    EmptyUrl,
    #[error("Missing closing ']' for tags")]
    MissingClosingBracket,
}

impl FromStr for Entry {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.trim();

        if let Some(comment) = line.strip_prefix('#') {
            return Ok(Entry::Comment(comment.trim().to_string()));
        }

        if line.is_empty() {
            return Ok(Entry::Empty);
        }

        let Some((name_part, url_part)) = line.split_once(':') else {
            return Err(EntryParseError::MissingSeparator(line.to_string()));
        };

        let name = name_part.trim().to_string();
        if name.is_empty() {
            return Err(EntryParseError::EmptyName);
        }

        let url_part = url_part.trim();
        let mut url = url_part.to_string();
        let mut tags = Vec::new();

        if let Some(start_index) = url_part.find('[') {
            let extracted_url = &url_part[..start_index].trim();
            if extracted_url.is_empty() {
                return Err(EntryParseError::EmptyUrl);
            }
            url = extracted_url.to_string();

            let Some(end_index) = url_part.rfind(']') else {
                return Err(EntryParseError::MissingClosingBracket);
            };

            let tags_str = &url_part[start_index + 1..end_index].trim();
            if !tags_str.is_empty() {
                tags = tags_str
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            }
        }

        if url.is_empty() {
            return Err(EntryParseError::EmptyUrl);
        }

        Ok(Entry::Bookmark(Bookmark { name, url, tags }))
    }
}
