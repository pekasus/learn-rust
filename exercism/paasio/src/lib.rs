use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    num_of_reads: usize,
    num_of_bytes: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> Self {
        Self {
            wrapped,
            num_of_bytes: 0,
            num_of_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_of_bytes
    }

    pub fn reads(&self) -> usize {
        self.num_of_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let num_of_bytes = self.wrapped.read(buf)?;
        self.num_of_reads += 1;
        self.num_of_bytes += num_of_bytes;
        Ok(num_of_bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    num_of_bytes: usize,
    num_of_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> Self {
        Self {
            wrapped,
            num_of_bytes: 0,
            num_of_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.num_of_bytes
    }

    pub fn writes(&self) -> usize {
        self.num_of_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // it worked with self.wrapped.write_all(buf), too. Then use buf.len().
        let written_length = self.wrapped.write(buf)?;
        self.num_of_writes += 1;
        self.num_of_bytes += written_length;
        Ok(written_length)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
