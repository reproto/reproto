#![recursion_limit = "1000"]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate genco;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate reproto_ast as ast;
extern crate reproto_core as core;
extern crate reproto_parser as parser;
extern crate reproto_path_parser as path_parser;
extern crate reproto_repository as repository;
extern crate serde_json;

mod base_decode;
mod base_encode;
mod converter;
mod dynamic_converter;
mod dynamic_decode;
mod dynamic_encode;
mod environment;
mod for_context;
mod into_bytes;
mod into_model;
mod macros;
mod naming;
mod package_processor;
mod package_utils;
mod scope;
mod code;
mod initializer;
pub mod errors;

pub use self::base_decode::BaseDecode;
pub use self::base_encode::BaseEncode;
pub use self::code::Code;
pub use self::converter::Converter;
pub use self::dynamic_converter::DynamicConverter;
pub use self::dynamic_decode::DynamicDecode;
pub use self::dynamic_encode::DynamicEncode;
pub use self::environment::Environment;
pub use self::for_context::ForContext;
pub use self::initializer::Initializer;
pub use self::into_bytes::IntoBytes;
pub use self::naming::{CamelCase, FromNaming, Naming, SnakeCase};
pub use self::package_processor::PackageProcessor;
pub use self::package_utils::PackageUtils;
pub use clap::{App, Arg, ArgMatches};
