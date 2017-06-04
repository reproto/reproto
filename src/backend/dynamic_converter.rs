//! # Converter for core data structures into processor-specific ones.

use core::*;

pub trait DynamicConverter {
    type DynamicConverterStmt;

    /// If the type deeply compatible already with the language and need no conversion.
    fn is_native(&self, &RpType) -> bool;

    fn map_key_var(&self) -> Self::DynamicConverterStmt;

    fn map_value_var(&self) -> Self::DynamicConverterStmt;

    fn array_inner_var(&self) -> Self::DynamicConverterStmt;
}
