use std::collections::HashMap;
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{Builder, JoinHandle};

use crate::sync::Mpmc;
use crate::sync::{oneshot, ReceiveOne};

type PinBoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct Executor {
    workers: Vec<JoinHandle<()>>,
    worker_state: Arc<WorkerState>,
}

enum Work {
    ShutDown,
    Work(Box<dyn FnOnce() + Send>),
    Register(PinBoxFuture),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FutureId {
    id: u64,
}

struct WorkerState {
    mpmc: Mpmc<Work>,
    future_counter: AtomicU64,
    future_registry: Mutex<HashMap<FutureId, (PinBoxFuture, Waker)>>,
}

struct ExecutorWaker {
    worker_state: Arc<WorkerState>,
    future_id: FutureId,
}

impl Wake for ExecutorWaker {
    fn wake(self: Arc<Self>) {
        let worker_state = self.worker_state.clone();
        let future_id = self.future_id;

        self.worker_state
            .mpmc
            .send(Work::Work(Box::new(move || worker_state.poll(future_id))));
    }
}

impl WorkerState {
    fn run(self: Arc<Self>) {
        loop {
            match self.mpmc.recv() {
                Work::Work(work) => (work)(),
                Work::Register(future) => self.register(future),
                Work::ShutDown => return,
            }
        }
    }

    fn register(self: &Arc<Self>, mut future: PinBoxFuture) {
        let future_id = FutureId {
            id: self.future_counter.fetch_add(1, Ordering::Relaxed),
        };

        let executor_waker = Arc::new(ExecutorWaker {
            future_id,
            worker_state: self.clone(),
        })
        .into();

        let mut cx = Context::from_waker(&executor_waker);

        if let Poll::Ready(()) = Future::poll(future.as_mut(), &mut cx) {
            return;
        }

        let mut future_registry = self.future_registry.lock().unwrap();

        future_registry.insert(future_id, (future, executor_waker));

        drop(future_registry);

        self.poll(future_id);
    }

    fn poll(self: &Arc<Self>, future_id: FutureId) {
        let (mut future, waker) = {
            let mut future_registry = self.future_registry.lock().unwrap();

            let Some((future, waker)) = future_registry.remove(&future_id) else {
                return;
            };

            (future, waker)
        };

        let mut cx = Context::from_waker(&waker);

        if let Poll::Ready(()) = Future::poll(future.as_mut(), &mut cx) {
            return;
        }

        let mut future_registry = self.future_registry.lock().unwrap();

        future_registry.insert(future_id, (future, waker));
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.worker_state.mpmc.send(Work::ShutDown);
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

        let worker_state = Arc::new(WorkerState {
            mpmc: Mpmc::new(),
            future_counter: AtomicU64::new(0),
            future_registry: Mutex::new(HashMap::new()),
        });

        for i in 0..number_of_workers {
            let worker = worker_state.clone();

            let join_handle = Builder::new()
                .name(format!("{}-worker-thread-{i}", crate::CARGO_PKG))
                .spawn(move || worker.run())
                .unwrap();

            workers.push(join_handle);
        }

        Executor {
            workers,
            worker_state,
        }
    }

    pub fn spawn<F, R>(&self, f: F) -> ReceiveOne<R>
    where
        F: 'static + FnOnce() -> R + Send,
        R: 'static + Send,
    {
        let (tx, rx) = oneshot();
        self.worker_state.mpmc.send(Work::Work(Box::new(move || {
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

        self.worker_state.mpmc.send(Work::Register(pin_box));

        rx
    }
}
