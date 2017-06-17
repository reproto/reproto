//! # Helper trait for building a dynamic-language decode method

use codeviz::common::Element;
use core::*;
use super::container::Container;
use super::converter::Converter;
use super::decode::Decode;
use super::dynamic_converter::DynamicConverter;
use super::errors::*;
use super::match_decode::MatchDecode;

pub trait DynamicDecode
    where Self: Converter,
          Self: DynamicConverter,
          Self: MatchDecode
{
    type Method;

    fn assign_type_var(&self, data: &Self::Stmt, type_var: &Self::Stmt) -> Self::Stmt;

    fn check_type_var(&self,
                      data: &Self::Stmt,
                      type_var: &Self::Stmt,
                      name: &RpLoc<String>,
                      type_name: &Self::Type)
                      -> Self::Elements;

    fn raise_bad_type(&self, type_var: &Self::Stmt) -> Self::Stmt;

    fn new_decode_method(&self, data: &Self::Stmt, body: Self::Elements) -> Self::Method;

    fn name_decode(&self, input: &Self::Stmt, name: Self::Type) -> Self::Stmt;

    fn array_decode(&self, input: &Self::Stmt, inner: Self::Stmt) -> Self::Stmt;

    fn map_decode(&self, input: &Self::Stmt, key: Self::Stmt, value: Self::Stmt) -> Self::Stmt;

    fn decode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt> {
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
                let name = self.convert_type(pos, &type_id.with_name(name.clone()))?;
                self.name_decode(input, name)
            }
            RpType::Array { ref inner } => {
                let inner_var = self.array_inner_var();
                let inner = DynamicDecode::decode(self, type_id, pos, inner, &inner_var)?;
                self.array_decode(input, inner)
            }
            RpType::Map { ref key, ref value } => {
                let map_key = self.map_key_var();
                let key = DynamicDecode::decode(self, type_id, pos, key, &map_key)?;
                let map_value = self.map_value_var();
                let value = DynamicDecode::decode(self, type_id, pos, value, &map_value)?;
                self.map_decode(input, key, value)
            }
            ref ty => {
                return Err(Error::pos(format!("type `{}` not supported", ty).into(), pos.clone()))
            }
        };

        Ok(input)
    }

    fn interface_decode_method(&self,
                               type_id: &RpTypeId,
                               body: &RpInterfaceBody)
                               -> Result<Self::Method> {
        let data = self.new_var("data");

        let mut decode_body = Self::Elements::new();

        if let Some(by_value) = self.decode_by_value(type_id, &body.match_decl, &data)? {
            decode_body.push(&by_value);
        }

        if let Some(by_type) = self.decode_by_type(type_id, &body.match_decl, &data)? {
            decode_body.push(&by_type);
        }

        let type_var = self.new_var("f_type");
        decode_body.push(&self.assign_type_var(&data, &type_var));

        for (_, ref sub_type) in &body.sub_types {
            for name in &sub_type.names {
                let type_id = type_id.extend(sub_type.name.clone());
                let type_name = self.convert_type(sub_type.pos(), &type_id)?;
                decode_body.push(&self.check_type_var(&data, &type_var, name, &type_name));
            }
        }

        decode_body.push(&self.raise_bad_type(&type_var));

        Ok(self.new_decode_method(&data, decode_body.join(Element::Spacing)))
    }
}

/// Dynamic decode is a valid decoding mechanism
impl<T> Decode for T
    where T: DynamicDecode
{
    fn decode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt> {
        DynamicDecode::decode(self, type_id, pos, ty, input)
    }
}
