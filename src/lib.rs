#![recursion_limit = "1000"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

extern crate ansi_term;
extern crate clap;
extern crate linked_hash_map;
extern crate pulldown_cmark;
extern crate reproto_backend;
extern crate reproto_backend_doc;
extern crate reproto_backend_java;
extern crate reproto_backend_js;
extern crate reproto_backend_json;
extern crate reproto_backend_python;
extern crate reproto_backend_rust;
extern crate reproto_core;
extern crate reproto_parser;
extern crate reproto_repository;
extern crate toml;
extern crate url;
extern crate url_serde;

pub mod ops;
pub mod config;
pub mod errors;
pub mod logger;
pub mod error_handling;

pub(crate) use clap::{App, Arg, ArgMatches, SubCommand};
pub(crate) use config::read_config;
pub(crate) use errors::*;
pub(crate) use reproto_backend::{CompilerOptions, Environment, Options, naming};
pub(crate) use reproto_core::{RpPackage, RpRequiredPackage, VersionReq};
