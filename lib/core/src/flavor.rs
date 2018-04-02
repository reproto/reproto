//! The flavor of RpIR being used.

use std::cmp;
use std::fmt;
use std::hash;
use {RpEndpoint, RpField, RpPackage, RpType, RpVersionedPackage};

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

/// The first flavor where packages are fully qualified.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Hash)]
pub struct CoreFlavor;

impl Flavor for CoreFlavor {
    type Type = RpType<CoreFlavor>;
    type Field = RpField<CoreFlavor>;
    type Endpoint = RpEndpoint<CoreFlavor>;
    type Package = RpVersionedPackage;
}

/// The second flavor where packages have been translated from a versioned variation, to a minimal
/// RpPackage variant where the names are identifier-safe.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Hash)]
pub struct CoreFlavor2;

impl Flavor for CoreFlavor2 {
    type Type = RpType<CoreFlavor2>;
    type Field = RpField<CoreFlavor2>;
    type Endpoint = RpEndpoint<CoreFlavor2>;
    type Package = RpPackage;
}
