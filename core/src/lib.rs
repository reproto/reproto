#![recursion_limit = "1000"]

extern crate mime as extern_mime;
extern crate num;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
pub extern crate semver;

mod object;
mod with_pos;
mod for_each_loc;
mod error_pos;
mod loc;
mod pos;
mod options;
mod models;
pub mod errors;
mod merge;
mod mime;
mod rp_number;
mod rp_package;
mod rp_versioned_package;
mod rp_required_package;
mod rp_modifier;

pub use self::error_pos::ErrorPos;
pub use self::for_each_loc::ForEachLoc;
pub use self::loc::Loc;
pub use self::merge::Merge;
pub use self::mime::Mime;
pub use self::models::*;
pub use self::object::{BytesObject, Object, PathObject};
pub use self::options::Options;
pub use self::pos::Pos;
pub use self::rp_modifier::RpModifier;
pub use self::rp_number::RpNumber;
pub use self::rp_package::RpPackage;
pub use self::rp_required_package::RpRequiredPackage;
pub use self::rp_versioned_package::RpVersionedPackage;
pub use self::with_pos::WithPos;
pub use semver::{Version, VersionReq};
