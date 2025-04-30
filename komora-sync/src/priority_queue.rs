use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};

pub struct PriorityQueue<T> {
    q: Mutex<BinaryHeap<Prioritized<T>>>,
    cv: Condvar,
}

struct Prioritized<T> {
    priority: u64,
    t: T,
}

impl<T> Ord for Prioritized<T> {
    fn cmp(&self, rhs: &Prioritized<T>) -> Ordering {
        self.priority.cmp(&rhs.priority)
    }
}

impl<T> PartialOrd for Prioritized<T> {
    fn partial_cmp(&self, rhs: &Prioritized<T>) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<T> PartialEq for Prioritized<T> {
    fn eq(&self, rhs: &Prioritized<T>) -> bool {
        self.priority == rhs.priority
    }
}

impl<T> Eq for Prioritized<T> {}

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
