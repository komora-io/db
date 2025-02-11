use std::io;

use crate::{Tx, WriteBatch};

pub struct Db {}

impl Db {
    pub fn tx(&self) -> Tx<'_> {
        todo!()
    }

    pub(crate) fn commit(&self, batch: WriteBatch) -> io::Result<()> {
        todo!()
    }
}
