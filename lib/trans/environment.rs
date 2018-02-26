use ast::{self, UseDecl};
use core::{Context, Loc, Object, Options, PathObject, Range, Resolved, Resolver, RpDecl, RpFile,
           RpName, RpPackage, RpReg, RpRequiredPackage, RpVersionedPackage, WithPos};
use core::errors::{Error, Result};
use into_model::IntoModel;
use linked_hash_map::LinkedHashMap;
use manifest::Lang;
use naming::{self, Naming};
use parser;
use scope::Scope;
use std::collections::{BTreeMap, HashMap, LinkedList, btree_map};
use std::path::Path;
use std::rc::Rc;
use std::vec;

/// Iterate over all files in the environment.
pub struct ForEachFile<'a> {
    iter: btree_map::Iter<'a, RpVersionedPackage, RpFile>,
}

impl<'a> Iterator for ForEachFile<'a> {
    type Item = (&'a RpVersionedPackage, &'a RpFile);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// Iterator over all toplevel declarations.
pub struct ToplevelDeclIter<'a> {
    it: vec::IntoIter<&'a RpDecl>,
}

impl<'a> Iterator for ToplevelDeclIter<'a> {
    type Item = &'a RpDecl;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

/// Iterator over all declarations in a file.
pub struct DeclIter<'a> {
    queue: LinkedList<&'a RpDecl>,
}

impl<'a> Iterator for DeclIter<'a> {
    type Item = &'a RpDecl;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(decl) = self.queue.pop_front() {
            self.queue.extend(decl.decls());
            Some(decl)
        } else {
            None
        }
    }
}

/// Scoped environment for evaluating reproto IDLs.
pub struct Environment {
    /// Global context for collecting errors.
    ctx: Rc<Context>,
    /// Global package prefix.
    package_prefix: Option<RpPackage>,
    /// Index resolver to use.
    resolver: Box<Resolver>,
    /// Store required packages, to avoid unnecessary lookups.
    visited: HashMap<RpRequiredPackage, Option<RpVersionedPackage>>,
    /// Registered types.
    types: LinkedHashMap<RpName, RpReg>,
    /// Files and associated declarations.
    files: BTreeMap<RpVersionedPackage, RpFile>,
    /// Keywords that need to be translated.
    keywords: Rc<HashMap<String, String>>,
    /// Whether to perform package translation or not.
    safe_packages: bool,
}

/// Environment containing all loaded declarations.
impl Environment {
    /// Construct a new, language-neutral environment.
    pub fn new(
        ctx: Rc<Context>,
        package_prefix: Option<RpPackage>,
        resolver: Box<Resolver>,
    ) -> Environment {
        Environment {
            ctx: ctx,
            package_prefix: package_prefix,
            resolver: resolver,
            visited: HashMap::new(),
            types: LinkedHashMap::new(),
            files: BTreeMap::new(),
            keywords: Rc::new(HashMap::new()),
            safe_packages: false,
        }
    }

    /// Helper to build an environment from a language specification.
    pub fn from_lang<L>(
        ctx: Rc<Context>,
        package_prefix: Option<RpPackage>,
        resolver: Box<Resolver>,
    ) -> Self
    where
        L: Lang,
    {
        let keywords = L::keywords()
            .into_iter()
            .map(|(f, t)| (f.to_string(), t.to_string()))
            .collect();

        Self::new(ctx.clone(), package_prefix.clone(), resolver)
            .with_keywords(keywords)
            .with_safe_packages(L::safe_packages())
    }

    /// Configure a new environment on how to use safe packages or not.
    pub fn with_safe_packages(self, safe_packages: bool) -> Self {
        Self {
            safe_packages: safe_packages,
            ..self
        }
    }

    /// Build the environment with the given keywords.
    pub fn with_keywords(self, keywords: HashMap<String, String>) -> Self {
        Self {
            keywords: Rc::new(keywords),
            ..self
        }
    }

    /// Lookup the declaration matching the given name.
    ///
    /// Returns the registered reference, if present.
    pub fn lookup<'a>(&'a self, name: &RpName) -> Result<&'a RpReg> {
        let key = name.clone().without_prefix();

        if let Some(registered) = self.types.get(&key) {
            return Ok(registered);
        }

        return Err(format!("no such type: {}", name).into());
    }

    /// Import a path into the environment.
    pub fn import_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        package: Option<RpVersionedPackage>,
    ) -> Result<RpVersionedPackage> {
        self.import_object(&PathObject::new(None, path), package)
    }

    /// Import an object into the environment.
    pub fn import_object(
        &mut self,
        object: &Object,
        package: Option<RpVersionedPackage>,
    ) -> Result<RpVersionedPackage> {
        let package = package.unwrap_or_else(|| RpVersionedPackage::new(RpPackage::empty(), None));
        let required = RpRequiredPackage::new(package.package.clone(), Range::any());

        if !self.visited.contains_key(&required) {
            let file = self.load_object(object, &package)?;
            self.process_file(package.clone(), file)?;
            self.visited.insert(required, Some(package.clone()));
        }

        Ok(package)
    }

    /// Import a single, structured file object.
    pub fn import_file(
        &mut self,
        file: ast::File,
        package: Option<RpVersionedPackage>,
    ) -> Result<RpVersionedPackage> {
        let package = package.unwrap_or_else(|| RpVersionedPackage::new(RpPackage::empty(), None));
        let required = RpRequiredPackage::new(package.package.clone(), Range::any());

        if !self.visited.contains_key(&required) {
            let file = self.load_file(file, &package)?;
            self.process_file(package.clone(), file)?;
            self.visited.insert(required, Some(package.clone()));
        }

        Ok(package)
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

        if let Some(Resolved { version, object }) = files.into_iter().last() {
            debug!("loading: {}", object);

            let package = RpVersionedPackage::new(required.package.clone(), version);
            let file = self.load_object(object.as_ref(), &package)?;

            candidates.entry(package).or_insert_with(Vec::new).push(
                file,
            );
        }

        let result = if let Some((versioned, files)) = candidates.into_iter().last() {
            debug!("found: {} ({})", versioned, required);

            for file in files.into_iter() {
                self.process_file(versioned.clone(), file)?;
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
        Ok(())
    }

    /// Iterate over all files.
    pub fn for_each_file(&self) -> ForEachFile {
        ForEachFile { iter: self.files.iter() }
    }

    /// Iterate over top level declarations of all registered objects.
    pub fn toplevel_decl_iter(&self) -> ToplevelDeclIter {
        let values = self.files
            .values()
            .flat_map(|f| f.decls.iter())
            .collect::<Vec<_>>();

        ToplevelDeclIter { it: values.into_iter() }
    }

    /// Walks the entire tree of declarations recursively of all registered objects.
    pub fn decl_iter(&self) -> DeclIter {
        let mut queue = LinkedList::new();
        queue.extend(self.files.values().flat_map(|f| f.decls.iter()));
        DeclIter { queue: queue }
    }

    /// Parse a naming option.
    ///
    /// Since lower_camel is default, do nothing on that case.
    fn parse_naming(&self, naming: Loc<String>) -> Result<Option<Box<Naming>>> {
        let (naming, pos) = Loc::take_pair(naming);

        let result: Option<Box<Naming>> = match naming.as_str() {
            "upper_camel" => Some(Box::new(naming::to_upper_camel())),
            "lower_camel" => Some(Box::new(naming::to_lower_camel())),
            "upper_snake" => Some(Box::new(naming::to_upper_snake())),
            "lower_snake" => None,
            _ => return Err("illegal value".into()).with_pos(pos),
        };

        Ok(result)
    }

    /// Load the provided Object into an `RpFile` without registering it to the set of visited
    /// files.
    pub fn load_object(&mut self, object: &Object, package: &RpVersionedPackage) -> Result<RpFile> {
        let object = Rc::new(object.clone_object());
        let input = parser::read_to_string(object.read()?)?;
        let file = parser::parse(object, input.as_str())?;
        self.load_file(file, package)
    }

    /// Loads the given file, without registering it to the set of visited packages.
    fn load_file(&mut self, file: ast::File, package: &RpVersionedPackage) -> Result<RpFile> {
        let prefixes = self.process_uses(&file.uses)?;

        let endpoint_naming = match file.options.find_one_identifier("endpoint_naming")? {
            Some(naming) => self.parse_naming(naming)?,
            _ => None,
        };

        let field_naming = match file.options.find_one_identifier("field_naming")? {
            Some(naming) => self.parse_naming(naming)?,
            _ => None,
        };

        let package_prefix = self.package_prefix.clone();
        let package = package.clone();

        let scope = Scope::new(
            self.ctx.clone(),
            package_prefix,
            package,
            prefixes,
            endpoint_naming,
            field_naming,
            self.keywords.clone(),
            self.safe_packages,
        );

        Ok(file.into_model(&scope)?)
    }

    /// Apply global package prefix.
    fn package_prefix(&self, package: &RpVersionedPackage) -> RpVersionedPackage {
        self.package_prefix
            .as_ref()
            .map(|prefix| prefix.join_versioned(package))
            .unwrap_or_else(|| package.clone())
    }

    /// Parse the given version requirement.
    fn parse_range(v: &Loc<String>) -> Result<Range> {
        let (value, pos) = Loc::borrow_pair(v);

        Range::parse(value)
            .map_err(|e| format!("bad version requirement: {}", e).into())
            .with_pos(pos)
    }

    /// Process use declarations found at the top of each object.
    fn process_uses(
        &mut self,
        uses: &[Loc<UseDecl>],
    ) -> Result<HashMap<String, RpVersionedPackage>> {
        use std::collections::hash_map::Entry;

        let mut prefixes = HashMap::new();

        for use_decl in uses {
            let package = Loc::value(&use_decl.package).clone();

            let range = use_decl
                .range
                .as_ref()
                .map(Self::parse_range)
                .unwrap_or_else(|| Ok(Range::any()))?;

            let required = RpRequiredPackage::new(package, range);

            let use_package = self.import(&required)?;

            if let Some(use_package) = use_package {
                let use_package = self.package_prefix(&use_package);

                if let Some(used) = use_decl.package.parts.iter().last() {
                    let alias = use_decl.alias.as_ref().map(|v| v.as_ref()).unwrap_or(used);

                    match prefixes.entry(alias.to_owned()) {
                        Entry::Vacant(entry) => entry.insert(use_package.clone()),
                        Entry::Occupied(_) => {
                            return Err(format!("alias {} already in use", alias).into())
                        }
                    };
                }

                continue;
            }

            return Err(
                Error::new(format!("no package found: {}", required))
                    .with_pos(Loc::pos(use_decl)),
            );
        }

        Ok(prefixes)
    }

    /// Process a single file, populating the environment.
    fn process_file(&mut self, package: RpVersionedPackage, file: RpFile) -> Result<()> {
        use linked_hash_map::Entry::*;

        let new_package = package.clone().with_replacements(&self.keywords);

        let file = match self.files.entry(new_package) {
            btree_map::Entry::Vacant(entry) => entry.insert(file),
            btree_map::Entry::Occupied(_) => {
                return Err(format!("package already registered: {}", package).into());
            }
        };

        for t in file.decls.iter().flat_map(|d| d.to_reg()) {
            let key = t.name().clone().without_prefix();

            debug!("new reg ty: {}", key);

            match self.types.entry(key) {
                Vacant(entry) => entry.insert(t),
                Occupied(entry) => {
                    return Err(
                        self.ctx
                            .report()
                            .err(t.pos(), "conflicting declaration")
                            .info(entry.get().pos(), "last declaration here")
                            .into(),
                    );
                }
            };
        }

        Ok(())
    }
}
