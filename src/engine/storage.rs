//! On disk real log records.

use super::log;
use bytes::{Bytes, BytesMut};
use failure::Error;
use std::sync::Arc;
use futures::lock::Mutex;
use tokio::{fs, prelude::*};

pub struct Storage {
    // TODO: replace to async mutex.
    file: Arc<Mutex<fs::File>>,
}

impl Storage {
    /// Asynchronously create a new wal file.
    pub async fn new() -> Self {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open("wal")
            .expect("couldn't create wal file");

        let file = fs::File::from_std(file);

        Storage {
            file: Arc::new(Mutex::new(file)),
        }
    }

    /// Simulate pread, but atomicity by mutex (which is bad).
    pub async fn pread(&self, offset: log::Offset, size: usize) -> Result<log::Repr, Error> {
        // Unfortunately there's no actual pread in tokio.
        // We've to first seek to the offset.

        let mut bytes = BytesMut::with_capacity(size);
        let mut guard_file = self.file.lock().await;

        guard_file.seek(std::io::SeekFrom::Start(offset)).await?;

        // NOTICE: there's a strange behavior if you use read or read_exact for reading.
        // Those methods can only work with fixed size byte array.
        guard_file.read_buf(&mut bytes).await?;

        let repr = log::Repr::new(bytes.freeze());

        Ok(repr)
    }

    /// Write to the end of file.
    pub async fn append(&self, data: Bytes) -> Result<log::Offset, Error> {
        // TODO: is there any ways to get the offset and write to end in one call?
        let mut guard_file = self.file.lock().await;

        let end = guard_file.seek(std::io::SeekFrom::End(0)).await?;

        guard_file.write(&data).await?;

        guard_file.sync_data().await?;

        Ok(end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_read_write() {
        let storage = Storage::new().await;
        let bytes = Bytes::from("Hello world");
        let size = bytes.len();
        assert_eq!(size, 11);

        let offset = storage.append(bytes).await.unwrap();
        assert_eq!(offset, 0);

        let repr = storage.pread(offset, size).await.unwrap();

        assert_eq!(repr.size, size);
        assert_eq!(repr.bs, Bytes::from("Hello world"));

        // Same for second time.
        let pre_size = size;
        let bytes = Bytes::from("Hello Jon");
        let size = bytes.len();
        let offset = storage.append(bytes).await.unwrap();

        // Offset should be previous size.
        assert_eq!(offset, pre_size as u64);

        let repr = storage.pread(offset, size).await.unwrap();

        assert_eq!(repr.size, size);
        assert_eq!(repr.bs, Bytes::from("Hello Jon"));
    }
}
