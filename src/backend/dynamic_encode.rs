//! # Helper trait for building a dynamic-language encode method

use core::*;
use super::converter::Converter;
use super::dynamic_converter::DynamicConverter;
use super::encode::Encode;
use super::errors::*;

pub trait DynamicEncode
    where Self: Converter + DynamicConverter<DynamicConverterStmt = <Self as DynamicEncode>::Stmt>
{
    type Stmt: Clone;

    fn name_encode(&self, input: &Self::Stmt, name: Self::Type) -> Self::Stmt;

    fn array_encode(&self, input: &Self::Stmt, inner: Self::Stmt) -> Self::Stmt;

    fn map_encode(&self, input: &Self::Stmt, key: Self::Stmt, value: Self::Stmt) -> Self::Stmt;

    fn encode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt> {
        if self.is_native(ty) {
            return Ok(input.clone());
        }

        let stmt = match *ty {
            RpType::Signed(_) |
            RpType::Unsigned(_) => input.clone(),
            RpType::Float | RpType::Double => input.clone(),
            RpType::String => input.clone(),
            RpType::Any => input.clone(),
            RpType::Boolean => input.clone(),
            RpType::Name(ref name) => {
                let name = self.convert_type(pos, &type_id.with_name(name.clone()))?;
                self.name_encode(&input, name)
            }
            RpType::Array(ref inner) => {
                let v = self.array_inner_var();
                let inner = self.encode(type_id, pos, inner, &v)?;
                self.array_encode(input, inner)
            }
            RpType::Map(ref key, ref value) => {
                let map_key = self.map_key_var();
                let key = self.encode(type_id, pos, key, &map_key)?;
                let map_value = self.map_value_var();
                let value = self.encode(type_id, pos, value, &map_value)?;
                self.map_encode(input, key, value)
            }
            _ => input.clone(),
        };

        Ok(stmt)
    }
}

/// Dynamic encode is a valid decoding mechanism
impl<T> Encode for T
    where T: DynamicEncode
{
    type Stmt = T::Stmt;

    fn encode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt> {
        DynamicEncode::encode(self, type_id, pos, ty, input)
    }
}
