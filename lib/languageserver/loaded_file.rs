//! A file that is loaded into a workspace.

use core::errors::Result;
use core::{Diagnostics, Encoding, Position, RpVersionedPackage, Source, Span};
use models::{Completion, Jump, Prefix, Range, Reference, Rename, Symbol};
use std::collections::{BTreeMap, HashMap};
use url::Url;

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
    /// All references for a given type.
    pub references: HashMap<Reference, Vec<Range>>,
    /// Type ranges to be modified when changing the name of a given type.
    pub type_ranges: HashMap<(RpVersionedPackage, Vec<String>), Vec<Range>>,
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
            references: HashMap::new(),
            type_ranges: HashMap::new(),
            symbol: HashMap::new(),
            diag: Diagnostics::new(source.clone()),
        }
    }

    /// Reset all state in the loaded file.
    pub fn clear(&mut self) {
        self.jump_triggers.clear();
        self.completion_triggers.clear();
        self.rename_triggers.clear();
        self.reference_triggers.clear();
        self.prefix_ranges.clear();
        self.implicit_prefixes.clear();
        self.prefixes.clear();
        self.symbols.clear();
        self.symbol.clear();
        self.references.clear();
        self.type_ranges.clear();
        self.diag.clear();
    }

    /// Compute a range from a span.
    pub fn range(&self, span: Span) -> Result<Range> {
        let (start, end) = self.diag.source.span_to_range(span, Encoding::Utf16)?;
        Ok(Range { start, end })
    }

    /// Insert the specified jump.
    pub fn register_jump(&mut self, range: Range, jump: Jump) {
        self.jump_triggers.insert(range.start, (range, jump));
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
    pub fn register_reference(
        &mut self,
        range: Range,
        package: RpVersionedPackage,
        path: Vec<String>,
    ) -> Result<()> {
        let key = Reference {
            package: package.clone(),
            path: path.clone(),
        };

        self.reference_triggers
            .insert(range.start, (range, key.clone()));

        self.references
            .entry(key)
            .or_insert_with(Vec::new)
            .push(range);
        Ok(())
    }

    /// Register a type range that should be replaced when the given type is being renamed.
    pub fn register_type_range(
        &mut self,
        range: Range,
        package: RpVersionedPackage,
        path: Vec<String>,
    ) -> Result<()> {
        let key = (package, path);

        self.type_ranges
            .entry(key)
            .or_insert_with(Vec::new)
            .push(range);

        Ok(())
    }

    /// Handle type rename.
    pub fn register_type_rename(
        &mut self,
        prefix: &Option<String>,
        full_path: &Vec<String>,
        span: Span,
    ) -> Result<()> {
        // we don't support refactoring in read-only contexts
        if self.diag.source.read_only {
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
            if let Some(p) = self.prefixes.get(p).cloned() {
                let range = self.range(span)?;

                if !p.read_only {
                    self.register_rename_trigger(range, prefix.clone(), full_path.clone())?;
                    self.register_type_range(range, p.package.clone(), full_path.clone())?;
                }
            }
        } else {
            let package = self.package.clone();
            let range = self.range(span)?;
            self.register_rename_trigger(range, prefix.clone(), full_path.clone())?;
            self.register_type_range(range, package, full_path.clone())?;
        }

        Ok(())
    }
}
