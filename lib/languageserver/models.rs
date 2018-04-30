//! Data models that are shared for the language server.

use core::{Position, RpVersionedPackage};
use std::collections::BTreeSet;
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
        ranges: Vec<(&'a Url, &'a Vec<Range>)>,
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
        package: Option<RpVersionedPackage>,
        path: Vec<String>,
    },
    /// Jump to the specified package prefix.
    Package { package: RpVersionedPackage },
    /// Jump to where the prefix is declared.
    Prefix { prefix: String },
}

/// Specifies a reference to some type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reference {
    pub package: RpVersionedPackage,
    pub path: Vec<String>,
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

impl From<(Position, Position)> for Range {
    fn from(value: (Position, Position)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

/// Range is Copy.
impl<'a> From<&'a Range> for Range {
    fn from(range: &'a Range) -> Self {
        *range
    }
}

/// Information about a single prefix.
#[derive(Debug, Clone)]
pub struct Prefix {
    /// The range of the prefix.
    pub range: Range,
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
