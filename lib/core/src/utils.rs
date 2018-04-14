use errors::Result;
use std::io::Read;

const NL: u8 = '\n' as u8;

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

    let mut it = r.bytes().peekable();
    let mut read = 0usize;

    while let Some(b) = it.next() {
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
pub fn find_range<'a, R: AsMut<Read + 'a>>(
    mut reader: R,
    span: (usize, usize),
) -> Result<(usize, usize, usize, usize)> {
    let r = reader.as_mut();

    let mut line_start = 0usize;
    let mut line_end = 0usize;

    let mut col_start = 0usize;
    let mut col_end = 0usize;

    let mut line = 0usize;
    let mut col = 0usize;

    let mut it = r.bytes().enumerate();

    while let Some((c, b)) = it.next() {
        let b = b?;

        let mut new_line = b == NL;

        if new_line {
            line += 1;
            col = 0;
        }

        if c == span.0 {
            line_start = line;
            col_start = col;
        }

        if c == span.1 {
            line_end = line;
            col_end = col;
            break;
        }

        if !new_line {
            col += 1;
        }
    }

    Ok((line_start, line_end, col_start, col_end))
}
