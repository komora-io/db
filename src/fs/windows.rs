use std::fs;
use std::os::windows::fs::FileExt;

pub(super) fn read_exact_at(
    file: &fs::File,
    mut buf: &mut [u8],
    mut offset: u64,
) -> io::Result<()> {
    while !buf.is_empty() {
        match maybe!(file.seek_read(buf, offset)) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
                offset += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(annotate!(e)),
        }
    }
    if !buf.is_empty() {
        Err(annotate!(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "failed to fill whole buffer"
        )))
    } else {
        Ok(())
    }
}

pub(super) fn write_all_at(file: &fs::File, mut buf: &[u8], mut offset: u64) -> io::Result<()> {
    while !buf.is_empty() {
        match maybe!(file.seek_write(buf, offset)) {
            Ok(0) => {
                return Err(annotate!(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                )));
            }
            Ok(n) => {
                buf = &buf[n..];
                offset += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(annotate!(e)),
        }
    }
    Ok(())
}
