//! # Helper trait for building a dynamic-language encode method

use super::converter::Converter;
use core::RpType;
use errors::*;
use genco::Tokens;

pub trait BaseEncode<'el>
where
    Self: Converter<'el>,
{
    fn base_encode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>>;
}
