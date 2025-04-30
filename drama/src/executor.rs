use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{Builder, JoinHandle};

use komora_sync::{oneshot, PriorityQueue, ReceiveOne};

use crate::{Classification, TenantId};

type PinBoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct Executor {
    workers: Vec<JoinHandle<()>>,
    worker_state: Arc<WorkerState>,
}

enum Work {
    ShutDown,
    Work(Box<dyn FnOnce() + Send>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct FutureId {
    id: u64,
}

struct WorkerState {
    pq: PriorityQueue<Work>,
    future_counter: AtomicU64,
}

struct TaskState {
    pin_box_future: PinBoxFuture,
    waker: Option<Weak<ExecutorWaker>>,
}

impl TaskState {
    fn poll(&mut self) {
        let weak_waker = self.waker.as_ref().expect("waker not set");
        let executor_waker: Arc<ExecutorWaker> =
            weak_waker.upgrade().expect("waker weak upgrade failed");
        let waker: Waker = Waker::from(executor_waker);

        let mut cx = Context::from_waker(&waker);

        if let Poll::Ready(()) = Future::poll(self.pin_box_future.as_mut(), &mut cx) {
            return;
        }
    }
}

struct ExecutorWaker {
    worker_state: Arc<WorkerState>,
    task_state: Arc<Mutex<TaskState>>,
    future_id: FutureId,
    tenant_id: TenantId,
    classification: Classification,
}

impl Wake for ExecutorWaker {
    fn wake(self: Arc<Self>) {
        let task_state = self.task_state.clone();
        let priority = self
            .worker_state
            .priority_for_tenant_id_and_classification(self.tenant_id, self.classification);

        self.worker_state.pq.push(
            Work::Work(Box::new(move || task_state.lock().unwrap().poll())),
            priority,
        );
    }
}

impl WorkerState {
    fn priority_for_tenant_id_and_classification(
        &self,
        tenant_id: TenantId,
        classification: Classification,
    ) -> u64 {
        // TODO: proper prioritization
        0
    }

    fn run(self: Arc<Self>) {
        loop {
            match self.pq.pop() {
                Work::Work(work) => (work)(),
                Work::ShutDown => return,
            }
        }
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.worker_state.pq.push(Work::ShutDown, u64::MIN);
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
            pq: PriorityQueue::new(),
            future_counter: AtomicU64::new(0),
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

    /// Spawn a plain closure on the [`Executor`].
    pub fn spawn<F, R>(
        &self,
        tenant_id: TenantId,
        classification: Classification,
        f: F,
    ) -> ReceiveOne<R>
    where
        F: 'static + FnOnce() -> R + Send,
        R: 'static + Send,
    {
        let (tx, rx) = oneshot();
        let priority = self
            .worker_state
            .priority_for_tenant_id_and_classification(tenant_id, classification);
        self.worker_state.pq.push(
            Work::Work(Box::new(move || {
                let ret: R = (f)();
                tx.send(ret);
            })),
            priority,
        );
        rx
    }

    /// Spawn a [`Future`] on the [`Executor`].
    pub fn execute<F, R>(
        &self,
        tenant_id: TenantId,
        classification: Classification,
        f: F,
    ) -> ReceiveOne<R>
    where
        F: 'static + Future<Output = R> + Send,
        R: 'static + Send,
    {
        let future_id = FutureId {
            id: self
                .worker_state
                .future_counter
                .fetch_add(1, Ordering::Relaxed),
        };

        let (tx, rx) = oneshot();

        let pin_box_future = Box::pin(async move {
            let output = f.await;
            tx.send(output);
        });

        let executor_waker: Arc<ExecutorWaker> = Arc::new(ExecutorWaker {
            future_id,
            tenant_id,
            classification,
            task_state: Arc::new(Mutex::new(TaskState {
                pin_box_future: pin_box_future,
                waker: None,
            })),
            worker_state: self.worker_state.clone(),
        })
        .into();

        let weak_waker = Arc::downgrade(&executor_waker.clone());

        let mut task_state = executor_waker.task_state.lock().unwrap();

        task_state.waker = Some(weak_waker);

        task_state.poll();

        rx
    }
}
