use ast;
use parser;
use super::models::*;
use super::errors::*;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

const EXT: &str = "reproto";

pub type TypeId = (Package, String);

pub struct Environment {
    paths: Vec<PathBuf>,
    visited: HashSet<Package>,
    pub types: BTreeMap<TypeId, Token<Decl>>,
    pub used: BTreeMap<(Package, String), Package>,
}

impl Environment {
    pub fn new(paths: Vec<PathBuf>) -> Environment {
        Environment {
            paths: paths,
            visited: HashSet::new(),
            types: BTreeMap::new(),
            used: BTreeMap::new(),
        }
    }

    fn register_type(&mut self, package: &Package, decl: Token<Decl>) -> Result<()> {
        let key = (package.clone(), decl.name().to_owned());

        match self.types.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(decl);
            }
            Entry::Occupied(entry) => {
                let target = entry.into_mut();
                let target_pos = target.pos.clone();

                match &mut target.inner {
                    &mut Decl::Type(ref mut body) => {
                        if let Decl::Type(ref other) = decl.inner {
                            body.merge(other)?;
                        } else {
                            return Err(Error::decl_merge(format!("Cannot merge {}",
                                                                 decl.display()),
                                                         decl.pos,
                                                         target_pos));
                        }
                    }
                    &mut Decl::Enum(ref mut body) => {
                        if let Decl::Enum(ref other) = decl.inner {
                            body.merge(other)?;
                        } else {
                            return Err(Error::decl_merge(format!("Cannot merge {}",
                                                                 decl.display()),
                                                         decl.pos,
                                                         target_pos));
                        }
                    }
                    _ => return Err("not yet supported".into()),
                }
            }
        };

        Ok(())
    }

    fn register_alias(&mut self, package: &Package, use_decl: &ast::UseDecl) -> Result<()> {
        if let Some(used) = use_decl.package.parts.iter().last() {
            let alias = if let Some(ref next) = use_decl.alias {
                next
            } else {
                used
            };

            let key = (package.clone(), alias.clone());

            match self.used.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(use_decl.package.clone());
                }
                Entry::Occupied(_) => return Err(format!("alias {} already in used", alias).into()),
            };
        }

        Ok(())
    }

    /// Lookup the package declaration a used alias refers to.
    pub fn lookup_used(&self, package: &Package, used: &str) -> Result<&Package> {
        // resolve alias
        let package = self.used
            .get(&(package.clone(), used.to_owned()))
            .ok_or(format!("Missing import alias for ({})", used))?;

        // check that type actually exists?
        let key = (package.clone(), used.to_owned());
        let _ = self.types.get(&key);

        Ok(package)
    }

    fn convert_type(&self, path: &Path, body: &ast::TypeBody) -> Result<TypeBody> {
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

        Ok(TypeBody::new(body.name.clone(), fields, codes))
    }

    fn convert_interface(&self, path: &Path, body: &ast::InterfaceBody) -> Result<InterfaceBody> {
        let mut fields = Vec::new();
        let mut codes = Vec::new();
        let mut sub_types = BTreeMap::new();

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

        for (key, sub_type) in &body.sub_types {
            let pos = (path.to_owned(), sub_type.pos.0, sub_type.pos.1);
            let ty = self.convert_type(path, sub_type)?;
            let names = self.find_all_strings(path, &sub_type.options, "name")?;
            let sub_type = SubType::new(ty.name, ty.fields, ty.codes, names);

            sub_types.insert(key.clone(), Token::new(sub_type, pos));
        }

        Ok(InterfaceBody::new(body.name.clone(), fields, codes, sub_types))
    }

    fn convert_enum(&self, path: &Path, body: &ast::EnumBody) -> Result<EnumBody> {
        let mut values = Vec::new();
        let mut fields = Vec::new();
        let mut codes = Vec::new();

        for value in &body.values {
            let mut enum_values = Vec::new();

            for enum_value in &value.values {
                let pos = (path.to_owned(), enum_value.pos.0, enum_value.pos.1);
                enum_values.push(Token::new(enum_value.inner.clone(), pos));
            }

            let pos = (path.to_owned(), value.pos.0, value.pos.1);
            let value = EnumValue {
                name: value.name.clone(),
                values: enum_values,
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

        let serialized_as: Option<Token<String>> =
            self.find_one_identifier(path, &body.options, "serialized_as")?;

        Ok(EnumBody::new(body.name.clone(), values, fields, codes, serialized_as))
    }

    fn find_one_identifier(&self,
                           path: &Path,
                           options: &ast::Options,
                           name: &str)
                           -> Result<Option<Token<String>>> {
        let mut out: Option<Token<String>> = None;

        for s in options.lookup(name) {
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

    fn find_all_strings(&self,
                        path: &Path,
                        options: &ast::Options,
                        name: &str)
                        -> Result<Vec<Token<String>>> {
        let mut out: Vec<Token<String>> = Vec::new();

        for s in options.lookup(name) {
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

    fn convert_tuple(&self, path: &Path, body: &ast::TupleBody) -> Result<TupleBody> {
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

    fn convert_decl(&self, path: &Path, decl: &ast::Token<ast::Decl>) -> Result<Token<Decl>> {
        let pos = (path.to_owned(), decl.pos.0, decl.pos.1);

        let decl = match decl.inner {
            ast::Decl::Type(ref body) => Decl::Type(self.convert_type(path, body)?),
            ast::Decl::Interface(ref body) => Decl::Interface(self.convert_interface(path, body)?),
            ast::Decl::Enum(ref body) => Decl::Enum(self.convert_enum(path, body)?),
            ast::Decl::Tuple(ref body) => Decl::Tuple(self.convert_tuple(path, body)?),
        };

        Ok(Token::new(decl, pos))
    }

    pub fn import_file(&mut self, path: &Path, package: Option<&Package>) -> Result<()> {
        debug!("in: {}", path.display());

        // TODO: fix this
        let file = parser::parse_file(&path).unwrap();

        if let Some(package) = package {
            if *file.package != *package {
                return Err(format!("Expected package ({}) in file {}, but was ({})",
                                   package,
                                   path.display(),
                                   *file.package)
                    .into());
            }
        }

        for use_decl in &file.uses {
            self.register_alias(&file.package, use_decl)?;
            self.import(&use_decl.package)?;
        }

        for decl in &file.decls {
            let decl = self.convert_decl(path, decl)?;
            self.register_type(&file.package, decl)?;
        }

        Ok(())
    }

    pub fn import(&mut self, package: &Package) -> Result<()> {
        if self.visited.contains(package) {
            return Ok(());
        }

        self.visited.insert(package.clone());

        let mut files: Vec<PathBuf> = Vec::new();

        let candidates: Vec<PathBuf> = self.paths
            .iter()
            .map(|p| {
                let mut path = p.clone();

                for part in &package.parts {
                    path.push(part);
                }

                path.set_extension(EXT);
                path
            })
            .collect();

        for path in &candidates {
            if !path.is_file() {
                continue;
            }

            files.push(path.clone());
        }

        if files.len() == 0 {
            let candidates_format: Vec<String> = candidates.iter()
                .map(|c| format!("{}", c.display()))
                .collect();

            let candidates_format = candidates_format.join(", ");

            return Err(format!("No files matching package ({}), expected one of: {}",
                               *package,
                               candidates_format)
                .into());
        }

        for path in files {
            self.import_file(&path, Some(package))?;
        }

        Ok(())
    }
}
