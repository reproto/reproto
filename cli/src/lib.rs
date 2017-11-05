#![recursion_limit = "1000"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

extern crate ansi_term;
extern crate clap;
extern crate reproto_backend as backend;
extern crate reproto_backend_doc;
extern crate reproto_backend_java;
extern crate reproto_backend_js;
extern crate reproto_backend_json;
extern crate reproto_backend_python;
extern crate reproto_backend_rust;
extern crate reproto_core as core;
extern crate reproto_parser as parser;
extern crate reproto_repository as repository;
extern crate toml;
extern crate url;

pub mod ops;
pub mod config;
pub mod manifest;
pub mod errors;
pub mod output;
