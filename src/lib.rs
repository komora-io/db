mod batch;
mod bytes;
mod config;
mod db;
mod frame;
mod poly_index;
mod range;
mod transactor;
mod tx;

pub use batch::{read_batch, write_batch};
pub use bytes::Bytes;
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

pub type WriteBatch = std::collections::BTreeMap<Bytes, Bytes>;

pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Db> {
    let config = Config {
        path: path.as_ref().to_path_buf(),
    };

    Ok(Db {
        transactors: Vec::new(),
        storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new())),
    })
}
