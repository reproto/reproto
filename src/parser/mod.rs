mod parser;
pub mod errors;

use ast;
use pest::Parser;
use pest::prelude::StringInput;
use self::errors::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

static NL: u8 = '\n' as u8;

pub fn find_line(path: &Path,
                 pos: (usize, usize))
                 -> Result<(String, String, usize, (usize, usize))> {
    let file = File::open(path)?;
    let reader = BufReader::new(&file);

    let start = pos.0;
    let end = pos.1;

    let mut line_start = 0usize;
    let mut exact_buffer: Vec<u8> = Vec::new();
    let mut line_buffer: Vec<u8> = Vec::new();
    let mut lines: usize = 0;
    let mut it = reader.bytes().enumerate();

    while let Some((i, b)) = it.next() {
        let b = b?;

        if i >= start && i < end {
            exact_buffer.push(b);
        }

        if b == NL {
            if i >= start {
                let line = String::from_utf8(line_buffer)?;
                let exact = String::from_utf8(exact_buffer)?;
                let range = (start - line_start, end - line_start);
                return Ok((line, exact, lines, range));
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
        let (_, pos) = parser.tracked_len_pos();
        let (line, _, lines, _) = find_line(path, (pos, pos))?;
        return Err(ErrorKind::Syntax("unexpected input".to_owned(), line, lines).into());
    }

    if !parser.end() {
        return Err("not parsed until end".into());
    }

    Ok(parser._file())
}
