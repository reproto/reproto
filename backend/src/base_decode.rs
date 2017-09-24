//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::{RpName, RpType};
use errors::*;

pub trait BaseDecode
where
    Self: Converter,
{
    fn base_decode(&self, name: &RpName, ty: &RpType, input: &Self::Stmt) -> Result<Self::Stmt>;
}
