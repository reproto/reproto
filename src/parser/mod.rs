pub mod ast;
pub mod errors;
pub mod parser;

use pest::Parser;
use pest::prelude::StringInput;
use self::errors::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

static NL: u8 = '\n' as u8;
static CR: u8 = '\r' as u8;

pub fn find_line(path: &Path, pos: (usize, usize)) -> Result<(String, usize, (usize, usize))> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);

    let start = pos.0;
    let end = pos.1;

    let mut line_start = 0usize;
    let mut line_buffer: Vec<u8> = Vec::new();
    let mut lines: usize = 0;
    let mut it = reader.bytes().enumerate();

    while let Some((i, b)) = it.next() {
        let b = b?;

        if b == NL || b == CR {
            if i >= start {
                let line = String::from_utf8(line_buffer)?;
                let end = if i > end { end } else { i };
                let range = (start - line_start, end - line_start);
                return Ok((line, lines, range));
            }

            line_start = i;
            lines = lines + 1;
            line_buffer.clear();
            continue;
        }

        line_buffer.push(b);
    }

    Err("bad file position".into())
}

pub fn parse_file(path: &Path) -> Result<ast::File> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    let mut parser = parser::Rdp::new(StringInput::new(&content));

    if !parser.file() {
        let pos = parser.tracked_len_pos();
        let (expected, _) = parser.expected();
        let pos = (path.to_owned(), pos.1, pos.1);
        return Err(ErrorKind::Syntax(pos, expected).into());
    }

    if !parser.end() {
        return Err("not parsed until end".into());
    }

    #[cfg(feature = "tracing")]
    {
        debug!("Parser queue for file {}:", path.display());

        for (i, e) in parser.queue().iter().enumerate() {
            debug!("  {:>3} = {:?}", i, e);
        }
    }

    parser._file()
}
