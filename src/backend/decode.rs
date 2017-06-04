//! # Helper trait for building a dynamic-language decode method

use core::*;
use super::errors::*;

pub trait Decode {
    type Stmt;

    fn decode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt>;
}
