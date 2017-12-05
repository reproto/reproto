//! # Helper trait for building a dynamic-language decode method

use base_decode::BaseDecode;
use converter::Converter;
use core::{RpInterfaceBody, RpType};
use dynamic_converter::DynamicConverter;
use errors::*;
use genco::Tokens;

pub trait DynamicDecode<'el>
where
    Self: Converter<'el>,
    Self: DynamicConverter<'el>,
{
    fn assign_type_var(&self, data: &'el str, type_var: &'el str) -> Tokens<'el, Self::Custom>;

    fn check_type_var(
        &self,
        data: &'el str,
        type_var: &'el str,
        name: &'el str,
        type_name: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

    fn raise_bad_type(&self, type_var: &'el str) -> Tokens<'el, Self::Custom>;

    fn new_decode_method(
        &self,
        data: &'el str,
        body: Tokens<'el, Self::Custom>,
    ) -> Tokens<'el, Self::Custom>;

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

    /// Handle the decoding of a datetime object.
    fn datetime_decode(&self, input: Tokens<'el, Self::Custom>) -> Tokens<'el, Self::Custom> {
        input
    }

    fn dynamic_decode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>> {
        use self::RpType::*;

        if self.is_native(ty) {
            return Ok(input);
        }

        let input = match *ty {
            Signed { size: _ } |
            Unsigned { size: _ } => input,
            Float | Double => input,
            String => input,
            DateTime => self.datetime_decode(input),
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

    fn interface_decode_method(
        &self,
        body: &'el RpInterfaceBody,
    ) -> Result<Tokens<'el, Self::Custom>> {
        let mut decode_body: Tokens<Self::Custom> = Tokens::new();

        let data = "data";
        let type_var = "f_type";
        decode_body.push(self.assign_type_var(data, type_var));

        for sub_type in body.sub_types.values() {
            let type_name = self.convert_type(&sub_type.name).map_err(|e| {
                ErrorKind::Pos(format!("{}", e), sub_type.pos().into())
            })?;

            decode_body.push(self.check_type_var(
                data,
                type_var,
                sub_type.name(),
                type_name,
            ));
        }

        decode_body.push(self.raise_bad_type(type_var));

        Ok(self.new_decode_method(
            data,
            decode_body.join_line_spacing(),
        ))
    }
}

/// Dynamic decode is a valid decoding mechanism
impl<'el, T> BaseDecode<'el> for T
where
    T: DynamicDecode<'el>,
{
    fn base_decode(
        &self,
        ty: &'el RpType,
        input: Tokens<'el, Self::Custom>,
    ) -> Result<Tokens<'el, Self::Custom>> {
        self.dynamic_decode(ty, input)
    }
}
