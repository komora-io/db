use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::mem;

use fst::{Map, MapBuilder};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

#[derive(Clone)]
pub struct Block<V> {
    buf: Vec<u8>,
    _pd: PhantomData<V>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, KnownLayout, Immutable, IntoBytes, Unaligned,
)]
#[repr(C)]
struct Header {
    value_start_offset: [u8; 8],
}

impl Header {
    fn value_start_offset(&self) -> usize {
        usize::try_from(u64::from_le_bytes(self.value_start_offset)).unwrap()
    }
}

/// Build an Block from a sorted map of byte-viewable keys and values.
///
/// Serializes the map as:
/// [Header, Fst from key to offset, array of values]
impl<K, V> From<&BTreeMap<K, V>> for Block<V>
where
    K: AsRef<[u8]>,
    V: Immutable + IntoBytes + KnownLayout + FromBytes + Unaligned,
{
    fn from(map: &BTreeMap<K, V>) -> Block<V> {
        let mut buf = vec![0; mem::size_of::<Header>()];
        let mut map_builder = MapBuilder::new(&mut buf).unwrap();
        let mut value_array: Vec<u8> = vec![];

        for (i, (k, v)) in map.iter().enumerate() {
            let value_bytes: &[u8] = v.as_bytes();
            value_array.extend_from_slice(value_bytes);
            map_builder.insert(k.as_ref(), i as u64).unwrap();
        }

        map_builder.finish().unwrap();

        let value_start_offset = (buf.len() as u64).to_le_bytes();

        let header = Header { value_start_offset };
        buf[0..mem::size_of::<Header>()].copy_from_slice(header.as_bytes());

        // take the bytes from value_array and append them to
        // the end of the key buffer.
        buf.append(&mut value_array);

        Block {
            buf: buf,
            _pd: PhantomData,
        }
    }
}

impl<V> Block<V>
where
    V: Immutable + IntoBytes + KnownLayout + FromBytes + Unaligned,
{
    fn header(&self) -> &Header {
        let header_len = size_of::<Header>();

        let header: &Header = Header::ref_from_bytes(&self.buf[..header_len]).unwrap();

        header
    }

    fn map(&self) -> Map<&[u8]> {
        let header_len = size_of::<Header>();
        let header = self.header();
        let map = Map::new(&self.buf[header_len..header.value_start_offset()]).unwrap();
        map
    }

    fn value_array(&self) -> &[u8] {
        let header = self.header();
        &self.buf[header.value_start_offset()..]
    }

    pub fn get<K>(&self, key: K) -> Option<&V>
    where
        K: AsRef<[u8]>,
    {
        let map = self.map();

        // early-exit if our key map doesn't contain what we're looking for
        let value_index_u64: u64 = map.get(key.as_ref())?;
        let value_index: usize = value_index_u64.try_into().unwrap();

        let value_start = value_index * mem::size_of::<V>();
        let value_end = (value_index + 1) * mem::size_of::<V>();

        let value_bytes = &self.value_array()[value_start..value_end];

        let value = V::ref_from_bytes(value_bytes).unwrap();

        Some(value)
    }
}

#[test]
fn smoke_fst_map() {
    use rand::{rng, Rng};

    const N_TESTS: usize = 1024;

    const TEST_SIZE: usize = 1024;

    let mut rng = rng();

    let before = std::time::Instant::now();

    for _ in 0..N_TESTS {
        let model: BTreeMap<Vec<u8>, Header> = (0..TEST_SIZE)
            .map(|_| {
                let k: u64 = rng.random();
                let k_buf = k.to_le_bytes();

                (
                    k_buf.to_vec(),
                    Header {
                        value_start_offset: k.to_le_bytes(),
                    },
                )
            })
            .collect();

        let map = Block::from(&model);

        for (k_a, v_a) in &model {
            let v_b = map.get(k_a).unwrap();
            assert_eq!(v_a, v_b);
        }
    }

    let wps = (N_TESTS * TEST_SIZE) as u128 * 1000 / before.elapsed().as_millis();
    dbg!(wps);
}
