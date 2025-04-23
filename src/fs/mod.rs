mod async_fs;
mod local_fs;
mod mem_fs;

pub(crate) use async_fs::AsyncFs;

use std::path::Path;

use terrors::{OneOf, TypeSet};

pub trait Fs {
    fn read_at_exact(
        &self,
        path: &Path,
        at: usize,
        buf: &mut [u8],
    ) -> Result<(), Error<(FileDoesNotExist, UnexpectedEof, Unavailable)>>;

    fn create_unique(
        &self,
        path: &Path,
        buf: &[u8],
    ) -> Result<(), Error<(FileAlreadyExists, Unavailable)>>;

    fn delete(&self, path: &Path) -> Result<(), Error<(FileDoesNotExist, Unavailable)>>;
}

#[derive(Debug, Clone)]
pub struct Error<T>
where
    T: TypeSet,
    OneOf<T>: std::fmt::Debug + Clone,
{
    at: &'static str,
    kind: OneOf<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct FileAlreadyExists;

#[derive(Clone, Copy, Debug)]
pub struct FileDoesNotExist;

#[derive(Clone, Copy, Debug)]
pub struct UnexpectedEof;

#[derive(Clone, Copy, Debug)]
pub struct Unavailable;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix::{read_exact_at, write_all_at};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::{read_exact_at, write_all_at};
