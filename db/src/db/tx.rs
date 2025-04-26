use std::io;

use super::{Db, Range};
use crate::{Bytes, WriteBatch};

pub struct Tx<'a> {
    pub(crate) db: &'a Db,
    pub(crate) write_batch: WriteBatch,
}

impl<'a> Tx<'a> {
    pub fn get(&'_ mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        // First check if the key exists in the write batch
        let key_bytes = Bytes::from(key);
        if let Some(value) = self.write_batch.get(&key_bytes) {
            return Ok(Some(value.inner.to_vec()));
        }

        // Otherwise check in the storage
        let storage = self.db.storage.lock().unwrap();
        if let Some(value) = storage.get(&key_bytes) {
            return Ok(Some(value.inner.to_vec()));
        }

        // Key not found
        Ok(None)
    }

    pub fn insert(&'_ mut self, key: &[u8], value: &[u8]) -> Option<Vec<u8>> {
        let key_bytes = Bytes::from(key);
        let value_bytes = Bytes::from(value);

        // Check if the key already exists in storage or write batch
        let old_value = {
            if let Some(value) = self.write_batch.get(&key_bytes) {
                Some(value.inner.to_vec())
            } else {
                let storage = self.db.storage.lock().unwrap();
                storage.get(&key_bytes).map(|v| v.inner.to_vec())
            }
        };

        // Add to write batch
        self.write_batch.insert(key_bytes, value_bytes);

        old_value
    }

    pub fn remove(&'_ mut self, key: &[u8]) -> Option<Vec<u8>> {
        let key_bytes = Bytes::from(key);

        // Check if the key exists in storage or write batch
        let old_value = {
            if let Some(value) = self.write_batch.get(&key_bytes) {
                Some(value.inner.to_vec())
            } else {
                let storage = self.db.storage.lock().unwrap();
                storage.get(&key_bytes).map(|v| v.inner.to_vec())
            }
        };

        // Remove from storage in the next commit by setting to None
        // Note: In a real implementation you might handle this differently
        self.write_batch.remove(&key_bytes);

        old_value
    }

    pub fn range<R>(&'_ mut self, _range: R) -> Range {
        // For now, return an empty Range since we haven't implemented it yet
        Range {}
    }

    pub fn commit(self) -> io::Result<()> {
        self.db.commit(self.write_batch)
    }
}
