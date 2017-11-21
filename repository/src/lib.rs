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
extern crate relative_path;

mod checksum;
mod git;
mod hex_slice;
mod index;
mod metadata;
mod objects;
mod repository;
mod resolver;
mod sha256;
pub mod errors;
mod update;

pub use self::checksum::Checksum;
pub use self::git::GitRepo;
pub use self::index::{Index, IndexConfig, NoIndex, index_from_path, index_from_url,
                      init_file_index};
pub use self::objects::{FileObjects, NoObjects, Objects, ObjectsConfig, objects_from_path,
                        objects_from_url};
pub use self::repository::Repository;
pub use self::resolver::{Paths, Resolved, ResolvedByPrefix, Resolver, Resolvers};
pub use self::sha256::{Sha256 as Digest, to_sha256 as to_checksum};
pub use self::update::Update;
