mod executor;

pub use executor::Executor;

const CARGO_PKG: &str = concat!(
    std::env!("CARGO_PKG_NAME"),
    ':',
    std::env!("CARGO_PKG_VERSION"),
);
