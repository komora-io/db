use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use terrors::OneOf;

use super::{Error, FileAlreadyExists, FileDoesNotExist, Fs, Unavailable, UnexpectedEof};

#[derive(Clone)]
pub struct MemFs {
    files: Arc<Mutex<BTreeMap<PathBuf, Box<[u8]>>>>,
}

impl Fs for MemFs {
    fn read_at_exact(
        &self,
        path: &Path,
        at: usize,
        buf: &mut [u8],
    ) -> Result<(), Error<(FileDoesNotExist, UnexpectedEof, Unavailable)>> {
        let files = self.files.lock().unwrap();

        let Some(file) = files.get(path) else {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: OneOf::new(FileDoesNotExist),
            });
        };

        if file.len() < buf.len() + at {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: OneOf::new(UnexpectedEof),
            });
        }

        buf.copy_from_slice(&file[at..at + buf.len()]);

        Ok(())
    }

    fn create_unique(
        &self,
        path: &Path,
        buf: &[u8],
    ) -> Result<(), Error<(FileAlreadyExists, Unavailable)>> {
        let mut files = self.files.lock().unwrap();

        if files.contains_key(path) {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: OneOf::new(FileAlreadyExists),
            });
        }

        let pb = PathBuf::from(path);

        files.insert(pb, buf.into());

        Ok(())
    }

    fn delete(&self, path: &Path) -> Result<(), Error<(FileDoesNotExist, Unavailable)>> {
        todo!()
    }
}
