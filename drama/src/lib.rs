mod executor;
mod utilization_sketch;

pub use executor::Executor;

use utilization_sketch::UtilizationSketch;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TenantId {
    pub(crate) id: u64,
}

impl TenantId {
    pub fn new<T: std::hash::Hash>(value: T) -> TenantId {
        use std::hash::{DefaultHasher, Hasher};

        let mut hasher = DefaultHasher::new();

        value.hash(&mut hasher);

        TenantId {
            id: hasher.finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Classification {
    Accept,
    Read,
    Compute,
    Write,
}

const CARGO_PKG: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    ':',
    std::env!("CARGO_PKG_VERSION"),
);
