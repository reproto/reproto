#![recursion_limit = "1000"]
#![allow(unknown_lints)]

extern crate mime as extern_mime;
pub extern crate serde;

#[macro_use]
mod macros;
mod as_loc;
mod attributes;
mod diagnostics;
pub mod errors;
mod flavor;
pub mod flavored;
mod fs;
mod import;
mod mime;
mod option_entry;
mod options;
pub mod regex;
mod reporter;
mod resolver;
mod rp_channel;
mod rp_code;
mod rp_decl;
mod rp_endpoint;
mod rp_enum;
mod rp_field;
mod rp_file;
mod rp_interface;
mod rp_name;
mod rp_number;
mod rp_package;
mod rp_package_format;
mod rp_path_spec;
mod rp_reg;
mod rp_required_package;
mod rp_service;
mod rp_tuple;
mod rp_type;
mod rp_type_model;
mod rp_value;
mod rp_versioned_package;
mod source;
mod span;
mod spanned;
pub mod translator;
pub mod utils;
mod with_span;

pub use self::attributes::{Attributes, Selection};
pub use self::diagnostics::{
    Diagnostic, Diagnostics, SourceDiagnostic, SourceDiagnostics, SymbolKind,
};
pub use self::flavor::{AsPackage, CoreFlavor, Flavor, FlavorField};
pub use self::fs::{CapturingFilesystem, Filesystem, Handle, RealFilesystem};
pub use self::import::Import;
pub use self::mime::Mime;
pub use self::option_entry::OptionEntry;
pub use self::options::Options;
pub use self::reporter::{Reported, Reporter};
pub use self::resolver::{EmptyResolver, Resolved, ResolvedByPrefix, Resolver};
pub use self::rp_channel::RpChannel;
pub use self::rp_code::{RpCode, RpContext};
pub use self::rp_decl::{RpDecl, RpNamed};
pub use self::rp_endpoint::{
    RpAccept, RpEndpoint, RpEndpointArgument, RpEndpointHttp, RpEndpointHttp1, RpHttpMethod,
};
pub use self::rp_enum::{
    RpEnumBody, RpEnumType, RpVariant, RpVariantRef, RpVariantValue, RpVariants,
};
pub use self::rp_field::RpField;
pub use self::rp_file::{RpEnabledFeature, RpFile};
pub use self::rp_interface::{RpInterfaceBody, RpSubType, RpSubTypeStrategy, DEFAULT_TAG};
pub use self::rp_name::RpName;
pub use self::rp_number::RpNumber;
pub use self::rp_package::RpPackage;
pub use self::rp_package_format::RpPackageFormat;
pub use self::rp_path_spec::{RpPathPart, RpPathSpec, RpPathStep};
pub use self::rp_reg::RpReg;
pub use self::rp_required_package::RpRequiredPackage;
pub use self::rp_service::{RpServiceBody, RpServiceBodyHttp};
pub use self::rp_tuple::RpTupleBody;
pub use self::rp_type::{
    RpNumberKind, RpNumberType, RpNumberValidate, RpStringType, RpStringValidate, RpType,
};
pub use self::rp_type_model::RpTypeBody;
pub use self::rp_value::RpValue;
pub use self::rp_versioned_package::RpVersionedPackage;
pub use self::source::Source;
pub use self::span::Span;
pub use self::spanned::Spanned;
pub use self::translator::{FlavorTranslator, PackageTranslator, Translate, Translator};
pub use self::utils::{Encoding, Position};
pub use self::with_span::WithSpan;
pub use num_bigint::BigInt;
pub use relative_path::{RelativePath, RelativePathBuf};
pub use ropey::Rope;
pub use semver::{Range, Version};
