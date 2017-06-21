use std::rc::Rc;
use super::*;
use super::errors::*;

/// Sub-types in interface declarations.
#[derive(Debug)]
pub struct SubType<'a> {
    pub name: &'a str,
    pub comment: Vec<&'a str>,
    pub members: Vec<AstLoc<Member<'a>>>,
}

impl<'a> IntoModel for SubType<'a> {
    type Output = Rc<RpSubType>;

    fn into_model(self, path: &Path) -> Result<Rc<RpSubType>> {
        let mut fields: Vec<RpLoc<RpField>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();
        let mut match_decl = RpMatchDecl::new();

        for member in self.members {
            let (member, member_pos) = member.both();
            let pos = (path.to_owned(), member_pos.0, member_pos.1);

            match member {
                Member::Field(field) => {
                    let field = field.into_model(path)?;

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
                    codes.push(utils::code(pos, context, lines));
                }
                Member::Option(option) => {
                    options.push(option.into_model(path)?);
                }
                Member::Match(m) => {
                    for member in m.members {
                        match_decl.push(member.into_model(path)?)?;
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
