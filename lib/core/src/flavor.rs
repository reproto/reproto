//! The flavor of RpIR being used.

use errors::Result;
use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::hash;
use {RpEndpoint, RpEnumType, RpField, RpName, RpPackage, RpType, RpVersionedPackage};

pub trait FlavorField: fmt::Debug + Clone {
    /// Indicates if the field is discriminating in an untagged context.
    fn is_discriminating(&self) -> bool;
}

pub trait AsPackage
where
    Self: Sized,
{
    /// Attempt to treat the current object as a package.
    fn try_as_package<'a>(&'a self) -> Result<Cow<'a, RpPackage>>;

    /// Attempt to prefix the package.
    fn prefix_with(self, prefix: RpPackage) -> Self;
}

/// The flavor of intermediate representation being used.
pub trait Flavor: fmt::Debug + Clone + cmp::Eq + hash::Hash {
    /// The type that this flavor serializes to.
    type Type: fmt::Debug + Clone + cmp::Eq;
    /// The local field name.
    type Name: fmt::Display + fmt::Debug + Clone + cmp::Eq;
    /// The field that this flavor serializes to.
    type Field: FlavorField;
    /// The endpoint that this flavor serializes to.
    type Endpoint: fmt::Debug + Clone;
    /// The package type.
    type Package: fmt::Debug + Clone + cmp::Eq + cmp::Ord + hash::Hash + AsPackage;
    /// Enum type.
    type EnumType: fmt::Debug + Clone + cmp::Eq;
}

/// The first flavor where packages are fully qualified.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Hash)]
pub struct CoreFlavor;

impl Flavor for CoreFlavor {
    type Type = RpType<CoreFlavor>;
    type Name = RpName<CoreFlavor>;
    type Field = RpField<CoreFlavor>;
    type Endpoint = RpEndpoint<CoreFlavor>;
    type Package = RpVersionedPackage;
    type EnumType = RpEnumType;
}
