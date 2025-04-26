use std::ops::{Bound, RangeBounds};

use crate::util::{Bytes, WriteBatch};
use crate::CollectionId;

pub enum InterestFilter {
    Key(Bytes),
    Range {
        start: Bound<Bytes>,
        end: Bound<Bytes>,
    },
}

impl InterestFilter {
    fn matches(&self, input: &WriteBatch) -> bool {
        match self {
            InterestFilter::Key(match_key) => input.keys().any(|k| k == match_key),
            InterestFilter::Range { start, end } => input
                .keys()
                .any(|k| (start.clone(), end.clone()).contains(k)),
        }
    }
}

pub struct Transactor {
    collection_id: CollectionId,
    filter: InterestFilter,
    transformer: Box<dyn Fn(WriteBatch) -> WriteBatch>,
}

impl Transactor {
    pub fn apply(&self, input: WriteBatch) -> WriteBatch {
        if self.filter.matches(&input) {
            (self.transformer)(input)
        } else {
            input
        }
    }
}
