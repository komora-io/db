use std::io;

use crate::{Transactor, Tx, WriteBatch};

pub struct Db {
    transactors: Vec<Transactor>,
}

impl Db {
    pub fn tx(&self) -> Tx<'_> {
        todo!()
    }

    pub(crate) fn commit(&self, batch: WriteBatch) -> io::Result<()> {
        todo!()
    }
}
