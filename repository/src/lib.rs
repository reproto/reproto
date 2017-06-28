#![recursion_limit = "1000"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate reproto_core;
extern crate toml;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate openssl;
extern crate hex;
extern crate git2;

mod hex_slice;
mod index;
mod metadata;
mod objects;
mod repository;
mod resolver;
mod sha256;
mod git;
pub mod errors;

use reproto_core as core;

pub use self::index::{Index, index_from_url, IndexConfig, init_file_index};
pub use self::objects::{Objects, objects_from_url, ObjectsConfig};
pub use self::repository::*;
pub use self::resolver::*;
