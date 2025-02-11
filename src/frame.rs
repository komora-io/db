use std::io;
use std::panic::Location;

use crc32fast::hash;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, KnownLayout, Immutable, IntoBytes, Unaligned,
)]
#[repr(C)]
struct FrameHeader {
    body_len: [u8; 8],
    body_len_crc: [u8; 4],
    body_crc: [u8; 4],
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub struct CrcMismatch {
    at: &'static str,
    caller: &'static Location<'static>,
    msg: &'static str,
    expected_crc: u32,
    actual_crc: u32,
}

impl std::fmt::Display for CrcMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CrcMismatch {}

fn header_for_buf(buf: &[u8]) -> FrameHeader {
    let body_len: [u8; 8] = (buf.len() as u64).to_le_bytes();
    let body_len_crc: [u8; 4] = hash(&body_len).to_le_bytes();
    let body_crc: [u8; 4] = hash(&buf).to_le_bytes();

    FrameHeader {
        body_len,
        body_len_crc,
        body_crc,
    }
}

pub fn write_frame<W: io::Write>(buf: &[u8], mut w: W) -> io::Result<()> {
    let header = header_for_buf(buf);
    w.write_all(header.as_bytes())?;
    w.write_all(buf)
}

#[track_caller]
pub fn read_frame<R: io::Read>(mut r: R) -> io::Result<Vec<u8>> {
    let mut header_bytes = [0_u8; std::mem::size_of::<FrameHeader>()];

    r.read_exact(&mut header_bytes)?;

    let header: &FrameHeader = FrameHeader::ref_from_bytes(&header_bytes[..]).unwrap();

    let actual_len_crc = hash(&header.body_len).to_le_bytes();

    if actual_len_crc != header.body_len_crc {
        let caller = Location::caller();

        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            CrcMismatch {
                actual_crc: u32::from_le_bytes(actual_len_crc),
                expected_crc: u32::from_le_bytes(header.body_len_crc),
                at: concat!(file!(), ':', line!()),
                caller: caller,
                msg: "failed crc in frame length",
            },
        ));
    }

    let buf_len: usize = u64::from_le_bytes(header.body_len).try_into().unwrap();
    let mut buf = Vec::with_capacity(buf_len);

    unsafe {
        buf.set_len(buf_len);
    }

    r.read_exact(&mut buf)?;

    let actual_body_crc = hash(&buf).to_le_bytes();

    if actual_body_crc != header.body_crc {
        let caller = Location::caller();

        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            CrcMismatch {
                actual_crc: u32::from_le_bytes(actual_len_crc),
                expected_crc: u32::from_le_bytes(header.body_len_crc),
                at: concat!(file!(), ':', line!()),
                caller: caller,
                msg: "failed crc in body of frame",
            },
        ));
    }

    Ok(buf)
}

#[test]
fn smoke_frame() {
    use rand::{thread_rng, Rng};

    const N: usize = 128;

    let mut rng = thread_rng();

    let mut expected_frames = vec![];
    let mut frame_buf = vec![];

    for _ in 0..N {
        let len = rng.gen_range(0..111);
        let mut buf = vec![0_u8; len];
        rng.fill(&mut buf[..]);

        write_frame(&buf, &mut frame_buf).unwrap();
        expected_frames.push(buf);
    }

    expected_frames.reverse();

    let mut read_slice = &frame_buf[..];

    for _ in 0..N {
        let next = read_frame(&mut read_slice).unwrap();
        let expected_next = expected_frames.pop().unwrap();
        assert_eq!(next, expected_next);
    }

    assert!(read_frame(&mut read_slice).is_err());
}
