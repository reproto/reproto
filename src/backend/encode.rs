//! # Helper trait for building a dynamic-language encode method

use core::*;
use super::errors::*;

pub trait Encode {
    type Stmt;

    fn encode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt>;
}
