extern crate languageserver_types as ty;
extern crate linked_hash_map;
extern crate log;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_naming as naming;
extern crate reproto_parser as parser;
extern crate reproto_path_parser as path_parser;
extern crate serde;
extern crate serde_json as json;
#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate url_serde;

mod types;

use self::ContentType::*;
use core::errors::Result;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::result;

#[derive(Debug)]
enum ContentType {
    JsonRPC,
}

#[derive(Debug)]
struct Headers {
    content_type: ContentType,
    content_length: u32,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            content_type: JsonRPC,
            content_length: 0u32,
        }
    }

    fn clear(&mut self) {
        self.content_type = JsonRPC;
        self.content_length = 0;
    }
}

struct InputReader<R> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R> InputReader<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    fn next_line<'a>(&'a mut self) -> Result<Option<&'a [u8]>> {
        self.buffer.clear();
        self.reader.read_until('\n' as u8, &mut self.buffer)?;

        if self.buffer.is_empty() {
            return Ok(None);
        }

        Ok(Some(trim(&self.buffer)))
    }
}

impl<R> Read for InputReader<R>
where
    R: BufRead,
{
    fn read(&mut self, buf: &mut [u8]) -> result::Result<usize, io::Error> {
        self.reader.read(buf)
    }
}

pub fn server<R, W>(input: R, mut o: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut reader = InputReader::new(BufReader::new(input));
    let mut headers = Headers::new();

    loop {
        headers.clear();

        if !read_headers(&mut reader, &mut headers)? {
            break;
        }

        if headers.content_length == 0 {
            continue;
        }

        let reader = (&mut reader).take(headers.content_length as u64);

        match headers.content_type {
            JsonRPC => {
                let env: types::RequestMessage = json::from_reader(reader)?;
                writeln!(o, "{:?}", env)?;
            }
        }
    }

    Ok(())
}

/// Read headers.
fn read_headers<R>(reader: &mut InputReader<R>, headers: &mut Headers) -> Result<bool>
where
    R: BufRead,
{
    loop {
        let line = reader.next_line()?;

        let line = match line {
            Some(line) => line,
            None => return Ok(false),
        };

        if line == b"" {
            break;
        }

        let mut parts = line.splitn(2, |b| *b == b':');

        let (key, value) = match (parts.next(), parts.next()) {
            (Some(key), Some(value)) => (trim(key), trim(value)),
            out => {
                return Err(format!("bad header: {:?}", out).into());
            }
        };

        match key {
            b"Content-Type" => match value {
                b"application/vscode-jsonrpc; charset=utf-8" => {
                    headers.content_type = JsonRPC;
                }
                value => {
                    return Err(format!("bad value: {:?}", value).into());
                }
            },
            b"Content-Length" => {
                let value = ::std::str::from_utf8(value)
                    .map_err(|e| format!("bad content-length: {:?}: {}", value, e))?;

                let value = value
                    .parse::<u32>()
                    .map_err(|e| format!("bad content-length: {}: {}", value, e))?;

                headers.content_length = value;
            }
            key => {
                return Err(format!("bad header: {:?}", key).into());
            }
        }
    }

    Ok(true)
}

/// Trim the string from whitespace.
fn trim(data: &[u8]) -> &[u8] {
    let s = data.iter()
        .position(|b| *b != b'\n' && *b != b'\r' && *b != b' ')
        .unwrap_or(data.len());

    let data = &data[s..];

    let e = data.iter()
        .rev()
        .position(|b| *b != b'\n' && *b != b'\r' && *b != b' ')
        .map(|p| data.len() - p)
        .unwrap_or(0usize);

    &data[..e]
}
