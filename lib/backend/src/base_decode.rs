//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::{Flavor, RpType};
use core::errors::*;
use genco::Tokens;

pub trait BaseDecode<'el, F>
where
    Self: Converter<'el, F>,
    F: Flavor<Type = RpType>,
{
    fn base_decode(
        &self,
        ty: &F::Type,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>>;
}
