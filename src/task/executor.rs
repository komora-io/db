use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

use crate::sync::Mpmc;
use crate::sync::{oneshot, ReceiveOne};

pub struct Executor {
    workers: Vec<JoinHandle<()>>,
    mpmc: Arc<Mpmc<Work>>,
}

enum Work {
    ShutDown,
    Work(Box<dyn FnOnce() + Send>),
    Register(Pin<Box<dyn Future<Output = ()> + Send>>),
}

impl Drop for Executor {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.mpmc.send(Work::ShutDown);
        }
        for join_handle in std::mem::take(&mut self.workers) {
            join_handle.join().unwrap();
        }
    }
}

impl Default for Executor {
    fn default() -> Executor {
        let available_parallelism = std::thread::available_parallelism()
            .unwrap_or(NonZeroUsize::MIN)
            .get();

        Executor::new(available_parallelism)
    }
}

impl Executor {
    pub fn new(number_of_workers: usize) -> Executor {
        let mut workers = vec![];
        let mpmc = Arc::new(Mpmc::new());

        for i in 0..number_of_workers {
            let mpmc = mpmc.clone();

            let join_handle = Builder::new()
                .name(format!("gift-tree-worker-{i}"))
                .spawn(move || loop {
                    match mpmc.recv() {
                        Work::Work(work) => (work)(),
                        Work::Register(future) => {
                            todo!()
                        }
                        Work::ShutDown => return,
                    }
                })
                .unwrap();

            workers.push(join_handle);
        }

        Executor { workers, mpmc }
    }

    pub fn spawn<F, R>(&self, f: F) -> ReceiveOne<R>
    where
        F: 'static + FnOnce() -> R + Send,
        R: 'static + Send,
    {
        let (tx, rx) = oneshot();
        self.mpmc.send(Work::Work(Box::new(move || {
            let ret: R = (f)();
            tx.send(ret);
        })));
        rx
    }

    pub fn execute<F, R>(&self, f: F) -> ReceiveOne<R>
    where
        F: 'static + Future<Output = R> + Send,
        R: 'static + Send,
    {
        let (tx, rx) = oneshot();

        let pin_box = Box::pin(async move { tx.send(f.await) });

        self.mpmc.send(Work::Register(pin_box));

        rx
    }
}
