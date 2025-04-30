use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct Mpmc<T> {
    q: Mutex<VecDeque<T>>,
    cv: Condvar,
}

impl<T> Default for Mpmc<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Mpmc<T> {
    pub fn new() -> Mpmc<T> {
        Mpmc {
            q: Mutex::default(),
            cv: Condvar::new(),
        }
    }

    pub fn send(&self, t: T) {
        let mut q = self.q.lock().unwrap();
        q.push_back(t);
        drop(q);
        self.cv.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut q = self.q.lock().unwrap();

        while q.is_empty() {
            q = self.cv.wait(q).unwrap();
        }

        q.pop_front().unwrap()
    }
}
