//! # Helper trait for building a dynamic-language decode method

use core::*;
use super::converter::Converter;
use super::decode::Decode;
use super::dynamic_converter::DynamicConverter;
use super::errors::*;

pub trait DynamicDecode
    where Self: Converter + DynamicConverter<DynamicConverterStmt = <Self as DynamicDecode>::Stmt>
{
    type Stmt: Clone;

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
            RpType::Signed(_) |
            RpType::Unsigned(_) => input.clone(),
            RpType::Float | RpType::Double => input.clone(),
            RpType::String => input.clone(),
            RpType::Any => input.clone(),
            RpType::Boolean => input.clone(),
            RpType::Name(ref name) => {
                let name = self.convert_type(pos, &type_id.with_name(name.clone()))?;
                self.name_decode(input, name)
            }
            RpType::Array(ref inner) => {
                let inner_var = self.array_inner_var();
                let inner = self.decode(type_id, pos, inner, &inner_var)?;
                self.array_decode(input, inner)
            }
            RpType::Map(ref key, ref value) => {
                let map_key = self.map_key_var();
                let key = self.decode(type_id, pos, key, &map_key)?;
                let map_value = self.map_value_var();
                let value = self.decode(type_id, pos, value, &map_value)?;
                self.map_decode(input, key, value)
            }
            ref ty => {
                return Err(Error::pos(format!("type `{}` not supported", ty).into(), pos.clone()))
            }
        };

        Ok(input)
    }
}

/// Dynamic decode is a valid decoding mechanism
impl<T> Decode for T
    where T: DynamicDecode
{
    type Stmt = T::Stmt;

    fn decode(&self,
              type_id: &RpTypeId,
              pos: &RpPos,
              ty: &RpType,
              input: &Self::Stmt)
              -> Result<Self::Stmt> {
        DynamicDecode::decode(self, type_id, pos, ty, input)
    }
}
