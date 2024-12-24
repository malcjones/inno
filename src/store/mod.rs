pub mod bookmark;
pub mod entry;
pub mod refs;

use bookmark::Bookmark;
use entry::Entry;
use refs::{BookmarkMut, BookmarkRef};

/// A store for bookmarks and other entries.
pub struct Store {
    pub entries: Vec<Option<Entry>>,
    pub bookmark_index: Vec<usize>,
}

impl Store {
    /// Create a new store
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            bookmark_index: Vec::new(),
        }
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

    /// Remove a bookmark by bookmark ID
    pub fn remove(&mut self, bookmark_id: usize) -> Option<Bookmark> {
        // Ensure the bookmark ID is valid
        if bookmark_id >= self.bookmark_index.len() {
            return None;
        }

        // Get the entry ID for the bookmark
        let entry_id = self.bookmark_index[bookmark_id];

        // Tombstone the entry in entries
        let removed_entry = self.entries[entry_id].take()?;

        // Remove the bookmark ID and shift subsequent bookmarks down
        self.bookmark_index.remove(bookmark_id);

        removed_entry.into_bookmark()
    }

    /// Add an entry to the store, returning the entry ID (bookmark ID if a bookmark)
    fn add_entry(&mut self, entry: Entry) -> usize {
        let mut id = self.entries.len();

        if let Entry::Bookmark(_) = entry {
            self.bookmark_index.push(id);
            id = self.bookmark_index.len() - 1;
        }

        self.entries.push(Some(entry));

        id
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
