//! # Converter for core data structures into processor-specific ones.

use core::RpName;
use core::errors::*;
use genco::{Custom, Tokens};

pub trait Converter<'el> {
    type Custom: 'el + Custom + Clone;

    fn convert_type(&self, name: &RpName) -> Result<Tokens<'el, Self::Custom>>;

    fn convert_constant(&self, name: &RpName) -> Result<Tokens<'el, Self::Custom>> {
        self.convert_type(name)
    }
}
