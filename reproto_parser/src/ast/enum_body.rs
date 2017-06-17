use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct EnumBody {
    pub name: String,
    pub comment: Vec<String>,
    pub variants: Vec<AstLoc<EnumVariant>>,
    pub members: Vec<AstLoc<Member>>,
}

impl IntoModel for EnumBody {
    type Output = Rc<RpEnumBody>;

    fn into_model(self, pos: &RpPos) -> Result<Rc<RpEnumBody>> {
        let mut variants: Vec<RpLoc<Rc<RpEnumVariant>>> = Vec::new();

        let mut ordinals = utils::OrdinalGenerator::new();

        let (fields, codes, options, match_decl) = utils::members_into_model(pos, self.members)?;

        for variant in self.variants {
            let ordinal = ordinals.next(&variant.ordinal, pos)
                .map_err(|e| ErrorKind::Pos(e.description().into(), pos.clone()))?;

            let (variant, variant_pos) = variant.both();

            let variant = RpLoc::new((variant, ordinal).into_model(pos)?,
                                     (pos.0.clone(), variant_pos.0, variant_pos.1));

            if fields.len() != variant.arguments.len() {
                return Err(ErrorKind::Pos(format!("expected {} arguments", fields.len()),
                                          variant.pos().clone())
                    .into());
            }

            if let Some(other) = variants.iter().find(|v| *v.name == *variant.name) {
                return Err(ErrorKind::EnumVariantConflict(other.name.pos().clone(),
                                                          variant.name.pos().clone())
                    .into());
            }

            variants.push(variant);
        }

        let options = Options::new(pos, options);

        let serialized_as: Option<RpLoc<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        let serialized_as_name = options.find_one_boolean("serialized_as_name")?
            .to_owned()
            .map(|t| t.move_inner())
            .unwrap_or(false);

        let en = RpEnumBody {
            name: self.name,
            comment: self.comment,
            variants: variants,
            fields: fields,
            codes: codes,
            match_decl: match_decl,
            serialized_as: serialized_as,
            serialized_as_name: serialized_as_name,
        };

        Ok(Rc::new(en))
    }
}
