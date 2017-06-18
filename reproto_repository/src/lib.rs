#![recursion_limit = "1000"]

extern crate toml;
extern crate semver;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate reproto_core;

mod errors;
mod repository;
mod metadata;
mod resolver;

pub use repository::Repository;
pub use resolver::*;
