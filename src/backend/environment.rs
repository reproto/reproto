use linked_hash_map::{self, LinkedHashMap};
use parser;
use reproto_core::*;
use reproto_parser::ast;
use reproto_parser::ast::IntoModel;
use reproto_repository::Resolver;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use super::errors::*;

pub type InitFields = HashMap<String, RpLoc<RpFieldInit>>;

pub struct Environment {
    resolver: Box<Resolver>,
    visited: HashSet<RpVersionedPackage>,
    pub types: LinkedHashMap<RpTypeId, RpLoc<RpRegistered>>,
    pub decls: LinkedHashMap<RpTypeId, Rc<RpLoc<RpDecl>>>,
    pub used: LinkedHashMap<(RpVersionedPackage, String), RpVersionedPackage>,
}

impl Environment {
    pub fn new(resolver: Box<Resolver>) -> Environment {
        Environment {
            resolver: resolver,
            visited: HashSet::new(),
            types: LinkedHashMap::new(),
            decls: LinkedHashMap::new(),
            used: LinkedHashMap::new(),
        }
    }

    fn into_registered_type(&self,
                            package: &RpVersionedPackage,
                            decl: Rc<RpLoc<RpDecl>>)
                            -> Result<Vec<(RpTypeId, RpLoc<RpRegistered>)>> {
        let mut out = Vec::new();

        match **decl {
            RpDecl::Type(ref ty) => {
                let type_id = package.into_type_id(RpName::with_parts(vec![ty.name.clone()]));
                let token = RpLoc::new(RpRegistered::Type(ty.clone()), decl.pos().clone());
                out.push((type_id, token));
            }
            RpDecl::Interface(ref interface) => {
                let current = vec![interface.name.clone()];
                let type_id = RpTypeId::new(package.clone(), RpName::with_parts(current.clone()));
                let token = RpLoc::new(RpRegistered::Interface(interface.clone()),
                                       decl.pos().clone());

                for (name, sub_type) in &interface.sub_types {
                    let sub_type = RpRegistered::SubType {
                        parent: interface.clone(),
                        sub_type: sub_type.as_ref().clone(),
                    };

                    let token = RpLoc::new(sub_type, decl.pos().clone());

                    let mut current = current.clone();
                    current.push(name.to_owned());
                    out.push((type_id.with_name(RpName::with_parts(current)), token));
                }

                out.push((type_id, token));
            }
            RpDecl::Enum(ref en) => {
                let current = vec![en.name.clone()];
                let type_id = RpTypeId::new(package.clone(), RpName::with_parts(current.clone()));
                let token = RpLoc::new(RpRegistered::Enum(en.clone()), decl.pos().clone());

                for variant in &en.variants {
                    let enum_constant = RpRegistered::EnumConstant {
                        parent: en.clone(),
                        variant: variant.as_ref().clone(),
                    };
                    let token = RpLoc::new(enum_constant, decl.pos().clone());

                    let mut current = current.clone();
                    current.push((*variant.name).to_owned());
                    out.push((type_id.with_name(RpName::with_parts(current)), token));
                }

                out.push((type_id, token));
            }
            RpDecl::Tuple(ref tuple) => {
                let type_id = RpTypeId::new(package.clone(),
                                            RpName::with_parts(vec![tuple.name.clone()]));
                let token = RpLoc::new(RpRegistered::Tuple(tuple.clone()), decl.pos().clone());
                out.push((type_id, token));
            }
            RpDecl::Service(ref service) => {
                let type_id = RpTypeId::new(package.clone(),
                                            RpName::with_parts(vec![service.name.clone()]));
                let token = RpLoc::new(RpRegistered::Service(service.clone()), decl.pos().clone());
                out.push((type_id, token));
            }
        }

        Ok(out)
    }

    fn register_alias(&mut self,
                      source_package: &RpVersionedPackage,
                      use_decl: &ast::UseDecl,
                      use_package: &RpVersionedPackage)
                      -> Result<()> {
        if let Some(used) = use_decl.package.parts.iter().last() {
            let alias = if let Some(ref next) = use_decl.alias {
                next
            } else {
                used
            };

            let key = (source_package.clone(), alias.clone());

            match self.used.entry(key) {
                linked_hash_map::Entry::Vacant(entry) => {
                    entry.insert(use_package.clone());
                }
                linked_hash_map::Entry::Occupied(_) => {
                    return Err(format!("alias {} already in used", alias).into())
                }
            };
        }

        Ok(())
    }

    pub fn is_assignable_from(&self,
                              package: &RpVersionedPackage,
                              target: &RpType,
                              source: &RpType)
                              -> Result<bool> {
        match (target, source) {
            (&RpType::Double, &RpType::Double) => Ok(true),
            (&RpType::Float, &RpType::Float) => Ok(true),
            (&RpType::Signed { size: Some(ref target) },
             &RpType::Signed { size: Some(ref source) }) => Ok(target <= source),
            // unknown size matches known
            (&RpType::Signed { size: _ }, &RpType::Signed { size: None }) => Ok(true),
            (&RpType::Unsigned { size: Some(ref target) },
             &RpType::Unsigned { size: Some(ref source) }) => Ok(target <= source),
            // unknown size matches known
            (&RpType::Unsigned { size: _ }, &RpType::Unsigned { size: None }) => Ok(true),
            (&RpType::Boolean, &RpType::Boolean) => return Ok(true),
            (&RpType::String, &RpType::String) => return Ok(true),
            (&RpType::Bytes, &RpType::Bytes) => return Ok(true),
            // everything assignable to any type
            (&RpType::Any, _) => Ok(true),
            (&RpType::Name { name: ref target }, &RpType::Name { name: ref source }) => {
                let target = self.lookup(package, target)?;
                let source = self.lookup(package, source)?;
                return Ok(target.is_assignable_from(source));
            }
            // arrays match if inner type matches
            (&RpType::Array { inner: ref target }, &RpType::Array { inner: ref source }) => {
                return self.is_assignable_from(package, target, source);
            }
            (&RpType::Map { key: ref target_key, value: ref target_value },
             &RpType::Map { key: ref source_key, value: ref source_value }) => {
                let key_assignable = self.is_assignable_from(package, target_key, source_key)?;
                let value_assignable =
                    self.is_assignable_from(package, target_value, source_value)?;

                return Ok(key_assignable && value_assignable);
            }
            _ => Ok(false),
        }
    }

    pub fn constant<'a>(&'a self,
                        pos: &RpPos,
                        package: &'a RpVersionedPackage,
                        constant: &RpName,
                        target: &RpName)
                        -> Result<&'a RpRegistered> {
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
                        pos: &RpPos,
                        package: &'a RpVersionedPackage,
                        instance: &RpInstance,
                        target: &RpName)
                        -> Result<(&'a RpRegistered, InitFields)> {
        let reg_instance = self.lookup(package, &instance.name)
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
            RpRegistered::Type(ref ty) => ty.fields(),
            RpRegistered::SubType { ref parent, ref sub_type } => {
                Box::new(parent.fields().chain(sub_type.fields()))
            }
            RpRegistered::Tuple(ref tuple) => tuple.fields(),
            _ => return Err(Error::pos("expected instantiable type".into(), pos.clone())),
        };

        // pick required fields.
        let required_fields = required_fields.filter(|f| f.modifier == RpModifier::Required);

        let mut known: HashMap<String, RpLoc<RpFieldInit>> = HashMap::new();

        // check that all required fields are set.
        let mut required: BTreeMap<String, RpLoc<RpField>> = required_fields.map(Clone::clone)
            .map(|f| (f.name().to_owned(), f))
            .collect();

        for init in &*instance.arguments {
            if let Some(ref field) = reg_instance.field_by_ident(&init.name)? {
                // TODO: map out init position, and check that required variables are set.
                known.insert(field.ident().to_owned(), init.clone());
                required.remove(field.name());
            } else {
                return Err(Error::pos("no such field".to_owned(), init.pos().clone()));
            }
        }

        if !required.is_empty() {
            let required: Vec<(String, RpLoc<RpField>)> = required.into_iter()
                .collect();

            let names: Vec<String> =
                required.iter().map(|&(ref name, _)| name.to_owned()).collect();

            let positions: Vec<RpPos> =
                required.iter().map(|&(_, ref t)| t.pos().clone()).collect();

            return Err(ErrorKind::MissingRequired(names,
                                                  instance.arguments.pos().clone(),
                                                  positions)
                .into());
        }

        Ok((reg_instance, known))
    }

    /// Lookup the package declaration a used alias refers to.
    pub fn lookup_used(&self,
                       package: &RpVersionedPackage,
                       used: &str)
                       -> Result<&RpVersionedPackage> {
        // resolve alias
        self.used
            .get(&(package.clone(), used.to_owned()))
            .ok_or_else(|| format!("Missing import alias for ({})", used).into())
    }

    /// Lookup the declaration matching the custom type.
    pub fn lookup<'a>(&'a self,
                      package: &'a RpVersionedPackage,
                      custom: &RpName)
                      -> Result<&'a RpRegistered> {
        let package = if let Some(ref prefix) = custom.prefix {
            self.lookup_used(package, prefix)?
        } else {
            package
        };

        let key = RpTypeId::new(package.clone(), custom.clone());

        if let Some(ty) = self.types.get(&key) {
            return Ok(ty);
        }

        return Err("no such type".into());
    }

    pub fn load_file(&mut self,
                     path: &Path,
                     package: Option<&RpPackage>,
                     version_req: Option<&VersionReq>)
                     -> Result<Option<(RpVersionedPackage, ast::File)>> {
        let file = parser::parse_file(&path)?;

        // TODO: remove clone requirement
        let options = Options::new(file.options.clone().into_model(path)?);
        let version = options.version()?;

        if let Some(version_req) = version_req {
            match version {
                Some(ref version) => {
                    if !version_req.matches(&version) {
                        return Ok(None);
                    }
                }
                None => {
                    if *version_req != VersionReq::any() {
                        return Ok(None);
                    }
                }
            }
        }

        if let Some(package) = package {
            if *file.package != *package {
                return Err(format!("Expected package ({}) in file {}, but was ({})",
                                   package,
                                   path.display(),
                                   *file.package)
                    .into());
            }
        }

        let versioned_package = RpVersionedPackage::new(file.package.as_ref().clone(), version);
        Ok(Some((versioned_package, file)))
    }

    pub fn process_file(&mut self,
                        path: &Path,
                        versioned_package: &RpVersionedPackage,
                        file: ast::File)
                        -> Result<()> {
        for use_decl in &file.uses {
            let package = use_decl.package.as_ref().clone();
            let version_req = use_decl.version_req.as_ref().map(|v| v.as_ref().clone());
            let package = RpRequiredPackage::new(package, version_req);

            if let Some(use_package) = self.import(&package)? {
                self.register_alias(versioned_package, use_decl, &use_package)?;
            }
        }

        let mut decls = LinkedHashMap::new();

        for decl in file.decls {
            let (decl, pos) = decl.both();
            let pos = (path.to_owned(), pos.0, pos.1);
            let decl = RpLoc::new(decl.into_model(path)?, pos);

            let custom = RpName::with_parts(vec![decl.name().to_owned()]);
            let key = versioned_package.into_type_id(custom);

            match decls.entry(key) {
                linked_hash_map::Entry::Vacant(entry) => {
                    entry.insert(Rc::new(decl));
                }
                linked_hash_map::Entry::Occupied(entry) => {
                    entry.into_mut().merge(Rc::new(decl))?;
                }
            }
        }

        let mut types = LinkedHashMap::new();

        // again, post-merge
        for (_, decl) in &decls {
            let registered_types = self.into_registered_type(versioned_package, decl.clone())?;

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

    pub fn import_file(&mut self,
                       path: &Path,
                       package: Option<&RpPackage>,
                       version_req: Option<&VersionReq>)
                       -> Result<()> {
        if let Some((versioned_package, file)) = self.load_file(path, package, version_req)? {
            self.process_file(path, &versioned_package, file)?;
        }

        Ok(())
    }

    pub fn import(&mut self, package: &RpRequiredPackage) -> Result<Option<RpVersionedPackage>> {
        debug!("import: {}", package);

        for visited in &self.visited {
            if visited.package == package.package {
                if let Some(ref version_req) = package.version_req {
                    if let Some(ref actual_version) = visited.version {
                        if version_req.matches(actual_version) {
                            return Ok(Some(visited.clone()));
                        }
                    }
                } else {
                    return Ok(Some(visited.clone()));
                }
            }
        }

        let files = self.resolver.resolve(package)?;

        let mut candidates: BTreeMap<RpVersionedPackage, Vec<_>> = BTreeMap::new();

        for path in files {
            debug!("loading: {}", path.display());

            let loaded =
                self.load_file(&path, Some(&package.package), package.version_req.as_ref())?;

            if let Some((versioned_package, file)) = loaded {
                candidates.entry(versioned_package).or_insert_with(Vec::new).push((path, file));
            }
        }

        if let Some((versioned_package, files)) = candidates.into_iter().nth(0) {
            if let Some(ref version_req) = package.version_req {
                debug!("found: {} ({})", versioned_package, version_req);
            } else {
                debug!("found: {}", versioned_package);
            }

            for (path, file) in files.into_iter() {
                debug!("in: {}", path.display());
                self.process_file(&path, &versioned_package, file)
                    .chain_err(|| format!("error when processing {}", path.display()))?;
            }

            self.visited.insert(versioned_package.clone());
            return Ok(Some(versioned_package));
        }

        Ok(None)
    }

    pub fn verify(&mut self) -> Result<()> {
        for (_, ref ty) in &self.decls {
            match ****ty {
                RpDecl::Type(ref ty) => {
                    ty.verify()?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
