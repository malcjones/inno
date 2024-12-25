pub mod bookmark;
pub mod entry;
pub mod refs;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    collections::HashMap,
};

use anyhow::{Context, Result};
use bookmark::Bookmark;
use entry::Entry;
use refs::{BookmarkMut, BookmarkRef};

/// A store for bookmarks and other entries.
pub struct Store {
    pub entries: Vec<Option<Entry>>,
    pub bookmark_index: Vec<usize>,
    pub tag_index: HashMap<String, Vec<usize>>, // maps tags to bookmark IDs
}

impl Store {
    /// Create a new store
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            bookmark_index: Vec::new(),
            tag_index: HashMap::new(),
        }
    }

    /// Load entries from a file    
    pub fn load(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let reader = BufReader::new(File::open(path)?);
        reader
            .lines()
            .enumerate()
            .try_for_each(|(line_number, line)| {
                let line = line.context("Failed to read line")?;
                let entry = line.parse::<Entry>().with_context(|| {
                    format!("Failed to parse entry on line {}", line_number + 1)
                })?;
                self.add_entry(entry);

                Ok::<_, anyhow::Error>(())
            })?;

        Ok(())
    }

    /// Save entries to a file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);

        for entry in &self.entries {
            if let Some(entry) = entry {
                writeln!(writer, "{}", entry)?;
            }
        }

        Ok(())
    }

    /// Add a bookmark to the store and return its bookmark ID
    pub fn add(&mut self, bookmark: Bookmark) -> usize {
        self.add_entry(Entry::Bookmark(bookmark))
    }

    /// Create a new bookmark and add it to the store
    pub fn create(&mut self, name: String, url: String, tags: Vec<String>) -> usize {
        self.add(Bookmark { name, url, tags })
    }

    /// Iterate over all bookmarks in the store
    pub fn iter(&self) -> impl Iterator<Item = BookmarkRef> + '_ {
        self.bookmark_index.iter().filter_map(move |&id| {
            self.entries[id]
                .as_ref()?
                .bookmark()
                .map(|bookmark| BookmarkRef::new(id, bookmark))
        })
    }

    /// Get a reference to a bookmark by bookmark ID
    pub fn get(&self, bookmark_id: usize) -> Option<BookmarkRef> {
        let entry_id = *self.bookmark_index.get(bookmark_id)?;
        self.entries
            .get(entry_id)?
            .as_ref()?
            .bookmark()
            .map(|bookmark| BookmarkRef::new(bookmark_id, bookmark))
    }

    /// Get a mutable reference to a bookmark by bookmark ID
    pub fn get_mut(&mut self, bookmark_id: usize) -> Option<BookmarkMut> {
        let entry_id = *self.bookmark_index.get(bookmark_id)?;
        self.entries
            .get_mut(entry_id)?
            .as_mut()?
            .bookmark_mut()
            .map(|bookmark| BookmarkMut::new(bookmark_id, bookmark))
    }

    /// Get the number of bookmarks in the store
    pub fn bookmark_count(&self) -> usize {
        self.bookmark_index.len()
    }

    /// Get the number of entries in the store
    pub fn entry_count(&self) -> usize {
        self.entries.iter().filter(|entry| entry.is_some()).count()
    }

    /// Get the number of entry slots in the store
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// Get all bookmarks that match a given query
    pub fn find(&self, query: &str) -> Vec<BookmarkRef> {
        self.iter()
            .filter(|bookmark| bookmark.matches(query))
            .collect()
    }

    /// Get all bookmarks that match a given tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<BookmarkRef> {
        self.tag_index
            .get(tag)
            .map(|bookmark_ids| {
                bookmark_ids
                    .iter()
                    .filter_map(|&id| self.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove a bookmark by bookmark ID
    pub fn remove(&mut self, bookmark_id: usize) -> Option<Bookmark> {
        if bookmark_id >= self.bookmark_index.len() {
            return None;
        }

        let entry_id = self.bookmark_index[bookmark_id];
        let removed_entry = self.entries[entry_id].take()?;

        if let Entry::Bookmark(bookmark) = &removed_entry {
            self.remove_from_tag_index(bookmark_id, &bookmark.tags);
        }

        self.bookmark_index.remove(bookmark_id);
        removed_entry.into_bookmark()
    }

    /// Add an entry to the store, returning the entry ID (bookmark ID if a bookmark)
    fn add_entry(&mut self, entry: Entry) -> usize {
        let mut id = self.entries.len();

        if let Entry::Bookmark(bookmark) = &entry {
            self.bookmark_index.push(id);
            let bookmark_id = self.bookmark_index.len() - 1;
            self.index_tags(bookmark_id, &bookmark.tags);
            id = bookmark_id;
        }

        self.entries.push(Some(entry));

        id
    }

    fn index_tags(&mut self, bookmark_id: usize, tags: &[String]) {
        for tag in tags {
            self.tag_index
                .entry(tag.clone())
                .or_default()
                .push(bookmark_id);
        }
    }

    fn remove_from_tag_index(&mut self, bookmark_id: usize, tags: &[String]) {
        for tag in tags {
            if let Some(bookmarks) = self.tag_index.get_mut(tag) {
                if let Some(pos) = bookmarks.iter().position(|&id| id == bookmark_id) {
                    bookmarks.remove(pos);
                }
                if bookmarks.is_empty() {
                    self.tag_index.remove(tag);
                }
            }
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_store() -> Store {
        let mut store = Store::new();

        // Populate the store with some bookmarks & a comment
        store.add(Bookmark {
            name: "Rust".to_string(),
            url: "https://www.rust-lang.org".to_string(),
            tags: vec!["rust".to_string()],
        }); // bookmark_id = 0, entry_id = 0

        store.add_entry(Entry::Comment("A comment".to_string())); // entry_id = 1, not a bookmark

        store.add(Bookmark {
            name: "The Rust Programming Language".to_string(),
            url: "https://doc.rust-lang.org/book/".to_string(),
            tags: vec!["rust".to_string()],
        }); // bookmark_id = 1, entry_id = 2

        store
    }

    #[test]
    fn test_get() {
        let store = mock_store();

        // Ensure the bookmarks were added correctly
        let bookmark = store.get(0).unwrap();
        assert_eq!(bookmark.id, 0);
        assert_eq!(bookmark.name, "Rust");

        let bookmark = store.get(1).unwrap();
        assert_eq!(bookmark.id, 1);
        assert_eq!(bookmark.name, "The Rust Programming Language");
    }

    #[test]
    fn test_get_mut() {
        let mut store = mock_store();

        {
            // Modify the bookmark name
            let mut bookmark = store.get_mut(0).unwrap();
            bookmark.name = "Rust Programming".to_string();
        }

        // Ensure the change is persisted
        let bookmark = store.get(0).unwrap();
        assert_eq!(bookmark.name, "Rust Programming");
    }

    #[test]
    fn test_remove() {
        let mut store = mock_store();

        // Remove the first bookmark
        let bookmark = store.remove(0).unwrap();
        assert_eq!(bookmark.name, "Rust");

        // After removal, the bookmark ID 1 shifts down to ID 0
        let bookmark = store.get(0).unwrap();
        assert_eq!(bookmark.id, 0);
        assert_eq!(bookmark.name, "The Rust Programming Language");

        // The original bookmark ID 1 no longer exists
        assert!(store.get(1).is_none());
    }

    #[test]
    fn test_tombstone_preservation() {
        let mut store = mock_store();

        // Remove the first bookmark
        store.remove(0);

        // Ensure comments and other entries remain untouched
        assert!(store.entries[1].is_some()); // Comment remains in place
        assert!(store.entries[0].is_none()); // Tombstoned entry
    }
}
