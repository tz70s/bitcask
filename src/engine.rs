//! Engine is the core storage engine from memory table to disk records.

use crate::engine::memtable::MemTable;

mod disk;
mod log;
mod memtable;
use futures::lock::Mutex;
use std::sync::Arc;

/// Engine is the core storage engine which operates fully asynchronous.
/// Including the file system based on tokio and few futures based synchronization primitives.
pub struct Engine {
    // include a memory table, should be protected via Arc & Mutext async version.
    memtable: Arc<Mutex<memtable::MemTable>>,
    // disk writes
}

impl Engine {
    pub fn new() -> Self {
        let memtable = MemTable::new();

        let locked = Arc::new(Mutex::new(memtable));

        Engine { memtable: locked }
    }

    // In this layer we should expose four apis as well: get, set, list, rm.
}
