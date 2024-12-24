use std::fmt::Display;

use super::Bookmark;

pub enum Entry {
    Bookmark(Bookmark),
    Comment(String),
    Empty
}

impl Entry {
    pub fn into_bookmark(self) -> Option<Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None
        }
    }

    pub fn bookmark(&self) -> Option<&Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None
        }
    }

    pub fn bookmark_mut(&mut self) -> Option<&mut Bookmark> {
        match self {
            Entry::Bookmark(bookmark) => Some(bookmark),
            _ => None
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