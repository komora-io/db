use std::path::{Path, PathBuf};

use super::{
    read_exact_at, write_all_at, Error, FileAlreadyExists, FileDoesNotExist, Fs, Unavailable,
    UnexpectedEof,
};

use terrors::OneOf;

pub struct LocalFs {}

impl Fs for LocalFs {
    fn read_at_exact(
        &self,
        path: &Path,
        at: usize,
        buf: &mut [u8],
    ) -> Result<(), Error<(FileDoesNotExist, UnexpectedEof, Unavailable)>> {
        todo!()
    }

    fn create_unique(
        &self,
        path: &Path,
        buf: &[u8],
    ) -> Result<(), Error<(FileAlreadyExists, Unavailable)>> {
        todo!()
    }

    fn delete(&self, path: &Path) -> Result<(), Error<(FileDoesNotExist, Unavailable)>> {
        todo!()
    }
}
