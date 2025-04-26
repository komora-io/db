use std::path::Path;
use std::sync::Arc;

use super::{Error, FileAlreadyExists, FileDoesNotExist, Fs, Unavailable, UnexpectedEof};
use crate::Executor;

pub(crate) struct AsyncFs {
    fs: Arc<dyn Fs + Send + Sync>,
    thread_pool: Executor,
}

impl AsyncFs {
    pub fn new(fs: Arc<dyn Fs + Send + Sync>, threads: usize) -> AsyncFs {
        AsyncFs {
            fs,
            thread_pool: Executor::new(threads),
        }
    }

    async fn read_at_exact(
        &self,
        path: &Path,
        at: usize,
        buf: &mut [u8],
    ) -> Result<(), Error<(FileDoesNotExist, UnexpectedEof, Unavailable)>> {
        let path: &'static Path = unsafe { std::mem::transmute(path) };
        let buf: &'static mut [u8] = unsafe { std::mem::transmute(buf) };
        let fs = self.fs.clone();

        self.thread_pool
            .spawn(move || fs.read_at_exact(path, at, buf))
            .await
            .expect("fs thread crashed")
    }

    async fn create_unique(
        &self,
        path: &Path,
        buf: &[u8],
    ) -> Result<(), Error<(FileAlreadyExists, Unavailable)>> {
        let path: &'static Path = unsafe { std::mem::transmute(path) };
        let buf: &'static [u8] = unsafe { std::mem::transmute(buf) };
        let fs = self.fs.clone();

        self.thread_pool
            .spawn(move || fs.create_unique(path, buf))
            .await
            .expect("fs thread crashed")
    }

    async fn delete(&self, path: &Path) -> Result<(), Error<(FileDoesNotExist, Unavailable)>> {
        let path: &'static Path = unsafe { std::mem::transmute(path) };
        let fs = self.fs.clone();

        self.thread_pool
            .spawn(move || fs.delete(path))
            .await
            .expect("fs thread crashed")
    }
}
