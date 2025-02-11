use std::io;

use crate::{Db, Range, WriteBatch};

pub struct Tx<'a> {
    db: &'a Db,
    write_batch: WriteBatch,
}

impl<'a> Tx<'a> {
    pub fn get(&'_ mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        todo!()
    }

    pub fn insert(&'_ mut self, key: &[u8], value: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn remove(&'_ mut self, key: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }

    pub fn range<R>(&'_ mut self, range: R) -> Range {
        todo!()
    }

    pub fn commit(self) -> io::Result<()> {
        self.db.commit(self.write_batch)
    }
}
