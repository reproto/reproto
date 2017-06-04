//! # Helper trait for building a dynamic-language decode method

use core::*;
use super::converter::Converter;
use super::errors::*;

pub trait DynamicDecode
    where Self: Converter
{
    type Output;

    /// If the type deeply compatible already with the language and need no conversion.
    fn is_native(&self, &RpType) -> bool;

    fn map_key_var(&self) -> Self::Output;

    fn map_value_var(&self) -> Self::Output;

    fn array_inner_var(&self) -> Self::Output;

    fn name_decode(&self, input: Self::Output, name: Self::Type) -> Self::Output;

    fn array_decode(&self, input: Self::Output, inner: Self::Output) -> Self::Output;

    fn map_decode(&self,
                  input: Self::Output,
                  key: Self::Output,
                  value: Self::Output)
                  -> Self::Output;

    fn decode<S>(&self,
                 type_id: &RpTypeId,
                 pos: &RpPos,
                 ty: &RpType,
                 input: S)
                 -> Result<Self::Output>
        where S: Into<Self::Output>
    {
        let input = input.into();

        // TODO: do not skip conversion if strict type checking is enabled
        if self.is_native(ty) {
            return Ok(input);
        }

        let input = match *ty {
            RpType::Signed(_) |
            RpType::Unsigned(_) => input,
            RpType::Float | RpType::Double => input,
            RpType::String => input,
            RpType::Any => input,
            RpType::Boolean => input,
            RpType::Name(ref name) => {
                let name = self.convert_type(pos, &type_id.with_name(name.clone()))?;
                self.name_decode(input, name)
            }
            RpType::Array(ref inner) => {
                let inner_var = self.array_inner_var();
                let inner = self.decode(type_id, pos, inner, inner_var)?;
                self.array_decode(input, inner)
            }
            RpType::Map(ref key, ref value) => {
                let map_key = self.map_key_var();
                let key = self.decode(type_id, pos, key, map_key)?;
                let map_value = self.map_value_var();
                let value = self.decode(type_id, pos, value, map_value)?;
                self.map_decode(input, key, value)
            }
            ref ty => {
                return Err(Error::pos(format!("type `{}` not supported", ty).into(), pos.clone()))
            }
        };

        Ok(input)
    }
}
