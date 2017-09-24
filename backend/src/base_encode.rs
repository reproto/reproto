//! # Helper trait for building a dynamic-language encode method

use core::{RpName, RpType};
use errors::*;

pub trait BaseEncode {
    type Stmt;

    fn base_encode(&self, name: &RpName, ty: &RpType, input: &Self::Stmt) -> Result<Self::Stmt>;
}
