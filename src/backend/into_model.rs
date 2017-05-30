//! Implementations for converting asts into models.
use parser::ast;
use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map;
use std::path::Path;
use super::errors::*;
use super::merge::Merge;
use super::models::*;
use super::options::Options;
use with_prefix::WithPrefix;

/// Adds the into_model() method for all types that supports conversion into models.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<Self::Output>>;
}

impl IntoModel for ast::InterfaceBody {
    type Output = InterfaceBody;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<InterfaceBody>> {
        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut sub_types: BTreeMap<String, Token<SubType>> = BTreeMap::new();
        let mut options = Vec::new();

        for member in self.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = Field::new(field.modifier, field.name, field.ty);
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(context, lines) => {
                    let code = Code::new(context, lines);
                    codes.push(Token::new(code, pos));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos, path)?);
                }
            }
        }

        for sub_type in self.sub_types {
            let pos = (path.to_owned(), sub_type.pos.0, sub_type.pos.1);
            let sub_type = sub_type.inner.into_model(pos, path)?;

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

        drop(_options);

        Ok(Token::new(InterfaceBody::new(self.name, fields, codes, sub_types),
                      pos.clone()))
    }
}

impl IntoModel for ast::EnumBody {
    type Output = EnumBody;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<EnumBody>> {
        let mut values = Vec::new();
        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();

        for value in self.values {
            let pos = value.pos;
            let pos = (path.to_owned(), pos.0, pos.1);
            let value = value.inner;

            let name = value.name;
            let arguments: Vec<Token<Value>> =
                value.arguments.into_iter().map(|a| a.with_prefix(path.to_owned())).collect();

            let value = EnumValue {
                name: name,
                arguments: arguments,
            };

            values.push(Token::new(value, pos));
        }

        for member in self.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = Field::new(field.modifier, field.name, field.ty);
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(context, lines) => {
                    let code = Code::new(context, lines);
                    codes.push(Token::new(code, pos));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos, path)?);
                }
            }
        }

        let options = Options::new(&pos, options);

        let serialized_as: Option<Token<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        let serialized_as_name: Option<Token<bool>> =
            options.find_one_boolean("serialized_as_name")?
                .to_owned();

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

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<TypeBody>> {
        let mut fields: Vec<Token<Field>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();

        for member in self.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = Field::new(field.modifier, field.name, field.ty);

                    if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                        return Err(Error::field_conflict(field.name, pos, other.pos.clone()));
                    }

                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(context, lines) => {
                    let code = Code::new(context, lines);
                    codes.push(Token::new(code, pos));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos, path)?);
                }
            }
        }

        let options = Options::new(&pos, options);

        let reserved: HashSet<Token<String>> =
            options.find_all_identifiers("reserved")?.into_iter().collect();

        Ok(Token::new(TypeBody::new(self.name, fields, codes, reserved),
                      pos.clone()))
    }
}

impl IntoModel for ast::SubType {
    type Output = SubType;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<SubType>> {
        let mut fields: Vec<Token<Field>> = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();

        for member in self.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = Field::new(field.modifier, field.name, field.ty);

                    if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                        return Err(Error::field_conflict(field.name, pos, other.pos.clone()));
                    }

                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(context, lines) => {
                    let code = Code::new(context, lines);
                    codes.push(Token::new(code, pos));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos, path)?);
                }
            }
        }

        let options = Options::new(&pos, options);

        let names = options.find_all_strings("name")?;

        Ok(Token::new(SubType::new(self.name, fields, codes, names), pos.clone()))
    }
}

impl IntoModel for ast::TupleBody {
    type Output = TupleBody;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<TupleBody>> {
        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut options = Vec::new();

        for member in self.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match member.inner {
                ast::Member::Field(field) => {
                    let field = Field::new(field.modifier, field.name, field.ty);
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(context, lines) => {
                    let code = Code::new(context, lines);
                    codes.push(Token::new(code, pos));
                }
                ast::Member::Option(option) => {
                    options.push(option.into_model(pos, path)?);
                }
            }
        }

        let _options = Options::new(&pos, options);

        Ok(Token::new(TupleBody::new(self.name, fields, codes), pos.clone()))
    }
}

impl IntoModel for ast::Token<ast::Decl> {
    type Output = Decl;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<Decl>> {
        let decl = match self.inner {
            ast::Decl::Type(body) => body.into_model(pos, path)?.map_inner(Decl::Type),
            ast::Decl::Interface(body) => body.into_model(pos, path)?.map_inner(Decl::Interface),
            ast::Decl::Enum(body) => body.into_model(pos, path)?.map_inner(Decl::Enum),
            ast::Decl::Tuple(body) => body.into_model(pos, path)?.map_inner(Decl::Tuple),
        };

        Ok(decl)
    }
}

impl IntoModel for ast::OptionDecl {
    type Output = OptionDecl;

    fn into_model(self, pos: Pos, path: &Path) -> Result<Token<OptionDecl>> {
        let mut values = Vec::new();

        for value in self.values {
            values.push(value.with_prefix(path.to_owned()));
        }

        Ok(Token::new(OptionDecl::new(self.name, values), pos))
    }
}
