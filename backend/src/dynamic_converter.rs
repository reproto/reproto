//! # Converter for core data structures into processor-specific ones.

use converter::Converter;
use core::RpType;
use genco::Tokens;

pub trait DynamicConverter<'el>
where
    Self: Converter<'el>,
{
    /// If the type deeply compatible already with the language and need no conversion.
    fn is_native(&self, &RpType) -> bool;

    fn map_key_var(&self) -> Tokens<'el, Self::Custom>;

    fn map_value_var(&self) -> Tokens<'el, Self::Custom>;

    fn array_inner_var(&self) -> Tokens<'el, Self::Custom>;
}
