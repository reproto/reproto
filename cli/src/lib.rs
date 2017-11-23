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
extern crate reproto_backend_doc as doc;
extern crate reproto_backend_java as java;
extern crate reproto_backend_js as js;
extern crate reproto_backend_json as json;
extern crate reproto_backend_python as python;
extern crate reproto_backend_rust as rust;
extern crate reproto_core as core;
extern crate reproto_semck as semck;
extern crate reproto_manifest as manifest;
extern crate reproto_parser as parser;
extern crate reproto_repository as repository;
extern crate toml;
extern crate url;
extern crate relative_path;

pub mod ops;
pub mod config;
pub mod errors;
pub mod output;
mod build_spec;
mod config_env;
