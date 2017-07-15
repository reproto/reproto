//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::{Pos, RpType, RpTypeId};
use errors::*;

pub trait BaseDecode
    where Self: Converter
{
    fn base_decode(&self,
                   type_id: &RpTypeId,
                   pos: &Pos,
                   ty: &RpType,
                   input: &Self::Stmt)
                   -> Result<Self::Stmt>;
}
