mod batch;
mod config;
mod db;
mod frame;
mod poly_index;
mod range;
mod tx;

pub use batch::{read_batch, write_batch};
pub use config::Config;
pub use db::Db;
pub use frame::{read_frame, write_frame};
pub use range::Range;
pub use tx::Tx;

pub struct VirtualStorageAddress {
    pub lsn: std::num::NonZeroU64,
}

pub type WriteBatch = std::collections::BTreeMap<Vec<u8>, Vec<u8>>;

pub fn open<P: AsRef<std::path::Path>>(_path: P) -> std::io::Result<Db> {
    todo!()
}
