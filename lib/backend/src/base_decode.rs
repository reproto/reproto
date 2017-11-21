//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::RpType;
use errors::*;
use genco::Tokens;

pub trait BaseDecode<'el>
where
    Self: Converter<'el>,
{
    fn base_decode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>>;
}
