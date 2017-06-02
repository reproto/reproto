use parser::ast;
use parser;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashSet, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use super::errors::*;
use super::into_model::IntoModel;
use super::merge::Merge;
use super::models::*;

pub type InitFields = HashMap<String, Token<FieldInit>>;

const EXT: &str = "reproto";

pub struct Environment {
    paths: Vec<PathBuf>,
    visited: HashSet<Package>,
    pub types: BTreeMap<TypeId, Token<Registered>>,
    pub decls: BTreeMap<TypeId, Rc<Token<Decl>>>,
    pub used: BTreeMap<(Package, String), Package>,
}

impl Environment {
    pub fn new(paths: Vec<PathBuf>) -> Environment {
        Environment {
            paths: paths,
            visited: HashSet::new(),
            types: BTreeMap::new(),
            decls: BTreeMap::new(),
            used: BTreeMap::new(),
        }
    }

    fn into_registered_type(&self,
                            package: &Package,
                            decl: Rc<Token<Decl>>)
                            -> Result<Vec<(TypeId, Token<Registered>)>> {
        let mut out = Vec::new();

        match decl.inner {
            Decl::Type(ref ty) => {
                let type_id = TypeId::new(package.clone(),
                                          Custom::with_parts(vec![ty.name.clone()]));
                let token = Token::new(Registered::Type(ty.clone()), decl.pos.clone());
                out.push((type_id, token));
            }
            Decl::Interface(ref interface) => {
                let current = vec![interface.name.clone()];
                let type_id = TypeId::new(package.clone(), Custom::with_parts(current.clone()));
                let token = Token::new(Registered::Interface(interface.clone()), decl.pos.clone());

                for (name, sub_type) in &interface.sub_types {
                    let sub_type = Registered::SubType {
                        parent: interface.clone(),
                        sub_type: sub_type.inner.clone(),
                    };
                    let token = Token::new(sub_type, decl.pos.clone());

                    let mut current = current.clone();
                    current.push(name.to_owned());
                    out.push((type_id.with_custom(Custom::with_parts(current)), token));
                }

                out.push((type_id, token));
            }
            Decl::Enum(ref en) => {
                let current = vec![en.name.clone()];
                let type_id = TypeId::new(package.clone(), Custom::with_parts(current.clone()));
                let token = Token::new(Registered::Enum(en.clone()), decl.pos.clone());

                for value in &en.values {
                    let enum_constant = Registered::EnumConstant {
                        parent: en.clone(),
                        value: value.inner.clone(),
                    };
                    let token = Token::new(enum_constant, decl.pos.clone());

                    let mut current = current.clone();
                    current.push((*value.name).to_owned());
                    out.push((type_id.with_custom(Custom::with_parts(current)), token));
                }

                out.push((type_id, token));
            }
            Decl::Tuple(ref tuple) => {
                let type_id = TypeId::new(package.clone(),
                                          Custom::with_parts(vec![tuple.name.clone()]));
                let token = Token::new(Registered::Tuple(tuple.clone()), decl.pos.clone());
                out.push((type_id, token));
            }
        }

        Ok(out)
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
                Entry::Vacant(entry) => entry.insert(use_decl.package.inner.clone()),
                Entry::Occupied(_) => return Err(format!("alias {} already in used", alias).into()),
            };
        }

        Ok(())
    }

    pub fn is_assignable_from(&self,
                              package: &Package,
                              target: &Type,
                              source: &Type)
                              -> Result<bool> {
        match (target, source) {
            (&Type::Double, &Type::Double) => Ok(true),
            (&Type::Float, &Type::Float) => Ok(true),
            (&Type::Signed(Some(ref target)), &Type::Signed(Some(ref source))) => {
                Ok(target <= source)
            }
            // unknown size matches known
            (&Type::Signed(_), &Type::Signed(None)) => Ok(true),
            (&Type::Unsigned(Some(ref target)), &Type::Unsigned(Some(ref source))) => {
                Ok(target <= source)
            }
            // unknown size matches known
            (&Type::Unsigned(_), &Type::Unsigned(None)) => Ok(true),
            (&Type::Boolean, &Type::Boolean) => return Ok(true),
            (&Type::String, &Type::String) => return Ok(true),
            (&Type::Bytes, &Type::Bytes) => return Ok(true),
            // everything assignable to any type
            (&Type::Any, _) => Ok(true),
            (&Type::Custom(ref target), &Type::Custom(ref source)) => {
                let target = self.lookup(package, target)?;
                let source = self.lookup(package, source)?;
                return Ok(target.is_assignable_from(source));
            }
            // arrays match if inner type matches
            (&Type::Array(ref target), &Type::Array(ref source)) => {
                return self.is_assignable_from(package, target, source);
            }
            (&Type::Map(ref target_key, ref target_value),
             &Type::Map(ref source_key, ref source_value)) => {
                let key_assignable = self.is_assignable_from(package, target_key, source_key)?;
                let value_assignable =
                    self.is_assignable_from(package, target_value, source_value)?;

                return Ok(key_assignable && value_assignable);
            }
            _ => Ok(false),
        }
    }

    pub fn constant<'a>(&'a self,
                        pos: &Pos,
                        package: &'a Package,
                        constant: &Custom,
                        target: &Custom)
                        -> Result<&'a Registered> {
        let reg_constant = self.lookup(package, constant)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

        let reg_target = self.lookup(package, target)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

        if !reg_target.is_assignable_from(reg_constant) {
            return Err(Error::pos(format!("expected instance of `{}` but found `{}`",
                                          reg_target.display(),
                                          reg_constant.display()),
                                  pos.clone()));
        }

        Ok(reg_constant)
    }

    /// Convert instance arguments to the known registered type of the instance, and a map
    /// containing the arguments being instantiated.
    pub fn instance<'a>(&'a self,
                        pos: &Pos,
                        package: &'a Package,
                        instance: &Instance,
                        target: &Custom)
                        -> Result<(&'a Registered, InitFields)> {
        let reg_instance = self.lookup(package, &instance.ty)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

        let reg_target = self.lookup(package, target)
            .map_err(|e| Error::pos(e.description().to_owned(), pos.clone()))?;

        if !reg_target.is_assignable_from(reg_instance) {
            return Err(Error::pos(format!("expected instance of `{}` but found `{}`",
                                          reg_target.display(),
                                          reg_instance.display()),
                                  pos.clone()));
        }

        let required_fields = match *reg_instance {
            Registered::Type(ref ty) => ty.fields(),
            Registered::SubType { ref parent, ref sub_type } => {
                Box::new(parent.fields().chain(sub_type.fields()))
            }
            Registered::Tuple(ref tuple) => tuple.fields(),
            _ => return Err(Error::pos("expected instantiable type".into(), pos.clone())),
        };

        // pick required fields.
        let required_fields = required_fields.filter(|f| f.modifier == Modifier::Required);

        let mut known: HashMap<String, Token<FieldInit>> = HashMap::new();

        // check that all required fields are set.
        let mut required: BTreeMap<String, Token<Field>> = required_fields.map(Clone::clone)
            .map(|f| (f.name.clone(), f))
            .collect();

        for init in &*instance.arguments {
            if let Some(ref field) = reg_instance.find_field(&init.name)? {
                // TODO: map out init position, and check that required variables are set.
                known.insert(field.name.clone(), init.clone());
                required.remove(&field.name);
            } else {
                return Err(Error::pos("no such field".to_owned(), init.pos.clone()));
            }
        }

        if !required.is_empty() {
            let required: Vec<(String, Token<Field>)> = required.into_iter()
                .collect();

            let names: Vec<String> =
                required.iter().map(|&(ref name, _)| name.to_owned()).collect();

            let positions: Vec<Pos> = required.iter().map(|&(_, ref t)| t.pos.clone()).collect();

            return Err(ErrorKind::MissingRequired(names,
                                                  instance.arguments.pos.clone(),
                                                  positions)
                .into());
        }

        Ok((reg_instance, known))
    }

    /// Lookup the package declaration a used alias refers to.
    pub fn lookup_used(&self, package: &Package, used: &str) -> Result<&Package> {
        // resolve alias
        self.used
            .get(&(package.clone(), used.to_owned()))
            .ok_or_else(|| format!("Missing import alias for ({})", used).into())
    }

    /// Lookup the declaration matching the custom type.
    pub fn lookup<'a>(&'a self, package: &'a Package, custom: &Custom) -> Result<&'a Registered> {
        let package = if let Some(ref prefix) = custom.prefix {
            self.lookup_used(package, prefix)?
        } else {
            package
        };

        let key = TypeId::new(package.clone(), custom.clone());

        if let Some(ty) = self.types.get(&key) {
            return Ok(ty);
        }

        return Err("no such type".into());
    }

    pub fn import_file(&mut self, path: &Path, package: Option<&Package>) -> Result<()> {
        debug!("in: {}", path.display());

        let file = parser::parse_file(&path)?;

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

        let mut decls = BTreeMap::new();

        for decl in file.decls {
            let pos = (path.to_owned(), decl.pos.0, decl.pos.1);
            let decl = decl.into_model(&pos)?;

            let custom = Custom::with_parts(vec![decl.name().to_owned()]);
            let key: TypeId = TypeId::new(file.package.inner.clone(), custom);

            match decls.entry(key) {
                Entry::Vacant(entry) => {
                    entry.insert(Rc::new(decl));
                }
                Entry::Occupied(entry) => {
                    entry.into_mut().merge(Rc::new(decl))?;
                }
            }
        }

        let mut types = BTreeMap::new();

        // again, post-merge
        for (_, decl) in &decls {
            let registered_types = self.into_registered_type(&file.package, decl.clone())?;

            for (key, t) in registered_types.into_iter() {
                if let Some(_) = types.insert(key.clone(), t) {
                    return Err(ErrorKind::RegisteredTypeConflict(key.clone()).into());
                }
            }
        }

        self.decls.extend(decls);
        self.types.extend(types);
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

    pub fn verify(&mut self) -> Result<()> {
        for (_, ref ty) in &self.decls {
            match ty.inner {
                Decl::Type(ref ty) => {
                    ty.verify()?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
