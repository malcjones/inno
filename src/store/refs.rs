use std::{fmt::Display, ops::{Deref, DerefMut}};

use super::Bookmark;

pub struct BookmarkRef<'a> {
    pub id: usize,
    pub bookmark: &'a Bookmark,
}

impl<'a> BookmarkRef<'a> {
    pub fn new(id: usize, bookmark: &'a Bookmark) -> Self {
        Self { id, bookmark }
    }
}

impl Deref for BookmarkRef<'_> {
    type Target = Bookmark;

    fn deref(&self) -> &Self::Target {
        self.bookmark
    }
}

impl Display for BookmarkRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}", self.id, self.bookmark)
    }
}

pub struct BookmarkMut<'a> {
    pub id: usize,
    pub bookmark: &'a mut Bookmark,
}

impl<'a> BookmarkMut<'a> {
    pub fn new(id: usize, bookmark: &'a mut Bookmark) -> Self {
        Self { id, bookmark }
    }
}

impl Deref for BookmarkMut<'_> {
    type Target = Bookmark;

    fn deref(&self) -> &Self::Target {
        self.bookmark
    }
}

impl DerefMut for BookmarkMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bookmark
    }
}

impl Display for BookmarkMut<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. {}", self.id, self.bookmark)
    }
}