#![recursion_limit = "1000"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate reproto_core as core;
extern crate toml;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate hex;
extern crate tokio_core;
extern crate hyper;
extern crate futures;
extern crate ring;

mod hex_slice;
mod index;
mod metadata;
mod objects;
mod repository;
mod resolver;
mod sha256;
mod git;
pub mod errors;


pub use self::index::{Index, IndexConfig, NoIndex, index_from_url, init_file_index};
pub use self::objects::{FileObjects, NoObjects, Objects, ObjectsConfig, objects_from_file,
                        objects_from_url};
pub use self::repository::*;
pub use self::resolver::*;
pub use self::sha256::{Checksum, Sha256 as Digest, to_sha256 as to_checksum};
