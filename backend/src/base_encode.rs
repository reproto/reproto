//! # Helper trait for building a dynamic-language encode method

use core::{Pos, RpType, RpTypeId};
use errors::*;

pub trait BaseEncode {
    type Stmt;

    fn base_encode(
        &self,
        type_id: &RpTypeId,
        pos: &Pos,
        ty: &RpType,
        input: &Self::Stmt,
    ) -> Result<Self::Stmt>;
}
