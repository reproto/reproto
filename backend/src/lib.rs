#![recursion_limit = "1000"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate genco;
extern crate reproto_core as core;
extern crate reproto_parser as parser;
extern crate reproto_repository as repository;
extern crate linked_hash_map;
extern crate clap;
extern crate serde_json;

mod into_bytes;
mod into_model;
mod scope;
mod base_decode;
mod base_encode;
mod converter;
mod dynamic_converter;
mod dynamic_decode;
mod dynamic_encode;
mod environment;
mod for_context;
mod package_processor;
mod package_utils;
pub mod errors;
mod naming;
mod options;
mod compiler_options;
mod macros;

pub use self::base_decode::BaseDecode;
pub use self::base_encode::BaseEncode;
pub use self::compiler_options::CompilerOptions;
pub use self::converter::Converter;
pub use self::dynamic_converter::DynamicConverter;
pub use self::dynamic_decode::DynamicDecode;
pub use self::dynamic_encode::DynamicEncode;
pub use self::environment::Environment;
pub use self::for_context::ForContext;
pub use self::into_bytes::IntoBytes;
pub use self::naming::{CamelCase, FromNaming, Naming, SnakeCase};
pub use self::options::Options;
pub use self::package_processor::PackageProcessor;
pub use self::package_utils::PackageUtils;
pub use clap::{App, Arg, ArgMatches};
