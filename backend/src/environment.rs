use super::into_model::IntoModel;
use super::naming::{FromNaming, SnakeCase};
use super::scope::Scope;
use core::{Loc, Merge, Object, Options, PathObject, RpDecl, RpFile, RpName, RpPackage,
           RpRegistered, RpRequiredPackage, RpVersionedPackage, Version, WithPos};
use errors::*;
use linked_hash_map::LinkedHashMap;
use parser;
use parser::ast::UseDecl;
use repository::Resolver;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::path::Path;
use std::rc::Rc;

/// Scoped environment for evaluating ReProto IDLs.
pub struct Environment {
    /// Global package prefix.
    package_prefix: Option<RpPackage>,
    /// Index resolver to use.
    resolver: Box<Resolver>,
    /// Memoized required packages, to avoid unecessary lookups.
    visited: HashMap<RpRequiredPackage, Option<RpVersionedPackage>>,
    /// Registered types.
    types: LinkedHashMap<RpName, RpRegistered>,
    /// All declarations.
    decls: LinkedHashMap<RpName, Rc<Loc<RpDecl>>>,
}

/// Environment containing all loaded declarations.
impl Environment {
    pub fn new(package_prefix: Option<RpPackage>, resolver: Box<Resolver>) -> Environment {
        Environment {
            package_prefix: package_prefix,
            resolver: resolver,
            visited: HashMap::new(),
            types: LinkedHashMap::new(),
            decls: LinkedHashMap::new(),
        }
    }

    /// Lookup the declaration matching the given name.
    ///
    /// Returns the registered reference, if present.
    pub fn lookup<'a>(&'a self, name: &RpName) -> Result<&'a RpRegistered> {
        let key = name.clone().without_prefix();

        if let Some(registered) = self.types.get(&key) {
            return Ok(registered);
        }

        return Err(format!("no such type: {}", name).into());
    }

    /// Import a file into the environment.
    pub fn import_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Option<RpVersionedPackage>> {
        let object = PathObject::new(path);

        if let Some((package, file)) = self.load_object(object, None, RpPackage::empty())? {
            let required = RpRequiredPackage::new(package.package.clone(), None);

            if !self.visited.contains_key(&required) {
                self.process_file(file)?;
                self.visited.insert(required, Some(package.clone()));
            }

            return Ok(Some(package));
        }

        Ok(None)
    }

    /// Import a package based on a package and version criteria.
    pub fn import(&mut self, required: &RpRequiredPackage) -> Result<Option<RpVersionedPackage>> {
        debug!("import: {}", required);

        if let Some(existing) = self.visited.get(required) {
            debug!("already loaded: {:?} ({})", existing, required);
            return Ok(existing.as_ref().cloned());
        }

        let mut candidates = BTreeMap::new();

        // find all matching objects from the resolver.
        let files = self.resolver.resolve(required)?;

        if let Some((version, object)) = files.into_iter().last() {
            debug!("loading: {}", object);

            let loaded = self.load_object(object, version, required.package.clone())?;

            if let Some((package, file)) = loaded {
                candidates.entry(package).or_insert_with(Vec::new).push(
                    file,
                );
            }
        }

        let result = if let Some((versioned, files)) = candidates.into_iter().last() {
            debug!("found: {} ({})", versioned, required);

            for file in files.into_iter() {
                self.process_file(file)?;
            }

            Some(versioned)
        } else {
            None
        };

        self.visited.insert(required.clone(), result.clone());
        Ok(result)
    }

    /// Verify all declarations.
    pub fn verify(&mut self) -> Result<()> {
        use self::RpDecl::*;

        for d in self.decls.values() {
            match ***d {
                Type(ref ty) => ty.verify()?,
                _ => {}
            }
        }

        Ok(())
    }

    /// Iterate over top level declarations of all registered objects.
    pub fn for_each_toplevel_decl<O>(&self, mut op: O) -> Result<()>
    where
        O: FnMut(Rc<Loc<RpDecl>>) -> Result<()>,
    {
        for decl in self.decls.values() {
            op(decl.clone()).with_pos(decl.pos())?;
        }

        Ok(())
    }

    /// Walks the entire tree of declarations recursively of all registered objects.
    pub fn for_each_decl<'a, O>(&'a self, mut op: O) -> Result<()>
    where
        O: FnMut(&'a Rc<Loc<RpDecl>>) -> Result<()>,
    {
        let mut queue = LinkedList::new();

        queue.extend(self.decls.values());

        while let Some(decl) = queue.pop_front() {
            op(decl).with_pos(decl.pos())?;

            for d in decl.decls() {
                queue.push_back(d);
            }
        }

        Ok(())
    }

    /// Load the provided Object into a `RpFile` and identify which package and version it belongs
    /// to.
    fn load_object<O: Into<Box<Object>>>(
        &mut self,
        object: O,
        version: Option<Version>,
        package: RpPackage,
    ) -> Result<Option<(RpVersionedPackage, RpFile)>> {
        let package = RpVersionedPackage::new(package, version);
        let object = object.into();
        let content = parser::read_reader(object.read()?)?;
        let object = Rc::new(object);

        let file = parser::parse_string(object, content.as_str())?;

        let prefixes = self.process_uses(&file.uses)?;

        let naming = match file.options.find_one_identifier("field_naming")? {
            Some(naming) => {
                let (naming, pos) = naming.take_pair();

                match naming.as_str() {
                    "upper_camel" => Some(SnakeCase::new().to_upper_camel()),
                    "lower_camel" => Some(SnakeCase::new().to_lower_camel()),
                    "upper_snake" => Some(SnakeCase::new().to_upper_snake()),
                    "lower_snake" => None,
                    _ => return Err("illegal value".into()).with_pos(pos),
                }
            }
            _ => None,
        };

        let scope = Scope::new(
            self.package_prefix.clone(),
            package.clone(),
            prefixes,
            naming,
        );

        let file = file.into_model(&scope)?;

        Ok(Some((package, file)))
    }

    /// Apply global package prefix.
    fn package_prefix(&self, package: &RpVersionedPackage) -> RpVersionedPackage {
        self.package_prefix
            .as_ref()
            .map(|prefix| prefix.join_versioned(package))
            .unwrap_or_else(|| package.clone())
    }

    /// Process use declarations found at the top of each object.
    fn process_uses(
        &mut self,
        uses: &[Loc<UseDecl>],
    ) -> Result<HashMap<String, RpVersionedPackage>> {
        use std::collections::hash_map::Entry;
        use self::ErrorKind::*;

        let mut prefixes = HashMap::new();

        for use_decl in uses {
            let package = use_decl.package.value().clone();
            let version_req = use_decl.version_req.as_ref().map(Loc::value).cloned();
            let required = RpRequiredPackage::new(package, version_req);

            let use_package = self.import(&required)?;

            if let Some(use_package) = use_package {
                let use_package = self.package_prefix(&use_package);

                if let Some(used) = use_decl.package.parts.iter().last() {
                    let alias = use_decl.alias.as_ref().map(|v| **v).unwrap_or(used);

                    match prefixes.entry(alias.to_owned()) {
                        Entry::Vacant(entry) => entry.insert(use_package.clone()),
                        Entry::Occupied(_) => {
                            return Err(format!("alias {} already in use", alias).into())
                        }
                    };
                }

                continue;
            }

            let error = "no matching package found".to_owned();
            return Err(Pos(error, use_decl.pos().into()).into());
        }

        Ok(prefixes)
    }

    /// Process, register, and merge declarations.
    ///
    /// Declarations are considered the same if they have the same qualified name.
    /// The same declarations are merged using `Merge`.
    fn process_decls<I>(&self, input: I) -> Result<LinkedHashMap<RpName, Rc<Loc<RpDecl>>>>
    where
        I: IntoIterator<Item = Loc<RpDecl>>,
    {
        use linked_hash_map::Entry::*;

        let mut decls = LinkedHashMap::new();

        for decl in input {
            match decls.entry(decl.name().clone()) {
                Vacant(entry) => {
                    entry.insert(Rc::new(decl));
                }
                Occupied(entry) => {
                    entry.into_mut().merge(Rc::new(decl))?;
                }
            }
        }

        Ok(decls)
    }

    /// Process all declarations and convert into a global collection of registered types.
    fn process_types<'a, I: 'a>(&mut self, decls: I) -> Result<LinkedHashMap<RpName, RpRegistered>>
    where
        I: IntoIterator<Item = &'a Rc<Loc<RpDecl>>>,
    {
        use linked_hash_map::Entry::*;
        use self::ErrorKind::*;

        let mut types = LinkedHashMap::new();

        for t in decls.into_iter().flat_map(|d| d.into_registered_type()) {
            let key = t.name().clone().without_prefix();

            match types.entry(key) {
                Vacant(entry) => {
                    entry.insert(t);
                }
                Occupied(entry) => {
                    return Err(RegisteredTypeConflict(entry.key().clone()).into());
                }
            }
        }

        Ok(types)
    }

    fn process_file(&mut self, file: RpFile) -> Result<()> {
        let decls = self.process_decls(file.decls)?;
        let types = self.process_types(decls.values())?;
        self.decls.extend(decls);
        self.types.extend(types);
        Ok(())
    }
}
