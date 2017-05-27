//! Implementations for converting asts into models.
use parser::ast;
use std::collections::BTreeMap;
use std::path::Path;
use super::errors::*;
use super::into_model::IntoModel;
use super::models::*;
use with_prefix::WithPrefix;

/// Extensions to ast::Options.
trait OptionsExt {
    /// Base lookup method (defers to ast::Options::lookup).
    fn lookup<'a>(&'a self,
                  name: &'a str)
                  -> Box<Iterator<Item = &ast::Token<ast::OptionValue>> + 'a>;

    fn find_all_strings(&self, path: &Path, name: &str) -> Result<Vec<Token<String>>> {
        let mut out: Vec<Token<String>> = Vec::new();

        for s in self.lookup(name) {
            let pos = (path.to_owned(), s.pos.0, s.pos.1);

            match **s {
                ast::OptionValue::String(ref string) => {
                    out.push(Token::new(string.clone(), pos));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected identifier", name), pos));
                }
            }
        }

        Ok(out)
    }

    fn find_one_identifier(&self, path: &Path, name: &str) -> Result<Option<Token<String>>> {
        let mut out: Option<Token<String>> = None;

        for s in self.lookup(name) {
            let pos = (path.to_owned(), s.pos.0, s.pos.1);

            if let Some(_) = out {
                return Err(Error::pos(format!("{}: only one value may be present", name), pos));
            }

            match **s {
                ast::OptionValue::Identifier(ref string) => {
                    out = Some(Token::new(string.clone(), pos));
                }
                _ => {
                    return Err(Error::pos(format!("{}: expected identifier", name), pos));
                }
            }
        }

        Ok(out)
    }
}

/// Binding for ast::Options to extensions.
impl OptionsExt for ast::Options {
    fn lookup<'a>(&'a self,
                  name: &'a str)
                  -> Box<Iterator<Item = &ast::Token<ast::OptionValue>> + 'a> {
        ast::Options::lookup(self, name)
    }
}

/// Trait that performs the basic conversion.
pub trait ConvertToModel {
    type Output;

    fn convert_to_model(Self, path: &Path) -> Result<Self::Output>;
}

impl ConvertToModel for ast::InterfaceBody {
    type Output = InterfaceBody;

    fn convert_to_model(body: ast::InterfaceBody, path: &Path) -> Result<InterfaceBody> {
        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut sub_types = BTreeMap::new();

        for member in body.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match *member {
                ast::Member::Field(ref field) => {
                    let field =
                        Field::new(field.modifier.clone(), field.name.clone(), field.ty.clone());
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(ref context, ref lines) => {
                    let code = Code::new(context.clone(), lines.clone());
                    codes.push(Token::new(code, pos));
                }
            }
        }

        for (key, sub_type) in body.sub_types {
            let pos = (path.to_owned(), sub_type.pos.0, sub_type.pos.1);
            let names = sub_type.options.find_all_strings(path, "name")?;
            let ty = sub_type.inner.into_model(path)?;
            let sub_type = SubType::new(ty.name, ty.fields, ty.codes, names);

            sub_types.insert(key.clone(), Token::new(sub_type, pos));
        }

        Ok(InterfaceBody::new(body.name.clone(), fields, codes, sub_types))
    }
}

impl ConvertToModel for ast::EnumBody {
    type Output = EnumBody;

    fn convert_to_model(body: ast::EnumBody, path: &Path) -> Result<EnumBody> {
        let mut values = Vec::new();
        let mut fields = Vec::new();
        let mut codes = Vec::new();

        for value in &body.values {
            let mut arguments = Vec::new();

            for argument in &value.arguments {
                arguments.push(argument.clone().with_prefix(path.to_owned()));
            }

            let pos = (path.to_owned(), value.pos.0, value.pos.1);
            let value = EnumValue {
                name: value.name.clone(),
                arguments: arguments,
            };

            values.push(Token::new(value, pos));
        }

        for member in &body.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match **member {
                ast::Member::Field(ref field) => {
                    let field =
                        Field::new(field.modifier.clone(), field.name.clone(), field.ty.clone());
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(ref context, ref lines) => {
                    let code = Code::new(context.clone(), lines.clone());
                    codes.push(Token::new(code, pos));
                }
            }
        }

        let serialized_as: Option<Token<String>> = body.options
            .find_one_identifier(path, "serialized_as")?;

        Ok(EnumBody::new(body.name.clone(), values, fields, codes, serialized_as))
    }
}

impl ConvertToModel for ast::TypeBody {
    type Output = TypeBody;

    fn convert_to_model(body: ast::TypeBody, path: &Path) -> Result<TypeBody> {
        let mut fields: Vec<Token<Field>> = Vec::new();
        let mut codes = Vec::new();

        for member in &body.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match **member {
                ast::Member::Field(ref field) => {
                    let field =
                        Field::new(field.modifier.clone(), field.name.clone(), field.ty.clone());

                    if let Some(other) = fields.iter().find(|f| f.name == field.name) {
                        return Err(Error::field_conflict(field.name.clone(),
                                                         pos.clone(),
                                                         other.pos.clone()));
                    }

                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(ref context, ref lines) => {
                    let code = Code::new(context.clone(), lines.clone());
                    codes.push(Token::new(code, pos));
                }
            }
        }

        Ok(TypeBody::new(body.name.clone(), fields, codes))
    }
}

impl ConvertToModel for ast::TupleBody {
    type Output = TupleBody;

    fn convert_to_model(body: ast::TupleBody, path: &Path) -> Result<TupleBody> {
        let mut fields = Vec::new();
        let mut codes = Vec::new();

        for member in &body.members {
            let pos = (path.to_owned(), member.pos.0, member.pos.1);

            match **member {
                ast::Member::Field(ref field) => {
                    let field =
                        Field::new(field.modifier.clone(), field.name.clone(), field.ty.clone());
                    fields.push(Token::new(field, pos));
                }
                ast::Member::Code(ref context, ref lines) => {
                    let code = Code::new(context.clone(), lines.clone());
                    codes.push(Token::new(code, pos));
                }
            }
        }

        Ok(TupleBody::new(body.name.clone(), fields, codes))
    }
}

impl ConvertToModel for ast::Token<ast::Decl> {
    type Output = Token<Decl>;

    fn convert_to_model(decl: ast::Token<ast::Decl>, path: &Path) -> Result<Token<Decl>> {
        let pos = (path.to_owned(), decl.pos.0, decl.pos.1);

        let decl = match decl.inner {
            ast::Decl::Type(body) => Decl::Type(body.into_model(path)?),
            ast::Decl::Interface(body) => Decl::Interface(body.into_model(path)?),
            ast::Decl::Enum(body) => Decl::Enum(body.into_model(path)?),
            ast::Decl::Tuple(body) => Decl::Tuple(body.into_model(path)?),
        };

        Ok(Token::new(decl, pos))
    }
}
