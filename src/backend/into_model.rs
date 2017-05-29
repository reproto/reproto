//! Implementations for converting asts into models.
use parser::ast;
use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map;
use std::path::Path;
use super::errors::*;
use super::merge::Merge;
use super::models::*;
use with_prefix::WithPrefix;

/// Adds the into_model() method for all types that supports conversion into models.
pub trait IntoModel {
    type Output;

    /// Convert the current type to a model.
    fn into_model(self, path: &Path) -> Result<Self::Output>;
}

impl IntoModel for ast::InterfaceBody {
    type Output = InterfaceBody;

    fn into_model(self, path: &Path) -> Result<InterfaceBody> {
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
                    options.push(Token::new(option.into_model(path)?, pos));
                }
            }
        }

        for sub_type in self.sub_types {
            let pos = (path.to_owned(), sub_type.pos.0, sub_type.pos.1);
            let sub_type = sub_type.inner.into_model(path)?;

            // key has to be owned by entry
            let key = sub_type.name.clone();

            match sub_types.entry(key) {
                btree_map::Entry::Occupied(entry) => {
                    let existing = &mut entry.into_mut().inner;
                    existing.merge(sub_type)?;
                }
                btree_map::Entry::Vacant(entry) => {
                    entry.insert(Token::new(sub_type, pos));
                }
            }
        }

        let _options = Options::new(options);

        Ok(InterfaceBody::new(self.name, fields, codes, sub_types))
    }
}

impl IntoModel for ast::EnumBody {
    type Output = EnumBody;

    fn into_model(self, path: &Path) -> Result<EnumBody> {
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
                    options.push(Token::new(option.into_model(path)?, pos));
                }
            }
        }

        let options = Options::new(options);

        let serialized_as: Option<Token<String>> = options.find_one_identifier("serialized_as")?
            .to_owned();

        Ok(EnumBody::new(self.name, values, fields, codes, serialized_as))
    }
}

impl IntoModel for ast::TypeBody {
    type Output = TypeBody;

    fn into_model(self, path: &Path) -> Result<TypeBody> {
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
                    options.push(Token::new(option.into_model(path)?, pos));
                }
            }
        }

        let options = Options::new(options);

        let reserved: HashSet<Token<String>> =
            options.find_all_identifiers("reserved")?.into_iter().collect();

        Ok(TypeBody::new(self.name, fields, codes, reserved))
    }
}

impl IntoModel for ast::SubType {
    type Output = SubType;

    fn into_model(self, path: &Path) -> Result<SubType> {
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
                    options.push(Token::new(option.into_model(path)?, pos));
                }
            }
        }

        let options = Options::new(options);

        let names = options.find_all_strings("name")?;
        Ok(SubType::new(self.name, fields, codes, names))
    }
}

impl IntoModel for ast::TupleBody {
    type Output = TupleBody;

    fn into_model(self, path: &Path) -> Result<TupleBody> {
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
                    options.push(Token::new(option.into_model(path)?, pos));
                }
            }
        }

        let _options = Options::new(options);

        Ok(TupleBody::new(self.name, fields, codes))
    }
}

impl IntoModel for ast::Token<ast::Decl> {
    type Output = Token<Decl>;

    fn into_model(self, path: &Path) -> Result<Token<Decl>> {
        let pos = (path.to_owned(), self.pos.0, self.pos.1);

        let decl = match self.inner {
            ast::Decl::Type(body) => Decl::Type(body.into_model(path)?),
            ast::Decl::Interface(body) => Decl::Interface(body.into_model(path)?),
            ast::Decl::Enum(body) => Decl::Enum(body.into_model(path)?),
            ast::Decl::Tuple(body) => Decl::Tuple(body.into_model(path)?),
        };

        Ok(Token::new(decl, pos))
    }
}

impl IntoModel for ast::OptionDecl {
    type Output = OptionDecl;

    fn into_model(self, path: &Path) -> Result<OptionDecl> {
        let mut values = Vec::new();

        for value in self.values {
            values.push(value.with_prefix(path.to_owned()));
        }

        Ok(OptionDecl::new(self.name, values))
    }
}
