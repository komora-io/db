use crate::{Config, Db};

pub fn open<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<Db> {
    let config = Config {
        path: path.as_ref().to_path_buf(),
    };

    Ok(Db {
        config,
        transactors: Vec::new(),
        storage: std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new())),
    })
}
