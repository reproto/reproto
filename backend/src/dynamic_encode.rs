//! # Helper trait for building a dynamic-language encode method

use base_encode::BaseEncode;
use core::{Pos, RpName, RpType};
use dynamic_converter::DynamicConverter;
use errors::*;

pub trait DynamicEncode
where
    Self: DynamicConverter,
{
    fn name_encode(&self, input: &Self::Stmt, name: Self::Type) -> Self::Stmt;

    fn array_encode(&self, input: &Self::Stmt, inner: Self::Stmt) -> Self::Stmt;

    fn map_encode(&self, input: &Self::Stmt, key: Self::Stmt, value: Self::Stmt) -> Self::Stmt;

    fn encode(
        &self,
        name: &RpName,
        pos: &Pos,
        ty: &RpType,
        input: &Self::Stmt,
    ) -> Result<Self::Stmt> {
        if self.is_native(ty) {
            return Ok(input.clone());
        }

        let stmt = match *ty {
            RpType::Signed { size: _ } |
            RpType::Unsigned { size: _ } => input.clone(),
            RpType::Float | RpType::Double => input.clone(),
            RpType::String => input.clone(),
            RpType::Any => input.clone(),
            RpType::Boolean => input.clone(),
            RpType::Name { ref name } => {
                let name = self.convert_type(pos, name)?;
                self.name_encode(&input, name)
            }
            RpType::Array { ref inner } => {
                let v = self.array_inner_var();
                let inner = self.encode(name, pos, inner, &v)?;
                self.array_encode(input, inner)
            }
            RpType::Map { ref key, ref value } => {
                let map_key = self.map_key_var();
                let key = self.encode(name, pos, key, &map_key)?;
                let map_value = self.map_value_var();
                let value = self.encode(name, pos, value, &map_value)?;
                self.map_encode(input, key, value)
            }
            _ => input.clone(),
        };

        Ok(stmt)
    }
}

/// Dynamic encode is a valid decoding mechanism
impl<T> BaseEncode for T
where
    T: DynamicEncode,
{
    type Stmt = T::Stmt;

    fn base_encode(
        &self,
        name: &RpName,
        pos: &Pos,
        ty: &RpType,
        input: &Self::Stmt,
    ) -> Result<Self::Stmt> {
        DynamicEncode::encode(self, name, pos, ty, input)
    }
}
