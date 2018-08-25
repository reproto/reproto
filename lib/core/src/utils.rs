use errors::Result;
use std::io::Read;
use Span;

const NL: u8 = b'\n';
const CR: u8 = b'\r';

/// A position withing a source.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

/// Find the line corresponding to the given position.
pub fn find_line<'a, R: AsMut<Read + 'a>>(
    mut reader: R,
    span: (usize, usize),
) -> Result<(String, usize, (usize, usize))> {
    let r = reader.as_mut();

    let mut line = 0usize;
    let mut current = 0usize;
    let mut buffer: Vec<u8> = Vec::new();

    let start = span.0;
    let end = span.1;

    let mut read = 0usize;

    for b in r.bytes() {
        let b = b?;
        read += 1;

        match b {
            NL => {}
            _ => {
                buffer.push(b);
                continue;
            }
        }

        let start_of_line = current;
        current += read;

        if current > start {
            let buffer = String::from_utf8(buffer)?;
            let end = ::std::cmp::min(end, current);
            let range = (start - start_of_line, end - start_of_line);
            return Ok((buffer, line, range));
        }

        read = 0usize;
        line += 1;
        buffer.clear();
    }

    Err("bad file position".into())
}

/// Find the range corresponding to the given position.
pub fn find_range<'a, R: AsMut<Read + 'a>, S: Into<Span>>(
    mut reader: R,
    span: S,
    encoding: Encoding,
) -> Result<(Position, Position)> {
    let span = span.into();

    let r = reader.as_mut();

    let mut start = Position::default();
    let mut end = Position::default();

    let mut line = 0usize;
    let mut col = 0usize;

    // keep the current line in buffer.
    let mut buffer = Vec::new();
    let mut it = r.bytes().enumerate().peekable();

    while let Some((c, b)) = it.next() {
        let b = b?;

        let nl = match b {
            // macos
            CR => {
                // windows
                if let Some(&(_, Ok(NL))) = it.peek() {
                    it.next();
                }

                true
            }
            NL => true,
            _ => false,
        };

        if nl {
            line += 1;
            col = 0;
            buffer.clear();
        } else {
            buffer.push(b);
        }

        if c == span.start {
            start.line = line;
            start.col = encoding.column(&buffer, col)?;
        }

        if c == span.end {
            end.line = line;
            end.col = encoding.column(&buffer, col)?;
            break;
        }

        if !nl {
            col += 1;
        }
    }

    Ok((start, end))
}

/// Encoding for which to check the range.
#[derive(Debug, Clone, Copy)]
pub enum Encoding {
    /// Emit the raw byte offset for the column.
    Raw,
    /// Emit the UTF-8 offset for the column.
    Utf8,
    /// Emit the UTF-16 offset for the column.
    Utf16,
}

impl Encoding {
    /// Calculate the column, which depends on the encoding.
    pub fn column(self, buffer: &[u8], col: usize) -> Result<usize> {
        use self::Encoding::*;

        match self {
            Raw => Ok(col),
            Utf8 => Ok(::std::str::from_utf8(&buffer[..col])?.chars().count()),
            Utf16 => Ok(::std::str::from_utf8(&buffer[..col])?
                .encode_utf16()
                .count()),
        }
    }

    /// Calculate the column in either number of characters (`Raw`), number of bytes (`Utf8`), or
    /// number of code units (`Utf16`) from an iterator.
    pub fn column_iter<I>(self, iter: I, col: usize) -> usize
    where
        I: IntoIterator<Item = char>,
    {
        use self::Encoding::*;

        match self {
            Raw => iter.into_iter().take(col).count(),
            Utf8 => iter.into_iter().take(col).map(|c| c.len_utf8()).sum(),
            Utf16 => iter.into_iter().take(col).map(|c| c.len_utf16()).sum(),
        }
    }
}
