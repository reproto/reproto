#![recursion_limit = "1000"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate codeviz_common;
extern crate reproto_core;
extern crate reproto_parser;
extern crate reproto_repository;
extern crate linked_hash_map;
extern crate clap;
extern crate serde_json;

mod base_decode;
mod base_encode;
mod collecting;
mod container;
mod converter;
mod dynamic_converter;
mod dynamic_decode;
mod dynamic_encode;
mod environment;
pub mod for_context;
mod match_decode;
mod package_processor;
mod package_utils;
mod value_builder;
mod variables;
pub mod errors;
pub mod naming;
pub mod options;
mod compiler_options;

pub use clap::{App, Arg, ArgMatches};
pub use compiler_options::CompilerOptions;
pub(crate) use errors::*;
pub use options::Options;
pub use reproto_core::*;
pub use self::base_decode::*;
pub use self::base_encode::*;
pub use self::collecting::*;
pub use self::container::Container;
pub use self::converter::*;
pub use self::dynamic_converter::*;
pub use self::dynamic_decode::*;
pub use self::dynamic_encode::*;
pub use self::environment::{Environment, InitFields};
pub use self::match_decode::*;
pub use self::package_processor::*;
pub use self::package_utils::*;
pub use self::value_builder::*;
pub use self::variables::*;
