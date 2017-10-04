//! # Helper trait to deal with value construction

use converter::Converter;
use core::{RpEnumOrdinal, RpEnumVariant};
use errors::*;
use genco::{Quoted, Tokens};
use std::rc::Rc;

pub trait ValueBuilder<'el>
where
    Self: Converter<'el>,
{
    fn ordinal<'a>(&self, variant: &'a RpEnumVariant) -> Result<Tokens<'el, Self::Custom>> {
        use self::RpEnumOrdinal::*;

        let out = match variant.ordinal {
            String(ref string) => Rc::new(string.as_str().to_string()).quoted().into(),
            Generated => Rc::new(variant.local_name.to_string()).quoted().into(),
        };

        Ok(out)
    }
}
