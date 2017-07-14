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
extern crate tokio_core;
extern crate hyper;
extern crate futures;

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
pub use self::objects::{FileObjects, Objects, objects_from_url, objects_from_file, ObjectsConfig};
pub use self::repository::*;
pub use self::resolver::*;
pub use self::sha256::{Checksum, to_sha256 as to_checksum, Sha256 as Digest};
