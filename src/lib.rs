mod batch;
mod config;
mod db;
mod frame;
mod poly_index;
mod range;
mod transactor;
mod tx;

pub use batch::{read_batch, write_batch};
pub use config::Config;
pub use db::Db;
pub use frame::{read_frame, write_frame};
pub use range::Range;
pub use transactor::{InterestFilter, Transactor};
pub use tx::Tx;

pub struct VirtualStorageAddress {
    pub lsn: std::num::NonZeroU64,
}

pub struct CollectionId {
    pub collection_id: std::num::NonZeroU64,
}

#[derive(Default, Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Bytes {
    inner: Box<[u8]>,
}

pub type WriteBatch = std::collections::BTreeMap<Bytes, Bytes>;

pub fn open<P: AsRef<std::path::Path>>(_path: P) -> std::io::Result<Db> {
    todo!()
}
