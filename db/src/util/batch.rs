use std::io;

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use super::{read_frame, write_frame};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, KnownLayout, Immutable, IntoBytes, Unaligned,
)]
#[repr(C)]
struct BatchHeader {
    sub_frames: [u8; 8],
}

pub fn write_batch<B, Buf, W>(batch: B, w: W) -> io::Result<()>
where
    W: io::Write,
    B: ExactSizeIterator<Item = Buf>,
    Buf: AsRef<[u8]>,
{
    let mut out = vec![];

    let header = BatchHeader {
        sub_frames: (batch.len() as u64).to_le_bytes(),
    };

    out.extend_from_slice(header.as_bytes());

    for buf in batch {
        let buf = buf.as_ref();
        out.extend_from_slice(&(buf.len() as u64).to_le_bytes());
        out.extend_from_slice(&buf);
    }

    write_frame(&out, w)
}

pub fn read_batch<R: io::Read>(r: R) -> io::Result<Vec<Vec<u8>>> {
    let frame = read_frame(r)?;

    let (header, mut rest) = BatchHeader::ref_from_prefix(&frame).unwrap();

    let sub_frames: usize = u64::from_le_bytes(header.sub_frames).try_into().unwrap();

    let mut ret = Vec::with_capacity(sub_frames);

    for _ in 0..sub_frames {
        let length: usize = u64::from_le_bytes(rest[..8].try_into().unwrap())
            .try_into()
            .unwrap();

        rest = &rest[8..];

        let buf = rest[..length].to_vec();

        ret.push(buf);

        rest = &rest[length..];
    }

    Ok(ret)
}

#[test]
fn smoke_batch() {
    use rand::{thread_rng, Rng};

    const N: u64 = 128;

    let mut rng = thread_rng();
    let mut expected_batches = vec![];
    let mut frame_buf = vec![];

    for _ in 0..N {
        let batch_len = rng.gen_range(0..111);
        let mut batch = vec![];

        for _ in 0..batch_len {
            let len = rng.gen_range(0..111);
            let mut buf = vec![0_u8; len];
            rng.fill(&mut buf[..]);
            batch.push(buf);
        }

        write_batch(batch.iter(), &mut frame_buf).unwrap();
        expected_batches.push(batch);
    }

    expected_batches.reverse();

    let mut read_slice = &frame_buf[..];

    for _ in 0..N {
        let next_batch = read_batch(&mut read_slice).unwrap();
        let expected_next = expected_batches.pop().unwrap();
        assert_eq!(next_batch, expected_next);
    }

    assert!(read_frame(&mut read_slice).is_err());
}
