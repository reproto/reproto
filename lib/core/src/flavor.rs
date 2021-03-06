//! The flavor of RpIR being used.

use crate::errors::Result;
use crate::{
    RpEndpoint, RpEnumType, RpField, RpName, RpPackage, RpType, RpVersionedPackage, Spanned,
};
use serde::Serialize;
use std::borrow::Cow;
use std::cmp;
use std::fmt;
use std::hash;

pub trait FlavorField
where
    Self: 'static + fmt::Debug + Clone,
{
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
pub trait Flavor
where
    Self: 'static + fmt::Debug + Clone + Copy + cmp::Eq + hash::Hash,
{
    /// The type that this flavor serializes to.
    type Type: fmt::Debug + Clone;
    /// The local field name.
    type Name: fmt::Debug + Clone;
    /// The field that this flavor serializes to.
    type Field: FlavorField;
    /// The endpoint that this flavor serializes to.
    type Endpoint: fmt::Debug + Clone;
    /// The package type.
    type Package: fmt::Debug + Clone + cmp::Eq + cmp::Ord + hash::Hash + Default + AsPackage;
    /// Enum type.
    type EnumType: fmt::Debug + Clone;
}

/// The first flavor where packages are fully qualified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Hash)]
pub enum CoreFlavor {}

impl Flavor for CoreFlavor {
    type Type = RpType<CoreFlavor>;
    type Name = Spanned<RpName<CoreFlavor>>;
    type Field = RpField<CoreFlavor>;
    type Endpoint = RpEndpoint<CoreFlavor>;
    type Package = RpVersionedPackage;
    type EnumType = RpEnumType;
}
