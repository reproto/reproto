//! # Helper trait for building a dynamic-language decode method

use core::*;
use super::errors::*;

pub trait Decode {
    type Output;

    fn decode<S>(&self,
                 type_id: &RpTypeId,
                 pos: &RpPos,
                 ty: &RpType,
                 input: S)
                 -> Result<Self::Output>
        where S: Into<Self::Output>;
}
