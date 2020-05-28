use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    r: R,
    read_count: usize,
    total_bytes: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            r: wrapped,
            read_count: 0,
            total_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.r
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.r.read(buf) {
            Ok(size) => {
                self.total_bytes += size;
                self.read_count += 1;
                Ok(size)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    w: W,
    write_count: usize,
    total_bytes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            w: wrapped,
            write_count: 0,
            total_bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.w
    }

    pub fn bytes_through(&self) -> usize {
        self.total_bytes
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.w.write(buf) {
            Ok(size) => {
                self.total_bytes += size;
                self.write_count += 1;
                Ok(size)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}
