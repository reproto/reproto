//! # Converter for core data structures into processor-specific ones.

use core::{RpEnumOrdinal, RpEnumVariant, RpName};
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
        use self::RpEnumOrdinal::*;

        let out = match variant.ordinal {
            String(ref string) => Rc::new(string.as_str().to_string()).quoted().into(),
            Generated => Rc::new(variant.local_name.to_string()).quoted().into(),
        };

        Ok(out)
    }
}
