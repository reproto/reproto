//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::{Pos, RpName, RpType};
use errors::*;

pub trait BaseDecode
where
    Self: Converter,
{
    fn base_decode(
        &self,
        name: &RpName,
        pos: &Pos,
        ty: &RpType,
        input: &Self::Stmt,
    ) -> Result<Self::Stmt>;
}
