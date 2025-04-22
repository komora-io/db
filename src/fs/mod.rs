use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::sync::{oneshot, ReceiveOne, SendOne};
use crate::task::Executor;

#[derive(Clone, Copy, Debug)]
pub struct Error {
    at: &'static str,
    kind: ErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    FileAlreadyExists,
    FileDoesNotExist,
    UnexpectedEof,
    Unavailable,
}

pub trait Io {
    fn read_at_exact(&self, path: &Path, at: usize, buf: &mut [u8]) -> Result<(), Error>;
    fn create_unique(&self, path: &Path, buf: &[u8]) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct MemFs {
    files: Arc<Mutex<BTreeMap<PathBuf, Box<[u8]>>>>,
}

impl Io for MemFs {
    fn read_at_exact(&self, path: &Path, at: usize, buf: &mut [u8]) -> Result<(), Error> {
        let files = self.files.lock().unwrap();

        let Some(file) = files.get(path) else {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: ErrorKind::FileDoesNotExist,
            });
        };

        if file.len() < buf.len() + at {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: ErrorKind::UnexpectedEof,
            });
        }

        buf.copy_from_slice(&file[at..at + buf.len()]);

        Ok(())
    }

    fn create_unique(&self, path: &Path, buf: &[u8]) -> Result<(), Error> {
        let mut files = self.files.lock().unwrap();

        if files.contains_key(path) {
            return Err(Error {
                at: concat!(file!(), ':', line!()),
                kind: ErrorKind::FileAlreadyExists,
            });
        }

        let pb = PathBuf::from(path);

        files.insert(pb, buf.into());

        Ok(())
    }
}

enum IoWork {
    ReadAtExact {
        path: &'static Path,
        at: usize,
        buf: &'static mut [u8],
        ret_tx: SendOne<Result<(), Error>>,
    },
    CreateUnique {
        path: &'static Path,
        buf: &'static [u8],
        ret_tx: SendOne<Result<(), Error>>,
    },
    ShutDown,
}

pub struct AsyncIo {
    io: Arc<dyn Io + Send + Sync>,
    thread_pool: Executor,
}

impl AsyncIo {
    pub fn new(io: Arc<dyn Io + Send + Sync>, threads: usize) -> AsyncIo {
        AsyncIo {
            io,
            thread_pool: Executor::new(threads),
        }
    }

    async fn read_at_exact(&self, path: &Path, at: usize, buf: &mut [u8]) -> Result<(), Error> {
        let path: &'static Path = unsafe { std::mem::transmute(path) };
        let buf: &'static mut [u8] = unsafe { std::mem::transmute(buf) };
        let io = self.io.clone();

        self.thread_pool
            .spawn(move || io.read_at_exact(path, at, buf))
            .await
            .expect("io thread crashed")
    }

    async fn create_unique(&self, path: &Path, buf: &[u8]) -> Result<(), Error> {
        let path: &'static Path = unsafe { std::mem::transmute(path) };
        let buf: &'static [u8] = unsafe { std::mem::transmute(buf) };
        let io = self.io.clone();

        self.thread_pool
            .spawn(move || io.create_unique(path, buf))
            .await
            .expect("io thread crashed")
    }
}
