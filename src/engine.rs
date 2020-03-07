//! Engine is the core storage engine from memory table to disk records.

use crate::engine::memtable::MemTable;

mod log;
mod memtable;
mod storage;

use failure::Error;
use futures::lock::Mutex;
use log::Record;
use std::sync::Arc;

/// Engine is the core storage engine which operates fully asynchronous.
/// Including the file system based on tokio and few futures based synchronization primitives.
///
/// It's intended to be thread-safe.
pub struct Engine {
    // include a memory table, should be protected via Arc & Mutext async version.
    // TODO: push down mutext to internal module, and make memtable async.
    memtable: Arc<Mutex<memtable::MemTable>>,

    // disk reads and writes
    storage: Arc<storage::Storage>,
}

impl Engine {
    pub async fn new() -> Self {
        let memtable = MemTable::new();

        let locked = Arc::new(Mutex::new(memtable));

        let storage = storage::Storage::new().await;

        Engine {
            memtable: locked,
            storage: Arc::new(storage),
        }
    }

    // Get record from given key, asynchronously.
    pub async fn get(&self, key: &str) -> Result<Option<Record>, Error> {
        // Get the file offset first.
        let inner = self.memtable.lock().await;
        let meta = inner.get(&key);

        let record = match meta {
            Some(m) => {
                let repr = self.storage.pread(m.offset, m.size).await?;
                let record = Record::from_repr(repr)?;
                Some(record)
            }
            None => None,
        };

        Ok(record)
    }

    // Set record from given key and value, asynchronously.
    pub async fn set(&self, key: String, val: String) -> Result<(), Error> {
        // TODO: eliminate this clone.
        let key_clone = key.clone();
        let record = Record::new(key, val);
        let bytes = record.to_bytes()?;
        let size = bytes.len();

        // TODO: we shouldn't rely on the size before write (should be after write).
        let offset = self.storage.append(bytes).await?;

        let meta = log::Meta { offset, size };
        let mut inner = self.memtable.lock().await;
        inner.set(key_clone, meta);
        Ok(())
    }

    // List all records asynchronously.
    // TODO: use stream is much better, but need to change the grpc interface as well.
    pub async fn list(&self) -> Result<Vec<Record>, Error> {
        let inner = self.memtable.lock().await;

        let mut records = vec![];

        for (k, meta) in inner.indexes.iter() {
            // To do so, we terminate the iteration when any of error occurred.
            // Should open up to further making progress?
            let repr = self.storage.pread(meta.offset, meta.size).await?;
            let record = Record::from_repr(repr)?;
            records.push(record)
        }
        Ok(records)
    }

    pub async fn rm(&self, key: &str) -> Result<(), Error> {
        let mut inner = self.memtable.lock().await;
        inner.rm(key);
        Ok(())
    }
}
