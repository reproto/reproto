//! # Helper trait to implement match-based decoding

use core::*;
use super::decode::Decode;
use super::errors::*;
use super::value_builder::{ValueBuilder, ValueBuilderEnv};
use super::variables::Variables;

pub trait MatchDecode
    where Self: ValueBuilder<Output = <Self as Decode>::Output>,
          Self: Decode
{
    type Elements;

    fn decode_by_value(&self,
                       type_id: &RpTypeId,
                       match_decl: &RpMatchDecl,
                       data: <Self as Decode>::Output)
                       -> Result<Option<Self::Elements>> {
        if match_decl.by_value.is_empty() {
            return Ok(None);
        }

        let variables = Variables::new();

        let mut elements = Elements::new();

        for &(ref value, ref result) in &match_decl.by_value {
            let value = self.value(&ValueBuilderEnv {
                    value: value,
                    package: &type_id.package,
                    ty: None,
                    variables: &variables,
                })?;

            let result = self.value(&ValueBuilderEnv {
                    value: result,
                    package: &type_id.package,
                    ty: Some(&RpType::Name(type_id.name.clone())),
                    variables: &variables,
                })?;

            let mut value_body = Elements::new();
            value_body.push(stmt!["if ", &data, " == ", &value, ":"]);
            value_body.push_nested(stmt!["return ", &result]);

            elements.push(value_body);
        }

        Ok(Some(elements.join(ElementSpec::Spacing)))
    }

    fn decode_by_type(&self,
                      type_id: &RpTypeId,
                      match_decl: &RpMatchDecl,
                      data: &<Self as Decode>::Output)
                      -> Result<Option<Self::Elements>> {
        if match_decl.by_type.is_empty() {
            return Ok(None);
        }

        let mut elements = Elements::new();

        for &(ref kind, ref result) in &match_decl.by_type {
            let variable = result.0.name.clone();

            let mut variables = Variables::new();
            variables.insert(variable.clone(), &result.0.ty);

            let decode_stmt = self.decode(type_id, &result.1.pos, &result.0.ty, data)?;

            let result = self.value(&ValueBuilderEnv {
                    value: &result.1,
                    package: &type_id.package,
                    ty: Some(&RpType::Name(type_id.name.clone())),
                    variables: &variables,
                })?;

            let check = match *kind {
                RpMatchKind::Any => stmt!["true"],
                RpMatchKind::Object => stmt![&self.isinstance, "(", &data, ", ", &self.dict, ")"],
                RpMatchKind::Array => stmt![&self.isinstance, "(", &data, ", ", &self.list, ")"],
                RpMatchKind::String => {
                    stmt![&self.isinstance, "(", &data, ", ", &self.basestring, ")"]
                }
                RpMatchKind::Boolean => {
                    stmt![&self.isinstance, "(", &data, ", ", &self.boolean, ")"]
                }
                RpMatchKind::Number => stmt![&self.isinstance, "(", &data, ", ", &self.number, ")"],
            };

            let mut value_body = Elements::new();

            value_body.push(stmt!["if ", check, ":"]);
            value_body.push_nested(stmt![&variable, " = ", decode_stmt]);
            value_body.push_nested(stmt!["return ", &result]);

            elements.push(value_body);
        }

        Ok(Some(elements.join(ElementSpec::Spacing)))
    }
}
