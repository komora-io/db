use std::ops::Deref;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Default, Clone)]
pub struct Bytes {
    inner: Vec<u8>,
}

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.inner
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(inner: Vec<u8>) -> Bytes {
        Bytes { inner }
    }
}
