use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Condvar, Mutex};
use std::task::Waker;
use std::task::{Context, Poll};

pub fn oneshot<T>() -> (SendOne<T>, ReceiveOne<T>) {
    let shared: Arc<Shared<T>> = Arc::default();

    (
        SendOne {
            shared: shared.clone(),
            sent: false,
        },
        ReceiveOne { shared },
    )
}

struct State<T> {
    value: Option<Result<T, SenderDropped>>,
    waker: Option<Waker>,
}

impl<T> Default for State<T> {
    fn default() -> State<T> {
        State {
            value: None,
            waker: None,
        }
    }
}

pub struct Shared<T> {
    state: Mutex<State<T>>,
    cv: Condvar,
}

impl<T> Default for Shared<T> {
    fn default() -> Shared<T> {
        Shared {
            state: Mutex::default(),
            cv: Condvar::new(),
        }
    }
}

pub struct SendOne<T> {
    shared: Arc<Shared<T>>,
    sent: bool,
}

pub struct ReceiveOne<T> {
    shared: Arc<Shared<T>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SenderDropped;

impl<T> SendOne<T> {
    pub fn send(mut self, t: T) {
        self.send_inner(Ok(t));
        self.sent = true;
    }

    fn send_inner(&mut self, t_res: Result<T, SenderDropped>) {
        let mut state = self.shared.state.lock().unwrap();

        assert!(state.value.is_none());

        state.value = Some(t_res);

        let waker_opt = state.waker.clone();

        drop(state);

        self.shared.cv.notify_one();

        if let Some(waker) = waker_opt {
            waker.wake();
        }
    }
}

impl<T> Drop for SendOne<T> {
    fn drop(&mut self) {
        if !self.sent {
            self.send_inner(Err(SenderDropped));
        }
    }
}

impl<T> ReceiveOne<T> {
    pub fn recv(self) -> Result<T, SenderDropped> {
        let mut state = self.shared.state.lock().unwrap();

        while state.value.is_none() {
            state = self.shared.cv.wait(state).unwrap();
        }

        state.value.take().unwrap()
    }
}

impl<T> Future for ReceiveOne<T> {
    type Output = Result<T, SenderDropped>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.shared.state.lock().unwrap();

        let waker = cx.waker().clone();

        state.waker = Some(waker);

        if let Some(filled_value) = state.value.take() {
            Poll::Ready(filled_value)
        } else {
            Poll::Pending
        }
    }
}
