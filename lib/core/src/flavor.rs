//! The flavor of RpIR being used.

use std::cmp;
use std::fmt;
use std::hash;
use {RpEndpoint, RpField, RpType, RpVersionedPackage};

/// The flavor of intermediate representation being used.
pub trait Flavor: fmt::Debug + Clone + cmp::Eq + hash::Hash {
    /// The type that this flavor serializes to.
    type Type: fmt::Debug + Clone + cmp::Eq;
    /// The field that this flavor serializes to.
    type Field: fmt::Debug + Clone;
    /// The endpoint that this flavor serializes to.
    type Endpoint: fmt::Debug + Clone;
    /// The package type.
    type Package: fmt::Debug + Clone + cmp::Eq + cmp::Ord + hash::Hash;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Hash)]
pub struct CoreFlavor;

impl Flavor for CoreFlavor {
    type Type = RpType<CoreFlavor>;
    type Field = RpField<CoreFlavor>;
    type Endpoint = RpEndpoint<CoreFlavor>;
    type Package = RpVersionedPackage;
}
