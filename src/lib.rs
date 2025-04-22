mod config;
mod db;
mod fs;
mod sync;
mod task;
mod util;

pub use config::Config;
pub use db::{open, Db};
pub use util::{Bytes, WriteBatch};

const CARGO_PKG: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    ':',
    std::env!("CARGO_PKG_VERSION"),
);

use fs::AsyncFs;
use task::Executor;
