use std::num::NonZeroU64;
use std::ops::Deref;

pub(crate) struct Lsn {
    pub(crate) value: NonZeroU64,
}

impl Deref for Lsn {
    type Target = NonZeroU64;

    fn deref(&self) -> &NonZeroU64 {
        &self.value
    }
}
