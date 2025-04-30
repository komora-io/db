use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};

use crate::Prioritized;

#[derive(Debug)]
pub struct PriorityQueue<T> {
    q: Mutex<BinaryHeap<Prioritized<T>>>,
    cv: Condvar,
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            q: Mutex::default(),
            cv: Condvar::new(),
        }
    }

    /// Higher priority gets popped first.
    ///
    /// # Examples
    /// ```
    /// let pq = komora_sync::PriorityQueue::new();
    /// pq.push(2, 2);
    /// pq.push(1, 1);
    /// pq.push(4, 4);
    /// pq.push(1, 1);
    /// assert_eq!(pq.pop(), 4);
    /// assert_eq!(pq.pop(), 2);
    /// assert_eq!(pq.pop(), 1);
    /// assert_eq!(pq.pop(), 1);
    /// ```
    pub fn push(&self, t: T, priority: u64) {
        let mut q = self.q.lock().unwrap();
        q.push(Prioritized { t, priority });
        drop(q);
        self.cv.notify_one();
    }

    pub fn pop(&self) -> T {
        let mut q = self.q.lock().unwrap();

        while q.is_empty() {
            q = self.cv.wait(q).unwrap();
        }

        q.pop().unwrap().t
    }
}
