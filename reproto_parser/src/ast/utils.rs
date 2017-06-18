use std::collections::HashMap;
use super::*;
use super::errors::*;

type Fields = Vec<RpLoc<RpField>>;
type Codes = Vec<RpLoc<RpCode>>;
type OptionVec = Vec<RpLoc<RpOptionDecl>>;

pub fn code(pos: RpPos, context: String, lines: Vec<String>) -> RpLoc<RpCode> {
    let code = RpCode {
        context: context,
        lines: lines,
    };

    RpLoc::new(code, pos)
}

pub fn members_into_model(path: &Path,
                          members: Vec<AstLoc<Member>>)
                          -> Result<(Fields, Codes, OptionVec, RpMatchDecl)> {
    let mut fields: Vec<RpLoc<RpField>> = Vec::new();
    let mut codes = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut match_decl = RpMatchDecl::new();

    for member in members {
        let pos = (path.to_owned(), member.pos().0, member.pos().1);

        match member.move_inner() {
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
                codes.push(code(pos, context, lines));
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

    Ok((fields, codes, options, match_decl))
}

/// Generate ordinal values.
pub struct OrdinalGenerator {
    next_ordinal: u32,
    ordinals: HashMap<u32, RpPos>,
}

impl OrdinalGenerator {
    pub fn new() -> OrdinalGenerator {
        OrdinalGenerator {
            next_ordinal: 0,
            ordinals: HashMap::new(),
        }
    }

    pub fn next(&mut self, ordinal: &Option<AstLoc<Value>>, path: &Path) -> Result<u32> {
        if let Some(ref ordinal) = *ordinal {
            let pos = (path.to_owned(), ordinal.pos().0, ordinal.pos().1);

            if let Value::Number(ref number) = *ordinal.as_ref() {
                let n: u32 = number.to_u32().ok_or_else(|| ErrorKind::Overflow)?;

                if let Some(other) = self.ordinals.get(&n) {
                    return Err(ErrorKind::Pos("duplicate ordinal".to_owned(), other.clone())
                        .into());
                }

                self.ordinals.insert(n, pos);
                self.next_ordinal = n + 1;
                return Ok(n);
            }

            return Err(ErrorKind::Pos("must be a number".to_owned(), pos).into());
        }

        let o = self.next_ordinal;

        self.next_ordinal += 1;

        if let Some(other) = self.ordinals.get(&o) {
            return Err(ErrorKind::Pos(format!("generated ordinal {} conflicts with existing", o),
                                      other.clone())
                .into());
        }

        Ok(o)
    }
}
