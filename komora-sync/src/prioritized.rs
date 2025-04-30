use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(Debug)]
pub(crate) struct Prioritized<T> {
    pub priority: u64,
    pub t: T,
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
