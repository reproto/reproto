//! # Converter for core data structures into processor-specific ones.

use core::*;
use super::errors::*;

pub trait Converter {
    type Type;

    fn convert_type(&self, pos: &RpPos, type_id: &RpTypeId) -> Result<Self::Type>;

    fn convert_constant(&self, pos: &RpPos, type_id: &RpTypeId) -> Result<Self::Type> {
        self.convert_type(pos, type_id)
    }
}
