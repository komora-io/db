use std::fs;
use std::io;
use std::os::unix::fs::FileExt;

use fault_injection::maybe;

pub(super) fn read_exact_at(file: &fs::File, buf: &mut [u8], offset: u64) -> io::Result<()> {
    match maybe!(file.read_exact_at(buf, offset)) {
        Ok(r) => Ok(r),
        Err(e) => {
            // FIXME BUG 3: failed to read 64 bytes at offset 192 from file with len 192
            println!(
                "failed to read {} bytes at offset {} from file with len {}",
                buf.len(),
                offset,
                file.metadata().unwrap().len(),
            );
            let _ = dbg!(std::backtrace::Backtrace::force_capture());
            Err(e)
        }
    }
}

pub(super) fn write_all_at(file: &fs::File, buf: &[u8], offset: u64) -> io::Result<()> {
    maybe!(file.write_all_at(buf, offset))
}
