#![recursion_limit = "1000"]

extern crate clap;
extern crate linked_hash_map;
extern crate pulldown_cmark;
extern crate serde_json;
extern crate reproto_core;
extern crate reproto_parser;
extern crate reproto_repository;
extern crate reproto_backend;
extern crate reproto_backend_doc;
extern crate reproto_backend_java;
extern crate reproto_backend_js;
extern crate reproto_backend_json;
extern crate reproto_backend_python;
extern crate reproto_backend_rust;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

pub mod commands;
pub mod errors;
pub mod logger;

pub(crate) use clap::{App, Arg, ArgMatches, SubCommand};
pub(crate) use errors::*;
pub(crate) use reproto_backend::{CompilerOptions, Environment, Options, naming};
pub(crate) use reproto_backend_doc as backend_doc;
pub(crate) use reproto_backend_java as backend_java;
pub(crate) use reproto_backend_js as backend_js;
pub(crate) use reproto_backend_json as backend_json;
pub(crate) use reproto_backend_python as backend_python;
pub(crate) use reproto_backend_rust as backend_rust;
pub(crate) use reproto_core::{RpPackage, RpRequiredPackage, VersionReq};
pub(crate) use reproto_repository::{Filesystem, Paths, Resolver, Resolvers};
