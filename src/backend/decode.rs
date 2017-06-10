//! # Helper trait for building a dynamic-language decode method

use core::*;
use super::converter::Converter;
use super::errors::*;

pub trait Decode
    where Self: Converter
{
    fn decode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt>;
}
