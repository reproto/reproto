//! # Converter for core data structures into processor-specific ones.

use core::{Flavor, RpName};
use core::errors::*;
use genco::{Custom, Tokens};

pub trait Converter<'el, F>
where
    F: Flavor,
{
    type Custom: 'el + Custom + Clone;

    fn convert_type(&self, name: &RpName) -> Result<Tokens<'el, Self::Custom>>;

    fn convert_constant(&self, name: &RpName) -> Result<Tokens<'el, Self::Custom>> {
        self.convert_type(name)
    }
}
