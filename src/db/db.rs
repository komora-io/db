use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

pub use super::{InterestFilter, Transactor, Tx};
pub use crate::util::{Bytes, WriteBatch};
use crate::Config;

pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Db> {
    let config = Config {
        path: path.as_ref().to_path_buf(),
    };

    Ok(Db {
        transactors: Vec::new(),
        storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new())),
    })
}

pub struct Db {
    pub(crate) transactors: Vec<Transactor>,
    pub(crate) storage: Arc<Mutex<BTreeMap<Bytes, Bytes>>>,
}

impl Db {
    pub fn tx(&self) -> Tx<'_> {
        Tx {
            db: self,
            write_batch: BTreeMap::new(),
        }
    }

    pub(crate) fn commit(&self, batch: WriteBatch) -> io::Result<()> {
        // Apply transformations from transactors
        let final_batch = self.apply_transactors(batch);

        // Apply the batch to storage
        let mut storage = self.storage.lock().unwrap();
        for (key, value) in final_batch {
            storage.insert(key, value);
        }

        Ok(())
    }

    fn apply_transactors(&self, mut batch: WriteBatch) -> WriteBatch {
        // Apply each transactor in sequence
        for transactor in &self.transactors {
            batch = transactor.apply(batch);
        }
        batch
    }
}
