pub(crate) struct VirtualStorageAddress {
    pub lsn: Lsn,
}

pub(crate) struct CollectionId {
    pub value: std::num::NonZeroU64,
}

pub(crate) struct Lsn {
    pub value: std::num::NonZeroU64,
}

pub(crate) struct HeapAddress {
    pub value: std::num::NonZeroU64,
}

