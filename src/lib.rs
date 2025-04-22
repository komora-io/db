mod config;
mod db;
mod fs;
mod sync;
mod task;
mod util;

pub use crate::config::Config;
pub use crate::db::{open, Db};
pub use crate::util::{Bytes, WriteBatch};

const CARGO_PKG: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    ':',
    std::env!("CARGO_PKG_VERSION"),
);

use fs::AsyncFs;
use task::Executor;
