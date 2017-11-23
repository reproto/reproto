#![recursion_limit = "1000"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
extern crate mime as extern_mime;
extern crate num;
extern crate serde;
extern crate relative_path;
extern crate linked_hash_map;
pub extern crate semver;

#[macro_use]
mod macros;
mod as_loc;
mod rp_code;
mod error_pos;
mod for_each_loc;
mod loc;
mod mime;
mod object;
mod option_entry;
mod options;
mod pos;
mod rp_channel;
mod rp_decl;
mod rp_endpoint;
mod rp_enum_body;
mod rp_enum_type;
mod rp_path_spec;
mod rp_path_segment;
mod rp_field;
mod rp_file;
mod rp_interface_body;
mod rp_modifier;
mod rp_name;
mod rp_number;
mod rp_option_decl;
mod rp_package;
mod rp_package_format;
mod rp_reg;
mod rp_required_package;
mod rp_service_body;
mod rp_sub_type;
mod rp_tuple_body;
mod rp_type;
mod rp_type_body;
mod rp_value;
mod rp_variant;
mod rp_versioned_package;
mod with_pos;
mod rp_enum_ordinal;
mod context;
pub mod errors;

pub use self::context::{Context, ContextItem, Reporter};
pub use self::error_pos::ErrorPos;
pub use self::for_each_loc::ForEachLoc;
pub use self::loc::Loc;
pub use self::mime::Mime;
pub use self::object::{BytesObject, Object, PathObject};
pub use self::option_entry::OptionEntry;
pub use self::options::Options;
pub use self::pos::Pos;
pub use self::rp_channel::RpChannel;
pub use self::rp_code::RpCode;
pub use self::rp_decl::RpDecl;
pub use self::rp_endpoint::RpEndpoint;
pub use self::rp_enum_body::RpEnumBody;
pub use self::rp_enum_ordinal::RpEnumOrdinal;
pub use self::rp_enum_type::RpEnumType;
pub use self::rp_field::RpField;
pub use self::rp_file::RpFile;
pub use self::rp_interface_body::RpInterfaceBody;
pub use self::rp_modifier::RpModifier;
pub use self::rp_name::RpName;
pub use self::rp_number::RpNumber;
pub use self::rp_option_decl::RpOptionDecl;
pub use self::rp_package::RpPackage;
pub use self::rp_package_format::RpPackageFormat;
pub use self::rp_path_segment::RpPathSegment;
pub use self::rp_path_spec::RpPathSpec;
pub use self::rp_reg::RpReg;
pub use self::rp_required_package::RpRequiredPackage;
pub use self::rp_service_body::RpServiceBody;
pub use self::rp_sub_type::RpSubType;
pub use self::rp_tuple_body::RpTupleBody;
pub use self::rp_type::RpType;
pub use self::rp_type_body::RpTypeBody;
pub use self::rp_value::RpValue;
pub use self::rp_variant::RpVariant;
pub use self::rp_versioned_package::RpVersionedPackage;
pub use self::with_pos::WithPos;
pub use semver::{Version, VersionReq};
