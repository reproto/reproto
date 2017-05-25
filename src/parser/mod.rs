mod parser;
pub mod errors;

use ast;
use pest::Parser;
use pest::prelude::StringInput;
use self::errors::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub fn find_line(path: &Path, pos: usize) -> Result<(String, usize)> {
    let file = File::open(path)?;
    let mut current_pos: usize = 0;
    let mut lines: usize = 0;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let line = line?;
        lines += 1;

        if current_pos >= pos {
            return Ok((line, lines));
        }

        current_pos += line.len() + 1;
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
        let (line_string, line) = find_line(path, pos)?;
        return Err(ErrorKind::Syntax("unexpected input".to_owned(), line_string, line).into());
    }

    if !parser.end() {
        return Err("not parsed until end".into());
    }

    Ok(parser._file()?)
}
