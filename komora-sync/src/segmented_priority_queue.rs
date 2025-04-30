use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};

use crate::Prioritized;

/// Like a [`PriorityQueue`] but avoids starvation by
/// using a rotating bip-buffer of priority queues.
#[derive(Debug)]
pub struct SegmentedPriorityQueue<T> {
    reader: Mutex<BinaryHeap<Prioritized<T>>>,
    writer: Mutex<BinaryHeap<Prioritized<T>>>,
    cv: Condvar,
}

impl<T> Default for SegmentedPriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SegmentedPriorityQueue<T> {
    pub fn new() -> SegmentedPriorityQueue<T> {
        SegmentedPriorityQueue {
            reader: Mutex::default(),
            writer: Mutex::default(),
            cv: Condvar::new(),
        }
    }

    pub fn push(&self, t: T, priority: u64) {
        let mut q = self.writer.lock().unwrap();
        q.push(Prioritized { t, priority });
        drop(q);
        self.cv.notify_one();
    }

    /// Higher priority tends to gets popped first.
    ///
    /// The internal bip buffer of priority queues gets
    /// rotated when the read side is empty.
    ///
    /// # Examples
    /// ```
    /// let pq = komora_sync::SegmentedPriorityQueue::new();
    /// pq.push(2, 2);
    /// pq.push(1, 1);
    ///
    /// // internal state:
    /// // read buffer: []
    /// // write buffer: [2, 1]
    ///
    /// // queue rotated on pop when read buffer is empty
    /// assert_eq!(pq.pop(), 2);
    ///
    /// // internal state:
    /// // read buffer: [1]
    /// // write buffer: []
    ///
    /// // new writes get pushed to write buffer
    /// pq.push(4, 4);
    /// pq.push(3, 3);
    ///
    /// // internal state:
    /// // read buffer: [1]
    /// // write buffer: [4, 3]
    ///
    /// // this is the last element in the reader side
    /// assert_eq!(pq.pop(), 1);
    ///
    /// // queue rotated on pop when reader is empty
    /// assert_eq!(pq.pop(), 4);
    /// assert_eq!(pq.pop(), 3);
    /// ```
    pub fn pop(&self) -> T {
        let mut q = self.reader.lock().unwrap();

        while q.is_empty() {
            // rotate the bip buffer
            let mut q2 = self.writer.lock().unwrap();

            std::mem::swap(&mut *q, &mut *q2);

            drop(q2);

            if q.is_empty() {
                q = self.cv.wait(q).unwrap();
            }
        }

        q.pop().unwrap().t
    }
}

#[test]
fn segmented_priority_queue_concurrent_pops() {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Barrier};

    const N_WRITES_PER_WRITER: u64 = 1_000_000;
    const N_WRITERS: u64 = 10;
    const N_WRITES: u64 = N_WRITERS * N_WRITES_PER_WRITER;
    const N_READERS: u64 = 10;

    let q = Arc::new(SegmentedPriorityQueue::new());
    let barrier = Arc::new(Barrier::new(N_WRITERS.try_into().unwrap()));
    let read = Arc::new(AtomicU64::new(0));

    for i in 0..N_WRITERS {
        let q = q.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();

            for j in 0..N_WRITES_PER_WRITER {
                q.push(i * j, i);
            }
        });
    }

    for _ in 0..N_READERS {
        let q = q.clone();
        let read = read.clone();
        std::thread::spawn(move || loop {
            q.pop();
            read.fetch_add(1, Ordering::SeqCst);
        });
    }

    while read.load(Ordering::SeqCst) < N_WRITES {
        std::hint::spin_loop();
    }
}
