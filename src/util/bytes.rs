use std::ops::Deref;

#[derive(Default, Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Bytes {
    pub(crate) inner: Box<[u8]>,
}

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self {
        Bytes {
            inner: data.into_boxed_slice(),
        }
    }

    pub fn from_slice(data: &[u8]) -> Self {
        Bytes {
            inner: data.to_vec().into_boxed_slice(),
        }
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<&[u8]> for Bytes {
    fn from(data: &[u8]) -> Self {
        Bytes::from_slice(data)
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Bytes::new(data)
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

