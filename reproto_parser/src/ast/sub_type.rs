use std::rc::Rc;
use super::*;
use super::errors::*;

/// Sub-types in interface declarations.
#[derive(Debug)]
pub struct SubType<'input> {
    pub name: &'input str,
    pub comment: Vec<&'input str>,
    pub members: Vec<AstLoc<'input, Member<'input>>>,
}

impl<'input> IntoModel for SubType<'input> {
    type Output = Rc<RpSubType>;

    fn into_model(self) -> Result<Rc<RpSubType>> {
        let mut fields: Vec<RpLoc<RpField>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();
        let mut match_decl = RpMatchDecl::new();

        for member in self.members {
            let (member, member_pos) = member.both();
            let pos = (member_pos.0.to_owned(), member_pos.1, member_pos.2);

            match member {
                Member::Field(field) => {
                    let field = field.into_model()?;

                    if let Some(other) = fields.iter()
                        .find(|f| f.name() == field.name() || f.ident() == field.ident()) {
                        return Err(ErrorKind::FieldConflict(field.ident().to_owned(),
                                                            pos,
                                                            other.pos().clone())
                            .into());
                    }

                    fields.push(RpLoc::new(field, pos));
                }
                Member::Code(context, lines) => {
                    codes.push(utils::code(pos, context.to_owned(), lines));
                }
                Member::Option(option) => {
                    options.push(option.into_model()?);
                }
                Member::Match(m) => {
                    for member in m.members {
                        match_decl.push(member.into_model()?)?;
                    }
                }
            }
        }

        let options = Options::new(options);

        let names = options.find_all_strings("name")?;

        let sub_type = RpSubType {
            name: self.name.to_owned(),
            comment: self.comment.into_iter().map(ToOwned::to_owned).collect(),
            fields: fields,
            codes: codes,
            names: names,
            match_decl: match_decl,
        };

        Ok(Rc::new(sub_type))
    }
}
