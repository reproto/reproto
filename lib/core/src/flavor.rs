//! The flavor of RpIR being used.

use RpType;
use serde;
use std::cmp;
use std::fmt;

/// The flavor of intermediate representation being used.
pub trait Flavor {
    type Type: cmp::PartialEq + cmp::Eq + serde::Serialize + fmt::Debug + fmt::Display + Clone;
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CoreFlavor;

impl Flavor for CoreFlavor {
    type Type = RpType;
}
