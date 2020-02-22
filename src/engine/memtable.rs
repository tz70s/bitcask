//! Key -> Log Offset

use super::log::Offset;
use std::collections::HashMap;

/// Store key -> log offset
#[derive(Debug)]
pub struct MemTable {
    indexes: HashMap<String, Offset>,
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
    pub fn set(&mut self, key: String, offset: Offset) {
        self.indexes.insert(key, offset);
    }

    /// Get get the associated offset from given key.
    pub fn get(&self, key: &str) -> Option<&Offset> {
        self.indexes.get(key)
    }

    /// Remove a key and associated offset.
    pub fn rm(&mut self, key: &str) {
        self.indexes.remove(key);
    }
}
