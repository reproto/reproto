use parser::ast;
use std::rc::Rc;
use super::errors::*;
use super::into_model::IntoModel;
use super::merge::Merge;
use super::options::Options;
use super::rp_code::RpCode;
use super::rp_enum_value::RpEnumValue;
use super::rp_field::RpField;
use super::rp_loc::{RpLoc, RpPos};
use super::rp_match_decl::RpMatchDecl;
use super::utils;

#[derive(Debug, Clone)]
pub struct RpEnumBody {
    pub name: String,
    pub values: Vec<RpLoc<Rc<RpEnumValue>>>,
    pub fields: Vec<RpLoc<RpField>>,
    pub codes: Vec<RpLoc<RpCode>>,
    pub match_decl: RpMatchDecl,
    pub serialized_as: Option<RpLoc<String>>,
    pub serialized_as_name: bool,
}

impl IntoModel for ast::EnumBody {
    type Output = Rc<RpEnumBody>;

    fn into_model(self, pos: &RpPos) -> Result<Rc<RpEnumBody>> {
        let mut values: Vec<RpLoc<Rc<RpEnumValue>>> = Vec::new();

        let mut ordinals = utils::OrdinalGenerator::new();

        let (fields, codes, options, match_decl) = utils::members_into_model(pos, self.members)?;

        for value in self.values {
            let ordinal = ordinals.next(&value.ordinal, pos)
                .map_err(|e| Error::pos(e.description().into(), pos.clone()))?;

            let value = RpLoc::new((value.inner, ordinal).into_model(pos)?,
                                   (pos.0.clone(), value.pos.0, value.pos.1));

            if fields.len() != value.arguments.len() {
                return Err(Error::pos(format!("expected {} arguments", fields.len()),
                                      value.pos.clone()));
            }

            if let Some(other) = values.iter().find(|v| *v.name == *value.name) {
                return Err(ErrorKind::EnumValueConflict(other.name.pos.clone(),
                                                        value.name.pos.clone())
                    .into());
            }

            /// need to tack on an ordinal value.
            values.push(value);
        }

        let options = Options::new(pos, options);

        let serialized_as: Option<RpLoc<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        let serialized_as_name = options.find_one_boolean("serialized_as_name")?
            .to_owned()
            .map(|t| t.inner)
            .unwrap_or(false);

        let en = RpEnumBody {
            name: self.name,
            values: values,
            fields: fields,
            codes: codes,
            match_decl: match_decl,
            serialized_as: serialized_as,
            serialized_as_name: serialized_as_name,
        };

        Ok(Rc::new(en))
    }
}

impl Merge for RpEnumBody {
    fn merge(&mut self, source: RpEnumBody) -> Result<()> {
        self.codes.merge(source.codes)?;
        Ok(())
    }
}
