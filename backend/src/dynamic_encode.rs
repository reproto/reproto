//! # Helper trait for building a dynamic-language encode method

use base_encode::BaseEncode;
use core::RpType;
use dynamic_converter::DynamicConverter;
use errors::*;
use genco::Tokens;

pub trait DynamicEncode<'el>
where
    Self: DynamicConverter<'el>,
{
    fn name_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn array_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        inner: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn map_encode(
        &self,
        input: Tokens<'el, Self::Custom>,
        key: Tokens<'el, Self::Custom>,
        value: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn dynamic_encode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>> {
        if self.is_native(ty) {
            return Ok(input);
        }

        let stmt = match *ty {
            RpType::Signed { size: _ } |
            RpType::Unsigned { size: _ } => input,
            RpType::Float | RpType::Double => input,
            RpType::String => input,
            RpType::Any => input,
            RpType::Boolean => input,
            RpType::Name { ref name } => {
                let name = self.convert_type(name)?;
                self.name_encode(input, name)
            }
            RpType::Array { ref inner } => {
                let v = self.array_inner_var();
                let inner = self.dynamic_encode(inner, v)?;
                self.array_encode(input, inner)
            }
            RpType::Map { ref key, ref value } => {
                let map_key = self.map_key_var();
                let key = self.dynamic_encode(key, map_key)?;
                let map_value = self.map_value_var();
                let value = self.dynamic_encode(value, map_value)?;
                self.map_encode(input, key, value)
            }
            _ => input,
        };

        Ok(stmt)
    }
}

/// Dynamic encode is a valid decoding mechanism
impl<'el, T> BaseEncode<'el> for T
where
    T: DynamicEncode<'el>,
{
    fn base_encode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>> {
        self.dynamic_encode(ty, input)
    }
}
