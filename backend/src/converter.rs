//! # Converter for core data structures into processor-specific ones.

use core::{RpEnumVariant, RpName};
use errors::*;
use genco::{Custom, Quoted, Tokens};
use std::rc::Rc;

pub trait Converter<'el> {
    type Custom: 'el + Custom + Clone;

    fn convert_type(&self, name: &'el RpName) -> Result<Tokens<'el, Self::Custom>>;

    fn convert_constant(&self, name: &'el RpName) -> Result<Tokens<'el, Self::Custom>> {
        self.convert_type(name)
    }

    /// Build an ordinal value.
    fn ordinal<'a>(&self, variant: &'a RpEnumVariant) -> Result<Tokens<'el, Self::Custom>> {
        Ok(Rc::new(variant.ordinal().to_string()).quoted().into())
    }
}
