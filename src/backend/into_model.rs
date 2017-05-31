//! Implementations for converting asts into models.
use parser::ast;
use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map;
use super::errors::*;
use super::merge::Merge;
use super::models::*;
use super::options::Options;
use with_prefix::WithPrefix;

fn code(pos: &Pos, ast_pos: ast::Pos, context: String, lines: Vec<String>) -> Token<Code> {
    let pos = (pos.0.clone(), ast_pos.0, ast_pos.1);

    let code = Code {
        context: context,
        lines: lines,
    };

    Token::new(code, pos)
}

type Fields = Vec<Token<Field>>;
type Codes = Vec<Token<Code>>;
type OptionVec = Vec<Token<OptionDecl>>;

fn members_into_model(pos: &Pos,
                      members: Vec<ast::Token<ast::Member>>)
                      -> Result<(Fields, Codes, OptionVec)> {
    let mut fields: Vec<Token<Field>> = Vec::new();
    let mut codes = Vec::new();
    let mut options = Vec::new();

    for member in members {
        let pos = (pos.0.to_owned(), member.pos.0, member.pos.1);

        match member.inner {
            ast::Member::Field(field) => {
                let field = field.into_model(pos.clone())?;

                if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                    return Err(Error::field_conflict(field.name.clone(), pos, other.pos.clone()));
                }

                fields.push(field);
            }
            ast::Member::Code(context, lines) => {
                codes.push(code(&pos, member.pos, context, lines));
            }
            ast::Member::Option(option) => {
                options.push(option.into_model(pos)?);
            }
        }
    }

    Ok((fields, codes, options))
}

struct OrdinalGenerator {
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

    pub fn next(&mut self, ordinal: &Option<ast::Token<Value>>, pos: &Pos) -> Result<u32> {
        if let Some(ref ordinal) = *ordinal {
            let pos = (pos.0.to_owned(), ordinal.pos.0, ordinal.pos.1);

            if let Value::Number(ref number) = ordinal.inner {
                let n: u32 = number.floor() as u32;

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

/// Adds the into_model() method for all types that supports conversion into models.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, pos: Pos) -> Result<Token<Self::Output>>;
}

impl IntoModel for ast::InterfaceBody {
    type Output = InterfaceBody;

    fn into_model(self, pos: Pos) -> Result<Token<InterfaceBody>> {
        let (fields, codes, options) = members_into_model(&pos, self.members)?;

        let mut sub_types: BTreeMap<String, Token<SubType>> = BTreeMap::new();

        for sub_type in self.sub_types {
            let pos = (pos.0.clone(), sub_type.pos.0, sub_type.pos.1);
            let sub_type = sub_type.inner.into_model(pos)?;

            // key has to be owned by entry
            let key = sub_type.name.clone();

            match sub_types.entry(key) {
                btree_map::Entry::Occupied(entry) => {
                    entry.into_mut().merge(sub_type)?;
                }
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(sub_type);
                }
            }
        }

        let _options = Options::new(&pos, options);

        let interface_body = InterfaceBody {
            name: self.name,
            fields: fields,
            codes: codes,
            sub_types: sub_types,
        };

        Ok(Token::new(interface_body, pos.clone()))
    }
}

impl IntoModel for ast::EnumBody {
    type Output = EnumBody;

    fn into_model(self, pos: Pos) -> Result<Token<EnumBody>> {
        let mut values = Vec::new();

        let mut ordinals = OrdinalGenerator::new();

        let (fields, codes, options) = members_into_model(&pos, self.members)?;

        for value in self.values {
            let pos = (pos.0.to_owned(), value.pos.0, value.pos.1);
            let value = value.inner;

            let name = value.name;
            let arguments: Vec<Token<Value>> =
                value.arguments.into_iter().map(|a| a.with_prefix(pos.0.to_owned())).collect();
            let ordinal = ordinals.next(&value.ordinal, &pos)?;

            let value = EnumValue {
                name: name,
                arguments: arguments,
                ordinal: ordinal,
            };

            values.push(Token::new(value, pos));
        }

        let options = Options::new(&pos, options);

        let serialized_as: Option<Token<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        let serialized_as_name = options.find_one_boolean("serialized_as_name")?
            .to_owned()
            .map(|t| t.inner)
            .unwrap_or(false);

        let en = EnumBody {
            name: self.name,
            values: values,
            fields: fields,
            codes: codes,
            serialized_as: serialized_as,
            serialized_as_name: serialized_as_name,
        };

        Ok(Token::new(en, pos.clone()))
    }
}

impl IntoModel for ast::TypeBody {
    type Output = TypeBody;

    fn into_model(self, pos: Pos) -> Result<Token<TypeBody>> {
        let (fields, codes, options) = members_into_model(&pos, self.members)?;

        let options = Options::new(&pos, options);

        let reserved: HashSet<Token<String>> =
            options.find_all_identifiers("reserved")?.into_iter().collect();

        let type_body = TypeBody {
            name: self.name,
            fields: fields,
            codes: codes,
            reserved: reserved,
        };

        Ok(Token::new(type_body, pos.clone()))
    }
}

impl IntoModel for ast::SubType {
    type Output = SubType;

    fn into_model(self, pos: Pos) -> Result<Token<SubType>> {
        let mut fields: Vec<Token<Field>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();

        for member in self.members {
            let pos = (pos.0.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = field.into_model(pos.clone())?;

                    if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                        return Err(Error::field_conflict(field.name.clone(),
                                                         pos,
                                                         other.pos.clone()));
                    }

                    fields.push(field);
                }
                ast::Member::Code(context, lines) => {
                    codes.push(code(&pos, member.pos, context, lines));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos)?);
                }
            }
        }

        let options = Options::new(&pos, options);

        let names = options.find_all_strings("name")?;

        let sub_type = SubType {
            name: self.name,
            fields: fields,
            codes: codes,
            names: names,
        };

        Ok(Token::new(sub_type, pos.clone()))
    }
}

impl IntoModel for ast::TupleBody {
    type Output = TupleBody;

    fn into_model(self, pos: Pos) -> Result<Token<TupleBody>> {
        let (fields, codes, options) = members_into_model(&pos, self.members)?;

        let _options = Options::new(&pos, options);

        let tuple_body = TupleBody {
            name: self.name,
            fields: fields,
            codes: codes,
        };

        Ok(Token::new(tuple_body, pos.clone()))
    }
}

impl IntoModel for ast::Token<ast::Decl> {
    type Output = Decl;

    fn into_model(self, pos: Pos) -> Result<Token<Decl>> {
        let decl = match self.inner {
            ast::Decl::Type(body) => body.into_model(pos)?.map_inner(Decl::Type),
            ast::Decl::Interface(body) => body.into_model(pos)?.map_inner(Decl::Interface),
            ast::Decl::Enum(body) => body.into_model(pos)?.map_inner(Decl::Enum),
            ast::Decl::Tuple(body) => body.into_model(pos)?.map_inner(Decl::Tuple),
        };

        Ok(decl)
    }
}

impl IntoModel for ast::OptionDecl {
    type Output = OptionDecl;

    fn into_model(self, pos: Pos) -> Result<Token<OptionDecl>> {
        let mut values = Vec::new();

        for value in self.values {
            values.push(value.with_prefix(pos.0.to_owned()));
        }

        let decl = OptionDecl {
            name: self.name,
            values: values,
        };

        Ok(Token::new(decl, pos))
    }
}

impl IntoModel for ast::Field {
    type Output = Field;

    fn into_model(self, pos: Pos) -> Result<Token<Field>> {
        let field_as = if let Some(ident) = self.field_as {
            let pos = (pos.0.clone(), ident.pos.0, ident.pos.1);
            Some(ident.into_model(pos)?)
        } else {
            None
        };

        let field = Field {
            modifier: self.modifier,
            name: self.name,
            ty: self.ty,
            field_as: field_as,
        };

        Ok(Token::new(field, pos))
    }
}

impl IntoModel for ast::Token<Value> {
    type Output = String;

    fn into_model(self, pos: Pos) -> Result<Token<String>> {
        if let Value::String(string) = self.inner {
            return Ok(Token::new(string, pos));
        }

        Err(Error::pos("expected string".to_owned(), pos))
    }
}
