use errors::Result;
use std::io::Read;

const NL: u8 = '\n' as u8;

/// Find the line corresponding to the given position.
pub fn find_line<'a, R: AsMut<Read + 'a>>(
    mut reader: R,
    pos: (usize, usize),
) -> Result<(String, usize, (usize, usize))> {
    let r = reader.as_mut();

    let mut line = 0usize;
    let mut current = 0usize;
    let mut buffer: Vec<u8> = Vec::new();

    let start = pos.0;
    let end = pos.1;

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
