use std::collections::BTreeMap;

use crate::VirtualStorageAddress;

type Bytes = Box<[u8]>;

type Block = BTreeMap<Bytes, Bytes>;

struct BlockKey {}

struct BlockMeta {
    high_key: Bytes,
    filter: Bytes,
    address: VirtualStorageAddress,
}

struct BlockIndex {
    blocks: BTreeMap<BlockKey, BlockMeta>,
}

struct PolyIndex {
    indexes: BTreeMap<u64, BlockIndex>,
}

#[test]
fn poly_index_smoke() {}
