use std::collections::HashSet;
use super::errors::*;
use super::*;

type Fields = Vec<RpLoc<RpField>>;
type Codes = Vec<RpLoc<RpCode>>;
type OptionVec = Vec<RpLoc<RpOptionDecl>>;

pub fn code(pos: &RpPos, ast_pos: ast::Pos, context: String, lines: Vec<String>) -> RpLoc<RpCode> {
    let pos = (pos.0.clone(), ast_pos.0, ast_pos.1);

    let code = RpCode {
        context: context,
        lines: lines,
    };

    RpLoc::new(code, pos)
}

pub fn members_into_model(pos: &RpPos,
                          members: Vec<ast::AstLoc<ast::Member>>)
                          -> Result<(Fields, Codes, OptionVec, RpMatchDecl)> {
    let mut fields: Vec<RpLoc<RpField>> = Vec::new();
    let mut codes = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut match_decl = RpMatchDecl::new();

    for member in members {
        let pos = (pos.0.to_owned(), member.pos.0, member.pos.1);

        match *member {
            ast::Member::Field(field) => {
                let field = field.into_model(&pos)?;

                if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                    return Err(Error::field_conflict(field.name.clone(), pos, other.pos.clone()));
                }

                fields.push(RpLoc::new(field, pos));
            }
            ast::Member::Code(context, lines) => {
                codes.push(code(&pos, member.pos, context, lines));
            }
            ast::Member::Option(option) => {
                options.push(option.into_model(&pos)?);
            }
            ast::Member::Match(m) => {
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

    pub fn next(&mut self, ordinal: &Option<ast::AstLoc<ast::Value>>, pos: &RpPos) -> Result<u32> {
        if let Some(ref ordinal) = *ordinal {
            let pos = (pos.0.to_owned(), ordinal.pos.0, ordinal.pos.1);

            if let ast::Value::Number(ref number) = *ordinal {
                let n: u32 = number.to_u32().ok_or_else(|| ErrorKind::Overflow)?;

                if self.ordinals.contains(&n) {
                    return Err(Error::pos("duplicate ordinal".to_owned(), pos));
                }

                self.ordinals.insert(n);

                self.next_ordinal = n + 1;

                return Ok(n);
            }

            return Err(Error::pos("must be a number".to_owned(), pos));
        }

        let o = self.next_ordinal;

        self.next_ordinal += 1;

        if self.ordinals.contains(&o) {
            return Err(Error::pos(format!("generated ordinal {} conflicts with existing", o),
                                  pos.clone()));
        }

        Ok(o)
    }
}
