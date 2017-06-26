#![recursion_limit = "1000"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate reproto_core;
extern crate toml;
extern crate serde_json;
extern crate url;
extern crate url_serde;

mod index;
mod objects;
mod metadata;
mod resolver;
mod repository;
pub mod errors;

pub use self::index::{Index, index_from_url};
pub use self::objects::{Objects, objects_from_url};
pub use self::repository::*;
pub use self::resolver::*;
