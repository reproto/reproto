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

pub fn members_into_model(members: Vec<RpLoc<Member>>)
                          -> Result<(Fields, Codes, OptionVec, RpMatchDecl)> {
    let mut fields: Vec<RpLoc<RpField>> = Vec::new();
    let mut codes = Vec::new();
    let mut options: Vec<RpLoc<RpOptionDecl>> = Vec::new();
    let mut match_decl = RpMatchDecl::new();

    for member in members {
        let (value, pos) = member.both();

        match value {
            Member::Field(field) => {
                let field = field.into_model()?;

                if let Some(other) = fields.iter()
                    .find(|f| f.name() == field.name() || f.ident() == field.ident()) {
                    return Err(ErrorKind::FieldConflict(field.ident().to_owned(),
                                                        pos.into(),
                                                        other.pos().into())
                        .into());
                }

                fields.push(RpLoc::new(field, pos.into()));
            }
            Member::Code(context, lines) => {
                codes.push(code(pos.into(), context.to_owned(), lines));
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

    pub fn next(&mut self, ordinal: &Option<RpLoc<Value>>) -> Result<u32> {
        if let Some(ref ordinal) = *ordinal {
            let pos = ordinal.pos();

            if let Value::Number(ref number) = *ordinal.as_ref() {
                let n: u32 = number.to_u32().ok_or_else(|| ErrorKind::Overflow(pos.into()))?;

                if let Some(other) = self.ordinals.get(&n) {
                    return Err(ErrorKind::Pos("duplicate ordinal".to_owned(), other.into()).into());
                }

                self.ordinals.insert(n, pos.clone());
                self.next_ordinal = n + 1;
                return Ok(n);
            }

            return Err(ErrorKind::Pos("must be a number".to_owned(), pos.into()).into());
        }

        let o = self.next_ordinal;

        self.next_ordinal += 1;

        if let Some(other) = self.ordinals.get(&o) {
            return Err(ErrorKind::Pos(format!("generated ordinal {} conflicts with existing", o),
                                      other.into())
                .into());
        }

        Ok(o)
    }
}
