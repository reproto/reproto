//! # Helper trait for building a dynamic-language decode method

use converter::Converter;
use core::errors::*;
use core::{Flavor, RpName, RpType};
use dynamic_converter::DynamicConverter;
use genco::Tokens;

pub trait DynamicDecode<'el, F: 'static>
where
    Self: Converter<'el, F>,
    Self: DynamicConverter<'el, F>,
    F: Flavor<Type = RpType<F>, Name = RpName<F>>,
{
    fn name_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn array_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        inner: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn map_decode(
        &self,
        input: Tokens<'el, Self::Custom>,
        key: Tokens<'el, Self::Custom>,
        value: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn dynamic_decode(
        &self,
        ty: &'el F::Type,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>> {
        use self::RpType::*;

        if self.is_native(ty) {
            return Ok(input);
        }

        let input = match *ty {
            Signed { size: _ } | Unsigned { size: _ } => input,
            Float | Double => input,
            String => input,
            DateTime => input,
            Boolean => input,
            Bytes => input,
            Any => input,
            Name { ref name } => {
                let name = self.convert_type(name)?;
                self.name_decode(input, name)
            }
            Array { ref inner } => {
                let inner_var = self.array_inner_var();
                let inner = self.dynamic_decode(inner, inner_var)?;
                self.array_decode(input, inner)
            }
            Map { ref key, ref value } => {
                let map_key = self.map_key_var();
                let key = self.dynamic_decode(key, map_key)?;
                let map_value = self.map_value_var();
                let value = self.dynamic_decode(value, map_value)?;
                self.map_decode(input, key, value)
            }
        };

        Ok(input)
    }
}
