use crate::fs::{AsyncFs, Error, Unavailable};

use super::{ObjectId, ObjectMetadata};

pub struct ObjectStore {
    fs: AsyncFs,
}

impl ObjectStore {
    pub async fn write_batch<I, B>(&self, objects: I) -> Result<(), Error<(Unavailable,)>>
    where
        I: IntoIterator<Item = (ObjectId, ObjectMetadata, B)>,
        B: AsRef<[u8]>,
    {
        todo!()
    }
}
