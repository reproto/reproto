//! # Helper trait for building a dynamic-language decode method

use super::*;

pub trait BaseDecode
    where Self: Converter
{
    fn base_decode(&self,
                   type_id: &RpTypeId,
                   pos: &RpPos,
                   ty: &RpType,
                   input: &Self::Stmt)
                   -> Result<Self::Stmt>;
}
