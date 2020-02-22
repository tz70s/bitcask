//! Engine is the core storage engine from memory table to disk records.

use crate::engine::memtable::MemTable;

mod disk;
mod log;
mod memtable;

/// Engine is the core storage engine which operates fully asynchronous.
/// Including the file system based on tokio and few futures based synchronization primitives.
struct Engine {
    // include a memory table, should be protected via Arc & Mutext async version.
    memtable: memtable::MemTable,
    // disk writes
}

impl Engine {
    fn new() -> Self {
        Engine {
            memtable: MemTable::new(),
        }
    }
}
