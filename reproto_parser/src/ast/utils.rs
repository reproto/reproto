use std::collections::HashSet;
use super::*;
use super::errors::*;

type Fields = Vec<RpLoc<RpField>>;
type Codes = Vec<RpLoc<RpCode>>;
type OptionVec = Vec<RpLoc<RpOptionDecl>>;

pub fn code(pos: &RpPos, ast_pos: Pos, context: String, lines: Vec<String>) -> RpLoc<RpCode> {
    let pos = (pos.0.clone(), ast_pos.0, ast_pos.1);

    let code = RpCode {
        context: context,
        lines: lines,
    };

    RpLoc::new(code, pos)
}

pub fn members_into_model(pos: &RpPos,
                          members: Vec<AstLoc<Member>>)
                          -> Result<(Fields, Codes, OptionVec, RpMatchDecl)> {
    let mut fields: Vec<RpLoc<RpField>> = Vec::new();
    let mut codes = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut match_decl = RpMatchDecl::new();

    for member in members {
        let pos = (pos.0.to_owned(), member.pos.0, member.pos.1);

        match member.inner {
            Member::Field(field) => {
                let field = field.into_model(&pos)?;

                if let Some(other) = fields.iter()
                    .find(|f| f.name() == field.name() || f.ident() == field.ident()) {
                    return Err(ErrorKind::FieldConflict(field.ident().to_owned(),
                                                        pos,
                                                        other.pos.clone())
                        .into());
                }

                fields.push(RpLoc::new(field, pos));
            }
            Member::Code(context, lines) => {
                codes.push(code(&pos, member.pos, context, lines));
            }
            Member::Option(option) => {
                options.push(option.into_model(&pos)?);
            }
            Member::Match(m) => {
                for member in m.members {
                    match_decl.push(member.into_model(&pos)?)?;
                }
            }
        }
    }

    Ok((fields, codes, options, match_decl))
}

/// Generate ordinal values.
pub struct OrdinalGenerator {
    next_ordinal: u32,
    ordinals: HashSet<u32>,
}

impl OrdinalGenerator {
    pub fn new() -> OrdinalGenerator {
        OrdinalGenerator {
            next_ordinal: 0,
            ordinals: HashSet::new(),
        }
    }

    pub fn next(&mut self, ordinal: &Option<AstLoc<Value>>, pos: &RpPos) -> Result<u32> {
        if let Some(ref ordinal) = *ordinal {
            let pos = (pos.0.to_owned(), ordinal.pos.0, ordinal.pos.1);

            if let Value::Number(ref number) = ordinal.inner {
                let n: u32 = number.to_u32().ok_or_else(|| ErrorKind::Overflow)?;

                if self.ordinals.contains(&n) {
                    return Err(ErrorKind::Pos("duplicate ordinal".to_owned(), pos).into());
                }

                self.ordinals.insert(n);

                self.next_ordinal = n + 1;

                return Ok(n);
            }

            return Err(ErrorKind::Pos("must be a number".to_owned(), pos).into());
        }

        let o = self.next_ordinal;

        self.next_ordinal += 1;

        if self.ordinals.contains(&o) {
            return Err(ErrorKind::Pos(format!("generated ordinal {} conflicts with existing", o),
                                      pos.clone())
                .into());
        }

        Ok(o)
    }
}
