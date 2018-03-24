//! The flavor of RpIR being used.

use {RpField, RpType};
use serde;
use std::cmp;
use std::fmt;

/// The flavor of intermediate representation being used.
pub trait Flavor {
    /// The type that this flavor serializes to.
    type Type: cmp::PartialEq + cmp::Eq + serde::Serialize + fmt::Debug + Clone;
    /// The field that this flavor serializes to.
    type Field: serde::Serialize + fmt::Debug + Clone;
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoreFlavor;

impl Flavor for CoreFlavor {
    type Type = RpType;
    type Field = RpField<CoreFlavor>;
}
