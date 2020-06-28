use ansi_escapes::EraseLine;
use std::io::Write;

/// Helper struct to edit a single line that can be flushed.
pub struct Line<W> {
    inner: W,
}

impl<W> Line<W>
where
    W: Write,
{
    pub fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<W> Write for Line<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()?;
        write!(self.inner, "{}\r", EraseLine)
    }
}
