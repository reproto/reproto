//! # Helper trait for building a dynamic-language decode method

use base_decode::BaseDecode;
use codeviz_common::Element;
use container::Container;
use converter::Converter;
use core::{Loc, RpInterfaceBody, RpName, RpType};
use dynamic_converter::DynamicConverter;
use errors::*;

pub trait DynamicDecode
where
    Self: Converter,
    Self: DynamicConverter,
{
    type Method;

    fn assign_type_var(&self, data: &Self::Stmt, type_var: &Self::Stmt) -> Self::Stmt;

    fn check_type_var(
        &self,
        data: &Self::Stmt,
        type_var: &Self::Stmt,
        name: &Loc<String>,
        type_name: &Self::Type,
    ) -> Self::Elements;

    fn raise_bad_type(&self, type_var: &Self::Stmt) -> Self::Stmt;

    fn new_decode_method(&self, data: &Self::Stmt, body: Self::Elements) -> Self::Method;

    fn name_decode(&self, input: &Self::Stmt, name: Self::Type) -> Self::Stmt;

    fn array_decode(&self, input: &Self::Stmt, inner: Self::Stmt) -> Self::Stmt;

    fn map_decode(&self, input: &Self::Stmt, key: Self::Stmt, value: Self::Stmt) -> Self::Stmt;

    fn dynamic_decode(&self, name: &RpName, ty: &RpType, input: &Self::Stmt) -> Result<Self::Stmt> {
        if self.is_native(ty) {
            return Ok(input.clone());
        }

        let input = match *ty {
            RpType::Signed { size: _ } |
            RpType::Unsigned { size: _ } => input.clone(),
            RpType::Float | RpType::Double => input.clone(),
            RpType::String => input.clone(),
            RpType::Any => input.clone(),
            RpType::Boolean => input.clone(),
            RpType::Name { ref name } => {
                let name = self.convert_type(name)?;
                self.name_decode(input, name)
            }
            RpType::Array { ref inner } => {
                let inner_var = self.array_inner_var();
                let inner = self.dynamic_decode(name, inner, &inner_var)?;
                self.array_decode(input, inner)
            }
            RpType::Map { ref key, ref value } => {
                let map_key = self.map_key_var();
                let key = self.dynamic_decode(name, key, &map_key)?;
                let map_value = self.map_value_var();
                let value = self.dynamic_decode(name, value, &map_value)?;
                self.map_decode(input, key, value)
            }
            ref ty => return Err(format!("type `{}` not supported", ty).into()),
        };

        Ok(input)
    }

    fn interface_decode_method(
        &self,
        name: &RpName,
        body: &RpInterfaceBody,
    ) -> Result<Self::Method> {
        let data = self.new_var("data");

        let mut decode_body = Self::Elements::new();

        let type_var = self.new_var("f_type");
        decode_body.push(&self.assign_type_var(&data, &type_var));

        for (_, ref sub_type) in &body.sub_types {
            for sub_type_name in &sub_type.names {
                let name = name.extend(sub_type_name.as_ref().clone());

                let type_name = self.convert_type(&name).map_err(|e| {
                    ErrorKind::Pos(format!("{}", e), sub_type.pos().into())
                })?;

                decode_body.push(&self.check_type_var(
                    &data,
                    &type_var,
                    &sub_type_name,
                    &type_name,
                ));
            }
        }

        decode_body.push(&self.raise_bad_type(&type_var));

        Ok(self.new_decode_method(
            &data,
            decode_body.join(Element::Spacing),
        ))
    }
}

/// Dynamic decode is a valid decoding mechanism
impl<T> BaseDecode for T
where
    T: DynamicDecode,
{
    fn base_decode(&self, name: &RpName, ty: &RpType, input: &Self::Stmt) -> Result<Self::Stmt> {
        self.dynamic_decode(name, ty, input)
    }
}
