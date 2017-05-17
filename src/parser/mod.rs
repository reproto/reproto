mod parser;

pub mod ast;
pub mod errors;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use pest::prelude::StringInput;
use pest::Parser;
use errors::*;

pub fn parse_file(path: &Path) -> Result<ast::File> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;

    let mut parser = parser::Rdp::new(StringInput::new(&content));

    if !parser.file() {
        return Err("invalid syntax".into());
    }

    if !parser.end() {
        return Err("not parsed until end".into());
    }

    Ok(parser._file()?)
}
