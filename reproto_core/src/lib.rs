#![recursion_limit = "1000"]

extern crate mime;
extern crate num;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
pub extern crate semver;

mod loc;
mod merge;
mod options;
mod rp_by_type_match;
mod rp_by_value_match;
mod rp_code;
mod rp_decl;
mod rp_enum_body;
mod rp_enum_variant;
mod rp_field;
mod rp_field_init;
mod rp_file;
mod rp_instance;
mod rp_interface_body;
mod rp_loc;
mod rp_match_condition;
mod error_pos;
mod rp_match_decl;
mod rp_match_kind;
mod rp_match_member;
mod rp_match_variable;
mod rp_modifier;
mod rp_modifiers;
mod rp_name;
mod rp_number;
mod rp_option_decl;
mod rp_package;
mod rp_package_decl;
mod rp_path_segment;
mod rp_path_spec;
mod rp_registered;
mod rp_required_package;
mod rp_service_accepts;
mod rp_service_body;
mod rp_service_endpoint;
mod rp_service_returns;
mod rp_sub_type;
mod rp_tuple_body;
mod rp_type;
mod rp_type_body;
mod rp_type_id;
mod rp_use_decl;
mod rp_value;
mod rp_versioned_package;
pub mod errors;

pub use self::error_pos::*;
pub use self::loc::*;
pub use self::merge::*;
pub use self::options::*;
pub use self::rp_by_type_match::*;
pub use self::rp_by_value_match::*;
pub use self::rp_code::*;
pub use self::rp_decl::*;
pub use self::rp_enum_body::*;
pub use self::rp_enum_body::*;
pub use self::rp_enum_variant::*;
pub use self::rp_field::*;
pub use self::rp_field_init::*;
pub use self::rp_file::*;
pub use self::rp_instance::*;
pub use self::rp_interface_body::*;
pub use self::rp_loc::*;
pub use self::rp_match_condition::*;
pub use self::rp_match_decl::*;
pub use self::rp_match_kind::*;
pub use self::rp_match_member::*;
pub use self::rp_match_variable::*;
pub use self::rp_modifier::*;
pub use self::rp_modifiers::*;
pub use self::rp_name::*;
pub use self::rp_number::*;
pub use self::rp_option_decl::*;
pub use self::rp_package::*;
pub use self::rp_package_decl::*;
pub use self::rp_path_segment::*;
pub use self::rp_path_spec::*;
pub use self::rp_registered::*;
pub use self::rp_required_package::*;
pub use self::rp_service_accepts::*;
pub use self::rp_service_body::*;
pub use self::rp_service_endpoint::*;
pub use self::rp_service_returns::*;
pub use self::rp_sub_type::*;
pub use self::rp_tuple_body::*;
pub use self::rp_type::*;
pub use self::rp_type_body::*;
pub use self::rp_type_id::*;
pub use self::rp_use_decl::*;
pub use self::rp_value::*;
pub use self::rp_versioned_package::*;
pub use semver::Version;
pub use semver::VersionReq;

#[derive(Debug, Clone)]
pub struct Mime(mime::Mime);

impl serde::Serialize for Mime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl ::std::str::FromStr for Mime {
    type Err = errors::Error;

    fn from_str(s: &str) -> errors::Result<Self> {
        Ok(Mime(s.parse().map_err(errors::ErrorKind::MimeFromStrError)?))
    }
}

impl ::std::fmt::Display for Mime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
