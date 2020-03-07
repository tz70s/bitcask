//! Key -> Log Offset

use super::log::Meta;
use std::collections::{HashMap, hash_map};

/// Store key -> log offset
#[derive(Debug)]
pub struct MemTable {
    indexes: HashMap<String, Meta>,
}

impl MemTable {
    /// Create a new MemTable.
    pub fn new() -> Self {
        MemTable {
            indexes: HashMap::new(),
        }
    }

    /// Set insert or update a key and offset pair.
    /// We don't need to distinguish two differences.
    pub fn set(&mut self, key: String, meta: Meta) {
        self.indexes.insert(key, meta);
    }

    /// Get get the associated offset from given key.
    pub fn get(&self, key: &str) -> Option<&Meta> {
        self.indexes.get(key)
    }

    /// Remove a key and associated offset.
    pub fn rm(&mut self, key: &str) {
        self.indexes.remove(key);
    }

    /// Pass out the internal iterator for list API.
    pub fn iter(&self) -> hash_map::Iter<'_, String, Meta> {
        self.indexes.iter()
    }
}
