//! # Converter for core data structures into processor-specific ones.

use converter::Converter;
use core::RpType;

pub trait DynamicConverter
    where Self: Converter
{
    /// If the type deeply compatible already with the language and need no conversion.
    fn is_native(&self, &RpType) -> bool;

    fn map_key_var(&self) -> Self::Stmt;

    fn map_value_var(&self) -> Self::Stmt;

    fn array_inner_var(&self) -> Self::Stmt;
}
