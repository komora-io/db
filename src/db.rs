use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

use crate::{Bytes, Transactor, Tx, WriteBatch};

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
