use std::rc::Rc;
use super::*;
use super::errors::*;

#[derive(Debug)]
pub struct EnumBody<'a> {
    pub name: &'a str,
    pub comment: Vec<&'a str>,
    pub variants: Vec<AstLoc<EnumVariant<'a>>>,
    pub members: Vec<AstLoc<Member<'a>>>,
}

impl<'a> IntoModel for EnumBody<'a> {
    type Output = Rc<RpEnumBody>;

    fn into_model(self, path: &Path) -> Result<Rc<RpEnumBody>> {
        let mut variants: Vec<RpLoc<Rc<RpEnumVariant>>> = Vec::new();

        let mut ordinals = utils::OrdinalGenerator::new();

        let (fields, codes, options, match_decl) = utils::members_into_model(path, self.members)?;

        for variant in self.variants {
            let (variant, variant_pos) = variant.both();
            let variant_pos = (path.to_owned(), variant_pos.0, variant_pos.1);

            let ordinal = ordinals.next(&variant.ordinal, path)
                .chain_err(|| {
                    ErrorKind::Pos("failed to generate ordinal".to_owned(), variant_pos.clone())
                })?;

            if fields.len() != variant.arguments.len() {
                return Err(ErrorKind::Pos(format!("expected {} arguments", fields.len()),
                                          variant_pos)
                    .into());
            }

            let variant = RpLoc::new((variant, ordinal).into_model(path)?, variant_pos);

            if let Some(other) = variants.iter().find(|v| *v.name == *variant.name) {
                return Err(ErrorKind::EnumVariantConflict(other.name.pos().clone(),
                                                          variant.name.pos().clone())
                    .into());
            }

            variants.push(variant);
        }

        let options = Options::new(options);

        let serialized_as: Option<RpLoc<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        let serialized_as_name = options.find_one_boolean("serialized_as_name")?
            .to_owned()
            .map(|t| t.move_inner())
            .unwrap_or(false);

        let en = RpEnumBody {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
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
