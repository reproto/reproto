//! A dynamically compiled and updated environment.

use ast;
use core::errors::Result;
use core::{self, Context, Diagnostics, Encoding, Handle, Import, Loc, Position, Resolved,
           ResolvedByPrefix, Resolver, RpPackage, RpRequiredPackage, RpVersionedPackage, Source,
           Span};
use env;
use manifest;
use parser;
use std::collections::Bound;
use std::collections::{hash_map, BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use ty;
use url::Url;

/// Specifies a rename.
#[derive(Debug, Clone)]
pub enum Rename {
    /// A prefix that should be name.
    Prefix { prefix: String },
    /// Rename a local type.
    LocalType {
        /// The path to the type.
        path: Vec<String>,
    },
    /// A type that was requested to be renamed.
    Type {
        /// The prefix at which the type should be looked up from, indicating that it is in
        /// a separate package.
        prefix: Option<String>,
        /// The path to the type.
        path: Vec<String>,
    },
}

/// The result of a find_rename call.
#[derive(Debug, Clone)]
pub enum RenameResult<'a> {
    /// All renames are in the same file as where the rename was requested.
    Local { ranges: &'a Vec<Range> },
    /// A package was renamed, and the range indicates the endl of the import that should be
    /// replaced.
    ImplicitPackage {
        ranges: &'a Vec<Range>,
        position: Position,
    },
    /// Multiple different URLs.
    Collections {
        ranges: Option<&'a HashMap<Url, Vec<Range>>>,
    },
    /// Not supported, only used during development.
    #[allow(unused)]
    NotSupported,
}

/// Specifies a type completion.
#[derive(Debug, Clone)]
pub enum Completion {
    /// Completions for type from a different package.
    Absolute {
        prefix: Option<String>,
        path: Vec<String>,
        suffix: Option<String>,
    },
    /// Completions for a given package.
    Package { results: BTreeSet<String> },
    /// Any type, including primitive types.
    Any { suffix: Option<String> },
}

/// Specifies a jump
#[derive(Debug, Clone)]
pub enum Jump {
    /// Perform an absolute jump.
    Absolute {
        prefix: Option<String>,
        path: Vec<String>,
    },
    /// Jump to the specified package prefix.
    Package { prefix: String },
    /// Jump to where the prefix is declared.
    Prefix { prefix: String },
}

/// Specifies a reference to some type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reference {
    package: RpVersionedPackage,
    path: Vec<String>,
}

/// The range of something.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range {
    /// Start position.
    pub start: Position,
    /// End position.
    pub end: Position,
}

impl Range {
    pub fn contains(&self, p: &Position) -> bool {
        self.start <= *p && *p <= self.end
    }
}

/// Information about a single prefix.
#[derive(Debug, Clone)]
pub struct Prefix {
    /// The span of the prefix.
    pub span: Span,
    /// The package the prefix refers to.
    pub package: RpVersionedPackage,
    /// Is this package read-only?
    pub read_only: bool,
}

/// Information about a single symbol.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Url where the symbol is located.
    pub url: Url,
    /// Range where the symbol is located.
    pub range: Range,
    /// The name of the symbol.
    pub name: String,
    /// Markdown documentation comment.
    pub comment: Option<String>,
}

impl Symbol {
    /// Convert symbol into documentation.
    pub fn to_documentation(&self) -> Option<ty::Documentation> {
        let comment = match self.comment.as_ref() {
            Some(comment) => comment,
            None => return None,
        };

        let doc = ty::MarkupContent {
            kind: ty::MarkupKind::Markdown,
            value: comment.to_string(),
        };

        Some(ty::Documentation::MarkupContent(doc))
    }
}

#[derive(Debug, Clone)]
pub struct LoadedFile {
    /// Url of the loaded file.
    pub url: Url,
    /// The package of a loaded file.
    pub package: RpVersionedPackage,
    /// Jumps available in the file.
    pub jump_triggers: BTreeMap<Position, (Range, Jump)>,
    /// Corresponding locations that have available type completions.
    pub completion_triggers: BTreeMap<Position, (Range, Completion)>,
    /// Rename locations.
    pub rename_triggers: BTreeMap<Position, (Range, Rename)>,
    /// Local reference triggers.
    pub reference_triggers: BTreeMap<Position, (Range, Reference)>,
    /// All the locations that a given prefix is present at.
    pub prefix_ranges: HashMap<String, Vec<Range>>,
    /// Implicit prefixes which _cannot_ be renamed.
    pub implicit_prefixes: HashMap<String, Position>,
    /// All prefixes that are in-scope for the file.
    /// These are defined in the use-declarations at the top of the file.
    pub prefixes: HashMap<String, Prefix>,
    /// Symbols present in the file.
    /// The key is the path that the symbol is located in.
    pub symbols: HashMap<Vec<String>, Vec<Symbol>>,
    /// Exact symbol lookup.
    pub symbol: HashMap<Vec<String>, Span>,
    /// Diagnostics for this file.
    pub diag: Diagnostics,
}

impl LoadedFile {
    pub fn new(url: Url, source: Source, package: RpVersionedPackage) -> Self {
        Self {
            url: url.clone(),
            package: package,
            jump_triggers: BTreeMap::new(),
            completion_triggers: BTreeMap::new(),
            rename_triggers: BTreeMap::new(),
            reference_triggers: BTreeMap::new(),
            prefix_ranges: HashMap::new(),
            implicit_prefixes: HashMap::new(),
            prefixes: HashMap::new(),
            symbols: HashMap::new(),
            symbol: HashMap::new(),
            diag: Diagnostics::new(source.clone()),
        }
    }

    /// Reset all state in the loaded file.
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.symbol.clear();
        self.prefixes.clear();
        self.jump_triggers.clear();
        self.completion_triggers.clear();
        self.rename_triggers.clear();
        self.reference_triggers.clear();
        self.prefix_ranges.clear();
        self.diag.clear();
    }

    /// Compute a range from a span.
    pub fn range(&self, span: Span) -> Result<Range> {
        let (start, end) = self.diag.source.span_to_range(span, Encoding::Utf16)?;
        Ok(Range { start, end })
    }

    /// Insert the specified jump.
    pub fn register_jump(&mut self, range: Range, jump: Jump) -> Result<()> {
        self.jump_triggers.insert(range.start, (range, jump));
        Ok(())
    }

    /// Set an implicit prefix.
    ///
    /// These prefixes _can not_ be renamed since they are the last part of the package.
    pub fn implicit_prefix(&mut self, prefix: &str, span: Span) -> Result<()> {
        if self.diag.source.read_only {
            return Ok(());
        }

        let (start, _) = self.diag.source.span_to_range(span, Encoding::Utf16)?;
        self.implicit_prefixes.insert(prefix.to_string(), start);
        Ok(())
    }

    /// Register a rename hook for a local type-declaration with the given path.
    ///
    /// This function does nothing if the loaded file is read-only.
    pub fn register_rename_decl(&mut self, span: Span, path: Vec<String>) -> Result<()> {
        if self.diag.source.read_only {
            return Ok(());
        }

        let (start, end) = self.diag.source.span_to_range(span, Encoding::Utf16)?;
        let range = Range { start, end };

        let rename = Rename::LocalType { path };
        self.rename_triggers.insert(start, (range, rename));
        Ok(())
    }

    /// Register a type rename.
    ///
    /// This function does nothing if the loaded file is read-only.
    pub fn register_rename_trigger(
        &mut self,
        range: Range,
        prefix: Option<String>,
        path: Vec<String>,
    ) -> Result<()> {
        if self.diag.source.read_only {
            return Ok(());
        }

        let rename = Rename::Type { prefix, path };

        self.rename_triggers.insert(range.start, (range, rename));
        Ok(())
    }

    /// Register a location that is only used to trigger a rename action, but should not be locally
    /// replaced itself.
    ///
    /// This function does nothing if the loaded file is read-only.
    pub fn register_rename_prefix_trigger(&mut self, prefix: &str, span: Span) -> Result<()> {
        if self.diag.source.read_only {
            return Ok(());
        }

        let (start, end) = self.diag.source.span_to_range(span, Encoding::Utf16)?;
        let range = Range { start, end };

        // replace the explicit rename.
        let rename = Rename::Prefix {
            prefix: prefix.to_string(),
        };

        self.rename_triggers.insert(start, (range, rename));
        Ok(())
    }

    /// Register a location that is only used to trigger a rename action.
    /// The specified span should also be replaced itself.
    ///
    /// This function does nothing if the loaded file is read-only.
    pub fn register_rename_immediate_prefix(&mut self, range: Range, prefix: &str) -> Result<()> {
        if self.diag.source.read_only {
            return Ok(());
        }

        // replace the explicit rename.
        let rename = Rename::Prefix {
            prefix: prefix.to_string(),
        };

        self.rename_triggers.insert(range.start, (range, rename));

        self.prefix_ranges
            .entry(prefix.to_string())
            .or_insert_with(Vec::new)
            .push(range);

        Ok(())
    }

    /// Register a reference.
    fn register_reference_trigger(
        &mut self,
        range: Range,
        package: RpVersionedPackage,
        path: Vec<String>,
    ) -> Result<()> {
        self.reference_triggers
            .insert(range.start, (range, Reference { package, path }));
        Ok(())
    }
}

#[derive(Clone)]
pub struct Workspace {
    /// Path of the workspace.
    pub root_path: PathBuf,
    /// Path to manifest.
    pub manifest_path: PathBuf,
    /// Packages which have been loaded through project.
    pub packages: HashMap<RpVersionedPackage, Url>,
    /// Files which have been loaded through project, including their files.
    pub files: HashMap<Url, LoadedFile>,
    /// Versioned packages that have been looked up.
    lookup: HashMap<RpRequiredPackage, (RpVersionedPackage, bool)>,
    /// Files which are currently being edited.
    pub edited_files: HashMap<Url, LoadedFile>,
    /// Type ranges to be modified when changing the name of a given type.
    pub type_ranges: HashMap<(RpVersionedPackage, Vec<String>), HashMap<Url, Vec<Range>>>,
    /// All references for a given type.
    pub references: HashMap<Reference, HashMap<Url, Vec<Range>>>,
    /// Context where to populate compiler errors.
    ctx: Rc<Context>,
}

impl Workspace {
    /// Create a new workspace from the given path.
    pub fn new<P: AsRef<Path>>(root_path: P, ctx: Rc<Context>) -> Self {
        Self {
            root_path: root_path.as_ref().to_owned(),
            manifest_path: root_path.as_ref().join(env::MANIFEST_NAME),
            packages: HashMap::new(),
            files: HashMap::new(),
            lookup: HashMap::new(),
            edited_files: HashMap::new(),
            type_ranges: HashMap::new(),
            references: HashMap::new(),
            ctx,
        }
    }

    /// Access all files in the workspace.
    pub fn files<'a>(&'a self) -> Files<'a> {
        Files {
            files: self.files.values(),
            edited_files: self.edited_files.values(),
        }
    }

    /// Access the loaded file with the given Url.
    pub fn file(&self, url: &Url) -> Option<&LoadedFile> {
        if let Some(file) = self.edited_files.get(url) {
            return Some(file);
        }

        if let Some(file) = self.files.get(url) {
            return Some(file);
        }

        None
    }

    /// Initialize the current project.
    pub fn initialize(&mut self, handle: &Handle) -> Result<()> {
        env::initialize(handle)?;
        Ok(())
    }

    /// Reload the workspace.
    pub fn reload(&mut self) -> Result<()> {
        self.packages.clear();
        self.files.clear();
        self.lookup.clear();
        self.type_ranges.clear();
        self.references.clear();

        let mut manifest = manifest::Manifest::default();

        if !self.manifest_path.is_file() {
            error!(
                "no manifest in root of workspace: {}",
                self.manifest_path.display()
            );
            return Ok(());
        }

        manifest.path = Some(self.manifest_path.to_owned());
        manifest.from_yaml(File::open(&self.manifest_path)?, env::convert_lang)?;

        let mut resolver = env::resolver(&manifest)?;

        for package in &manifest.packages {
            if let Err(e) = self.process(resolver.as_mut(), package) {
                error!("failed to process: {}: {}", package, e.display());
            }
        }

        self.try_compile(manifest)?;
        Ok(())
    }

    /// Try to compile the current environment.
    fn try_compile(&mut self, manifest: manifest::Manifest) -> Result<()> {
        let ctx = self.ctx.clone();
        ctx.clear()?;

        let lang = manifest.lang_or_nolang();
        let package_prefix = manifest.package_prefix.clone();
        let mut env = lang.into_env(ctx.clone(), package_prefix, self);

        for package in &manifest.packages {
            if let Err(e) = env.import(package) {
                debug!("failed to import: {}: {}", package, e.display());
            }
        }

        if let Err(e) = lang.compile(ctx.clone(), env, manifest) {
            // ignore and just go off diagnostics?
            debug!("compile error: {}", e.display());
        }

        return Ok(());
    }

    /// Process the given required package request.
    ///
    /// If package has been found, returns a `(package, bool)` tuple.
    /// The `package` is the exact package that was imported, and the `bool` indicates if it is
    /// read-only or not.
    fn process(
        &mut self,
        resolver: &mut Resolver,
        package: &RpRequiredPackage,
    ) -> Result<Option<(RpVersionedPackage, bool)>> {
        // need method to report errors in this stage.
        let (url, source, versioned) = {
            let entry = match self.lookup.entry(package.clone()) {
                hash_map::Entry::Occupied(e) => return Ok(Some(e.get().clone())),
                hash_map::Entry::Vacant(e) => e,
            };

            let resolved = resolver.resolve(package)?;

            let Resolved { version, source } = match resolved.into_iter().last() {
                Some(resolved) => resolved,
                None => return Ok(None),
            };

            let path = match source.path().map(|p| p.to_owned()) {
                Some(path) => path,
                None => return Ok(None),
            };

            let versioned = RpVersionedPackage::new(package.package.clone(), version);
            entry.insert((versioned.clone(), source.read_only));

            // TODO: report error through diagnostics.
            let path = match path.canonicalize() {
                Ok(path) => path,
                Err(_) => return Ok(None),
            };

            let path = path.canonicalize()
                .map_err(|e| format!("cannot canonicalize path: {}: {}", path.display(), e))?;

            let url = Url::from_file_path(&path)
                .map_err(|_| format!("cannot build url from path: {}", path.display()))?;

            (url, source, versioned)
        };

        let read_only = if let Some(mut loaded) = self.edited_files.remove(&url) {
            loaded.clear();
            self.inner_process(resolver, &mut loaded)?;

            let read_only = loaded.diag.source.read_only;

            self.edited_files.insert(url.clone(), loaded);

            read_only
        } else {
            let mut loaded = LoadedFile::new(url.clone(), source, versioned.clone());

            let read_only = loaded.diag.source.read_only;

            self.inner_process(resolver, &mut loaded)?;
            self.files.insert(url.clone(), loaded);

            read_only
        };

        self.packages.insert(versioned.clone(), url);
        Ok(Some((versioned, read_only)))
    }

    fn inner_process(&mut self, resolver: &mut Resolver, loaded: &mut LoadedFile) -> Result<()> {
        let content = {
            let mut content = String::new();
            let mut reader = loaded.diag.source.read()?;
            reader.read_to_string(&mut content)?;
            content
        };

        let file = match parser::parse(&mut loaded.diag, content.as_str()) {
            Ok(file) => file,
            Err(()) => {
                return Ok(());
            }
        };

        for u in &file.uses {
            let (u, span) = Loc::borrow_pair(u);

            let range = match u.range {
                Some(ref range) => match core::Range::parse(range.as_str()) {
                    Ok(range) => range,
                    Err(_) => continue,
                },
                None => core::Range::any(),
            };

            let package = {
                let (package, span) = Loc::borrow_pair(&u.package);

                let (start, end) = loaded.diag.source.span_to_range(span, Encoding::Utf16)?;
                let range = Range { start, end };

                let content = &content[span.start..span.end];
                let completion = self.package_completion(content, resolver)?;
                loaded
                    .completion_triggers
                    .insert(start, (range, completion));
                package
            };

            let parts = match *package {
                ast::Package::Package { ref parts } => parts,
                ast::Package::Error => {
                    continue;
                }
            };

            let endl = match u.endl {
                Some(endl) => endl,
                None => continue,
            };

            let prefix = if let Some(ref alias) = u.alias {
                // note: can be renamed!
                let (alias, span) = Loc::borrow_pair(alias);
                let range = loaded.range(span)?;
                loaded.register_rename_immediate_prefix(range, alias.as_ref())?;
                Some(alias.as_ref())
            } else {
                match parts.last() {
                    Some(suffix) => {
                        let (suffix, span) = Loc::borrow_pair(suffix);

                        loaded.implicit_prefix(suffix.as_ref(), endl)?;
                        loaded.register_rename_prefix_trigger(suffix.as_ref(), span)?;
                        Some(suffix.as_ref())
                    }
                    None => None,
                }
            };

            let package = RpPackage::new(parts.iter().map(|p| p.to_string()).collect());
            let package = RpRequiredPackage::new(package.clone(), range);
            let package = self.process(resolver, &package)?;

            if let Some(prefix) = prefix {
                let prefix = prefix.to_string();

                let range = loaded.range(span)?;

                loaded.register_jump(
                    range,
                    Jump::Package {
                        prefix: prefix.clone(),
                    },
                )?;

                if let Some((package, read_only)) = package {
                    loaded.prefixes.insert(
                        prefix,
                        Prefix {
                            span,
                            package,
                            read_only,
                        },
                    );
                };
            }
        }

        let mut queue = VecDeque::new();

        queue.extend(file.decls.iter().map(|d| (vec![], d)));

        while let Some((mut path, decl)) = queue.pop_front() {
            let comment = decl.comment();

            let comment = if !comment.is_empty() {
                Some(
                    comment
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            } else {
                None
            };

            let symbol_path = path.clone();
            path.push(decl.name().to_string());

            loaded.symbol.insert(path.clone(), Loc::span(&decl.name()));

            self.process_decl(&path, loaded, content.as_str(), decl)?;

            queue.extend(decl.decls().map(|decl| (path.clone(), decl)));

            let (name, span) = Loc::take_pair(decl.name());

            let (start, end) = loaded.diag.source.span_to_range(span, Encoding::Utf16)?;
            let range = Range { start, end };

            loaded
                .symbols
                .entry(symbol_path.clone())
                .or_insert_with(Vec::default)
                .push(Symbol {
                    url: loaded.url.clone(),
                    range,
                    name: name.to_string(),
                    comment,
                });
        }

        Ok(())
    }

    /// Process all locations assocaited with the declarations.
    ///
    /// * `completions`, locations which are applicable for type completions.
    fn process_decl<'input>(
        &mut self,
        current: &Vec<String>,
        loaded: &mut LoadedFile,
        content: &str,
        decl: &ast::Decl<'input>,
    ) -> Result<()> {
        use ast::Decl::*;

        let (_, span) = Loc::take_pair(decl.name());

        let range = loaded.range(span)?;
        let package = loaded.package.clone();

        // we don't support refactoring from read-only sources.
        if !loaded.diag.source.read_only {
            // mark the name declaration as a location that can issue a rename.
            loaded.register_rename_decl(span, current.clone())?;

            // mark the name declaration as a range that should changed when refactoring.
            self.register_type_range(loaded, range, package.clone(), current.clone())?;
        }

        // reference triggers are unconditionally set for names.
        loaded.register_reference_trigger(range, package, current.clone())?;

        match *decl {
            Type(ref ty) => for f in ty.fields() {
                self.process_ty(current, loaded, content, &f.ty)?;
            },
            Tuple(ref tuple) => for f in tuple.fields() {
                self.process_ty(current, loaded, content, &f.ty)?;
            },
            Interface(ref interface) => for f in interface.fields() {
                self.process_ty(current, loaded, content, &f.ty)?;
            },
            Enum(ref _en) => {}
            Service(ref service) => {
                for e in service.endpoints() {
                    for a in &e.arguments {
                        self.process_ty(current, loaded, content, a.channel.ty())?;
                    }

                    if let Some(response) = e.response.as_ref() {
                        self.process_ty(current, loaded, content, response.ty())?;
                    }
                }
            }
        }

        Ok(())
    }

    fn process_ty<'input>(
        &mut self,
        current: &Vec<String>,
        loaded: &mut LoadedFile,
        content: &str,
        ty: &Loc<ast::Type<'input>>,
    ) -> Result<()> {
        let (ty, span) = Loc::borrow_pair(ty);

        match *ty {
            ast::Type::Array { ref inner } => {
                self.process_ty(current, loaded, content, inner.as_ref())?;
            }
            ast::Type::Map { ref key, ref value } => {
                self.process_ty(current, loaded, content, key.as_ref())?;
                self.process_ty(current, loaded, content, value.as_ref())?;
            }
            ref ty => {
                // load jump-to definitions
                if let ast::Type::Name { ref name } = *ty {
                    self.process_name(name, current, loaded)?;
                }

                let (start, end) = loaded.diag.source.span_to_range(span, Encoding::Utf16)?;
                let range = Range { start, end };

                let content = &content[span.start..span.end];
                let completion = self.type_completion(current, content)?;

                loaded
                    .completion_triggers
                    .insert(start, (range, completion));
            }
        }

        Ok(())
    }

    /// Process the name by:
    ///
    ///  * Register all available jumps.
    ///  * Register prefix renames.
    fn process_name<'input>(
        &mut self,
        name: &Loc<ast::Name<'input>>,
        current: &Vec<String>,
        loaded: &mut LoadedFile,
    ) -> Result<()> {
        let (name, _) = Loc::borrow_pair(name);

        match *name {
            ast::Name::Relative { ref path } => {
                // path that has been traversed so far.
                let mut full_path = current.clone();

                for p in path {
                    let (p, span) = Loc::borrow_pair(p);

                    full_path.push(p.to_string());

                    // register a type range to be modified if the given name is changed.
                    let package = loaded.package.clone();

                    let range = loaded.range(span)?;

                    self.register_type_range(loaded, range, package.clone(), full_path.clone())?;
                    loaded.register_rename_trigger(range, None, full_path.clone())?;
                    self.register_reference(loaded, range, package, full_path.clone())?;

                    loaded.register_jump(
                        range,
                        Jump::Absolute {
                            prefix: None,
                            path: full_path.clone(),
                        },
                    )?;
                }
            }
            ast::Name::Absolute {
                ref prefix,
                ref path,
            } => {
                // path that has been traversed so far.
                let mut full_path = Vec::new();

                if let Some(ref prefix) = *prefix {
                    let (prefix, span) = Loc::borrow_pair(prefix);

                    let range = loaded.range(span)?;

                    // register prefix rename.
                    loaded.register_rename_immediate_prefix(range, prefix)?;

                    loaded.register_jump(
                        range,
                        Jump::Prefix {
                            prefix: prefix.to_string(),
                        },
                    )?;
                }

                let prefix = prefix.as_ref().map(|p| p.to_string());

                for p in path {
                    let (p, span) = Loc::borrow_pair(p);

                    full_path.push(p.to_string());

                    self.register_type_rename(loaded, &prefix, &full_path, span)?;

                    let range = loaded.range(span)?;

                    loaded.register_jump(
                        range,
                        Jump::Absolute {
                            prefix: prefix.clone(),
                            path: full_path.clone(),
                        },
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Handle type rename.
    fn register_type_rename(
        &mut self,
        loaded: &mut LoadedFile,
        prefix: &Option<String>,
        full_path: &Vec<String>,
        span: Span,
    ) -> Result<()> {
        // we don't support refactoring in read-only contexts
        if loaded.diag.source.read_only {
            return Ok(());
        }

        // block evaluates to an optional range indicating whether this is a legal rename
        // position or not.
        // it might be illegal if for example the prefix being referenced does not
        // exist, in which case it would be irresponsible to kick-off a rename.
        if let Some(ref p) = *prefix {
            // NOTE: uh oh, we _must_ guarantee that prefixes are loaded _before_ this
            // point. they should, but just take care that use declarations are loaded before
            // all other declarations!
            if let Some(p) = loaded.prefixes.get(p).cloned() {
                let range = loaded.range(span)?;

                if !p.read_only {
                    self.register_type_range(loaded, range, p.package.clone(), full_path.clone())?;
                    loaded.register_rename_trigger(range, prefix.clone(), full_path.clone())?;
                }

                self.register_reference(loaded, range, p.package, full_path.clone())?;
            }
        } else {
            let package = loaded.package.clone();
            let range = loaded.range(span)?;
            self.register_type_range(loaded, range, package, full_path.clone())?;
            loaded.register_rename_trigger(range, prefix.clone(), full_path.clone())?;
        }

        Ok(())
    }

    /// Register a type range that should be replaced when the given type is being renamed.
    fn register_type_range(
        &mut self,
        loaded: &mut LoadedFile,
        range: Range,
        package: RpVersionedPackage,
        path: Vec<String>,
    ) -> Result<()> {
        let key = (package, path);

        self.type_ranges
            .entry(key)
            .or_insert_with(HashMap::new)
            .entry(loaded.url.clone())
            .or_insert_with(Vec::new)
            .push(range);

        Ok(())
    }

    /// Register a reference.
    fn register_reference(
        &mut self,
        loaded: &mut LoadedFile,
        range: Range,
        package: RpVersionedPackage,
        path: Vec<String>,
    ) -> Result<()> {
        let key = Reference {
            package: package.clone(),
            path: path.clone(),
        };

        self.references
            .entry(key)
            .or_insert_with(HashMap::new)
            .entry(loaded.url.clone())
            .or_insert_with(Vec::new)
            .push(range);

        loaded.register_reference_trigger(range, package, path)?;
        Ok(())
    }

    /// Build a package completion.
    fn package_completion(&self, content: &str, resolver: &mut Resolver) -> Result<Completion> {
        debug!("package completion from {:?}", content);

        let mut parts = content.split(|c: char| c.is_whitespace());

        let content = match parts.next() {
            Some(content) => content,
            None => content,
        };

        let mut parts = content
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let suffix = parts.pop();
        let package = RpPackage::new(parts);

        let resolved = resolver.resolve_by_prefix(&package)?;

        let mut results = BTreeSet::new();

        for r in resolved {
            if let Some(value) = r.package.parts().skip(package.len()).next() {
                if let Some(suffix) = suffix.as_ref() {
                    let suffix = suffix.to_lowercase();

                    if !value.to_lowercase().starts_with(&suffix) {
                        continue;
                    }
                }

                results.insert(value.to_string());
            }
        }

        Ok(Completion::Package { results })
    }

    /// Figure out the kind of completion to support.
    fn type_completion(&self, current: &Vec<String>, content: &str) -> Result<Completion> {
        let mut it = content.split("::").peekable();

        let mut prefix = None;
        let mut suffix = None;
        let mut path = Vec::new();
        let mut first = true;

        while let Some(step) = it.next() {
            let step = step.trim();

            if it.peek().is_none() {
                suffix = Some(step.to_string());
                continue;
            }

            if !first {
                path.push(step.to_string());
                continue;
            }

            first = false;

            // relative
            if step.is_empty() {
                path.extend(current.iter().cloned());
                continue;
            }

            if step.chars().all(|c| c.is_lowercase() || c.is_numeric()) {
                prefix = Some(step.to_string());
                continue;
            }

            path.push(step.to_string());
            continue;
        }

        if prefix.is_none() && path.is_empty() {
            return Ok(Completion::Any { suffix });
        }

        return Ok(Completion::Absolute {
            prefix,
            path,
            suffix,
        });
    }

    /// Find the type completion associated with the given position.
    pub fn find_completion(
        &self,
        url: &Url,
        position: ty::Position,
    ) -> Option<(&LoadedFile, &Completion)> {
        let file = match self.file(url) {
            Some(file) => file,
            None => return None,
        };

        if let Some(value) = self.test_trigger(&file.completion_triggers, position) {
            return Some((file, value));
        }

        None
    }

    /// Find the associated jump.
    pub fn find_jump(&self, url: &Url, position: ty::Position) -> Option<(&LoadedFile, &Jump)> {
        let file = match self.file(url) {
            Some(file) => file,
            None => return None,
        };

        if let Some(value) = self.test_trigger(&file.jump_triggers, position) {
            return Some((file, value));
        }

        None
    }

    /// Find the specified rename.
    pub fn find_rename<'a>(
        &'a self,
        url: &Url,
        position: ty::Position,
    ) -> Option<RenameResult<'a>> {
        let file = match self.file(url) {
            Some(file) => file,
            None => return None,
        };

        let value = match self.test_trigger(&file.rename_triggers, position) {
            Some(value) => value,
            None => return None,
        };

        match *value {
            Rename::Prefix { ref prefix } => {
                let ranges = match file.prefix_ranges.get(prefix) {
                    Some(ranges) => ranges,
                    None => return None,
                };

                // implicit prefixes cannot be renamed.
                if let Some(position) = file.implicit_prefixes.get(prefix) {
                    return Some(RenameResult::ImplicitPackage {
                        ranges,
                        position: *position,
                    });
                }

                return Some(RenameResult::Local { ranges });
            }
            Rename::LocalType { ref path } => {
                let ranges = self.type_ranges.get(&(file.package.clone(), path.clone()));

                // look up _all_ ranges that should be replaced for the given type.
                return Some(RenameResult::Collections { ranges });
            }
            // We are referencing an imported type, so we need to resolve the prefix during lookup.
            Rename::Type {
                ref prefix,
                ref path,
            } => {
                let package = if let Some(ref prefix) = *prefix {
                    let &Prefix { ref package, .. } = match file.prefixes.get(prefix) {
                        Some(prefix) => prefix,
                        None => return None,
                    };

                    package
                } else {
                    &file.package
                };

                let ranges = self.type_ranges.get(&(package.clone(), path.clone()));

                // look up _all_ ranges that should be replaced for the given type.
                return Some(RenameResult::Collections { ranges });
            }
        }
    }

    /// Find out if there is a reference in the given location.
    pub fn find_reference<'a>(
        &'a self,
        url: &Url,
        position: ty::Position,
    ) -> Option<&'a HashMap<Url, Vec<Range>>> {
        let file = match self.file(url) {
            Some(file) => file,
            None => return None,
        };

        if let Some(reference) = self.test_trigger(&file.reference_triggers, position) {
            return self.references.get(reference);
        }

        return None;
    }

    /// Get URL to the manifest.
    pub fn manifest_url(&self) -> Result<Url> {
        let url = Url::from_file_path(&self.manifest_path)
            .map_err(|_| format!("cannot convert to url: {}", self.manifest_path.display()))?;

        Ok(url)
    }

    /// Test if the given position matches a trigger from the source.
    fn test_trigger<'a, V>(
        &self,
        source: &'a BTreeMap<Position, (Range, V)>,
        position: ty::Position,
    ) -> Option<&'a V> {
        let end = Position {
            line: position.line as usize,
            col: position.character as usize,
        };

        let mut range = source.range((Bound::Unbounded, Bound::Included(&end)));

        let (range, value) = match range.next_back() {
            Some((_, &(ref range, ref value))) => (range, value),
            None => return None,
        };

        if !range.contains(&end) {
            return None;
        }

        Some(value)
    }
}

impl Resolver for Workspace {
    /// Resolve a single package.
    fn resolve(&mut self, package: &RpRequiredPackage) -> Result<Vec<Resolved>> {
        let mut result = Vec::new();

        if let Some(&(ref looked_up, _)) = self.lookup.get(package) {
            if let Some(url) = self.packages.get(looked_up) {
                if let Some(loaded) = self.file(url) {
                    result.push(Resolved {
                        version: looked_up.version.clone(),
                        source: loaded.diag.source.clone(),
                    });
                }
            }
        }

        Ok(result)
    }

    /// Not supported for workspace.
    fn resolve_by_prefix(&mut self, _: &RpPackage) -> Result<Vec<ResolvedByPrefix>> {
        Ok(vec![])
    }
}

/// Iterator over all files.
pub struct Files<'a> {
    files: hash_map::Values<'a, Url, LoadedFile>,
    edited_files: hash_map::Values<'a, Url, LoadedFile>,
}

impl<'a> Iterator for Files<'a> {
    type Item = &'a LoadedFile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.files.next() {
            return Some(f);
        }

        self.edited_files.next()
    }
}
