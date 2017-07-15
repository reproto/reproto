//! # Helper trait to implement match-based decoding

use base_decode::BaseDecode;
use container::Container;
use converter::Converter;
use core::{RpByTypeMatch, RpMatchDecl, RpMatchKind, RpObject, RpType, RpTypeId, RpValue};
use errors::*;
use value_builder::{ObjectContext, ValueBuilder, ValueContext};
use variables::Variables;

pub trait MatchDecode
    where Self: ValueBuilder,
          Self: BaseDecode,
          Self: Converter
{
    fn match_value(&self,
                   data: &Self::Stmt,
                   value: &RpValue,
                   value_stmt: Self::Stmt,
                   result: &RpObject,
                   result_stmt: Self::Stmt)
                   -> Result<Self::Elements>;

    fn match_type(&self,
                  type_id: &RpTypeId,
                  data: &Self::Stmt,
                  kind: &RpMatchKind,
                  variable: &str,
                  decode: Self::Stmt,
                  result: Self::Stmt,
                  value: &RpByTypeMatch)
                  -> Result<Self::Elements>;

    fn decode_by_value(&self,
                       type_id: &RpTypeId,
                       match_decl: &RpMatchDecl,
                       data: &Self::Stmt)
                       -> Result<Option<Self::Elements>> {
        if match_decl.by_value.is_empty() {
            return Ok(None);
        }

        let variables = Variables::new();

        let mut elements = Self::Elements::new();

        for &(ref value, ref result) in &match_decl.by_value {
            let value_stmt =
                self.value(ValueContext::new(&type_id.package, &variables, &value, None))?;

            let result_stmt = self.object(ObjectContext::new(&type_id.package,
                                           &variables,
                                           &result.object,
                                           Some(&RpType::Name { name: type_id.name.clone() })))?;

            elements.push(&self.match_value(data, value, value_stmt, &result.object, result_stmt)?);
        }

        Ok(Some(elements))
    }

    fn decode_by_type(&self,
                      type_id: &RpTypeId,
                      match_decl: &RpMatchDecl,
                      data: &Self::Stmt)
                      -> Result<Option<Self::Elements>> {
        if match_decl.by_type.is_empty() {
            return Ok(None);
        }

        let mut elements = Self::Elements::new();

        for &(ref kind, ref result) in &match_decl.by_type {
            let variable = &result.variable.name;

            let mut variables = Variables::new();
            variables.insert(variable.clone(), &result.variable.ty);

            let decode =
                self.base_decode(type_id, result.variable.pos(), &result.variable.ty, data)?;

            let result_value = self.object(ObjectContext::new(&type_id.package,
                                           &variables,
                                           &result.object,
                                           Some(&RpType::Name { name: type_id.name.clone() })))?;

            elements.push(&self.match_type(type_id, data, kind, variable, decode, result_value, result)?);
        }

        Ok(Some(elements))
    }
}
