use crate::line::Line;
use std::io::{Result, Write};

pub trait Visual {
    fn step(&self) -> usize;
    fn total(&self) -> usize;
    fn dots(&mut self) -> &str;
}

/// Dummy implementation for [Visual].
impl Visual for () {
    fn step(&self) -> usize {
        0
    }

    fn total(&self) -> usize {
        0
    }

    fn dots(&mut self) -> &str {
        ""
    }
}

enum Inner<W, V> {
    WithoutVisual { writer: W },
    WithVisual { line: Line<W>, visual: V },
}

pub struct Progress<W, V> {
    buf: Vec<u8>,
    inner: Inner<W, V>,
}

impl<W, V> Progress<W, V>
where
    W: Write,
    V: Visual,
{
    pub fn without_visual(writer: W) -> Self {
        Self {
            buf: Vec::new(),
            inner: Inner::WithoutVisual { writer },
        }
    }

    pub fn with_visual(writer: W, visual: V) -> Self {
        Self {
            buf: Vec::new(),
            inner: Inner::WithVisual {
                line: Line::new(writer),
                visual,
            },
        }
    }

    pub fn print(&mut self) -> Result<()> {
        match &mut self.inner {
            Inner::WithoutVisual { .. } => {}
            Inner::WithVisual { line, visual } => {
                line.write(visual.dots().as_bytes())?;
                write!(line, " ({}/{})", visual.step(), visual.total())?;

                if !self.buf.is_empty() {
                    write!(line, " ")?;
                    line.write(&self.buf)?;
                    self.buf.clear();
                }

                line.flush()?;
            }
        }

        Ok(())
    }

    pub fn ok(&mut self) -> Result<()> {
        match &mut self.inner {
            Inner::WithoutVisual { writer } => {
                write!(writer, "  OK: ")?;
                writer.write(&self.buf)?;
                writeln!(writer)?;
                writer.flush()?;
                self.buf.clear();
            }
            Inner::WithVisual { line, visual } => {
                line.write(visual.dots().as_bytes())?;
                write!(line, " ({}/{})", visual.step(), visual.total())?;

                if !self.buf.is_empty() {
                    write!(line, " ")?;
                    line.write(&self.buf)?;
                    self.buf.clear();
                }

                line.flush()?;
            }
        }

        Ok(())
    }

    pub fn fail(&mut self) -> Result<()> {
        match &mut self.inner {
            Inner::WithoutVisual { writer } => {
                write!(writer, "FAIL: ")?;
                writer.write(&self.buf)?;
                writeln!(writer)?;
                writer.flush()?;
                self.buf.clear();
            }
            Inner::WithVisual { line, visual } => {
                line.write(visual.dots().as_bytes())?;
                write!(line, " ({}/{}) ", visual.step(), visual.total())?;

                if !self.buf.is_empty() {
                    write!(line, " ")?;
                    line.write(&self.buf)?;
                    self.buf.clear();
                }

                line.flush()?;
            }
        }

        Ok(())
    }
}

impl<W, V> Write for Progress<W, V>
where
    W: Write,
{
    fn write(&mut self, b: &[u8]) -> Result<usize> {
        self.buf.extend(b.iter().copied());
        Ok(b.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
