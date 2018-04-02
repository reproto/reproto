//! # Helper trait for building a dynamic-language encode method

use super::converter::Converter;
use core::errors::*;
use core::{Flavor, RpType};
use genco::Tokens;

pub trait BaseEncode<'el, F: 'static>
where
    Self: Converter<'el, F>,
    F: Flavor<Type = RpType<F>>,
{
    fn base_encode(
        &self,
        ty: &'el F::Type,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>>;
}
