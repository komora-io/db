mod config;
mod db;
mod fs;
mod num;
mod object_store;
mod sync;
mod task;
mod util;

pub use crate::config::Config;
pub use crate::db::{open, Db, InterestFilter};
pub use crate::util::{Bytes, WriteBatch};

const CARGO_PKG: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    ':',
    std::env!("CARGO_PKG_VERSION"),
);

use num::{CollectionId, Lsn, VirtualStorageAddress};
use task::Executor;
