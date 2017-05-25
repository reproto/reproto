use backend::TypeId;
use parser::ast;
use parser;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::btree_map::{Entry, OccupiedEntry};
use std::path::{Path, PathBuf};

use errors::*;

const EXT: &str = "reproto";

pub struct Environment {
    paths: Vec<PathBuf>,
    visited: HashSet<ast::Package>,
    pub types: BTreeMap<TypeId, ast::Decl>,
    pub used: BTreeMap<(ast::Package, String), ast::Package>,
}

fn handle_occupied<'a>(path: &Path,
                       entry: OccupiedEntry<'a, TypeId, ast::Decl>,
                       decl: &ast::Decl)
                       -> Result<()> {
    let existing = entry.get().clone();

    let result = entry.into_mut()
        .merge(decl);

    result.chain_err(move || {
        let pos = decl.pos();

        parser::find_line(path, pos.0)
            .map(|(line_string, line)| {
                ErrorKind::DeclConflict(path.to_owned(), line_string, line, existing, decl.clone())
            })
            .unwrap_or_else(|e| ErrorKind::Parser(e.into()).into())
    })
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

    fn validate_tuple(&self, body: &ast::TupleBody) -> Result<()> {
        for member in &body.members {
            if let ast::Member::Field(ref field, _) = *member {
                if field.modifier == ast::Modifier::Optional {
                    return Err("Tuples must not have optional fields".into());
                }
            }
        }

        Ok(())
    }

    fn validate_decl(&self, decl: &ast::Decl) -> Result<()> {
        match *decl {
            ast::Decl::Tuple(ref body, _) => self.validate_tuple(body),
            _ => Ok(()),
        }
    }

    fn register_type(&mut self,
                     path: &Path,
                     package: &ast::Package,
                     decl: &ast::Decl)
                     -> Result<()> {
        self.validate_decl(decl)?;

        let key = (package.clone(), decl.name());

        match self.types.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(decl.clone());
            }
            Entry::Occupied(entry) => {
                handle_occupied(path, entry, decl)?;
            }
        };

        Ok(())
    }

    fn register_alias(&mut self, package: &ast::Package, use_decl: &ast::UseDecl) -> Result<()> {
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
    pub fn lookup_used(&self, package: &ast::Package, used: &str) -> Result<&ast::Package> {
        // resolve alias
        let package = self.used
            .get(&(package.clone(), used.to_owned()))
            .ok_or(format!("Missing import alias for ({})", used))?;

        // check that type actually exists?
        let key = (package.clone(), used.to_owned());
        let _ = self.types.get(&key);

        Ok(package)
    }

    pub fn import_file(&mut self, path: &Path, package: Option<&ast::Package>) -> Result<()> {
        debug!("in: {}", path.display());

        let file =
            parser::parse_file(&path).chain_err(|| format!("Failed to parse: {}", path.display()))?;

        if let Some(package) = package {
            if file.package != *package {
                return Err(format!("Expected package ({}) in file {}, but was ({})",
                                   package,
                                   path.display(),
                                   file.package)
                    .into());
            }
        }

        for use_decl in &file.uses {
            self.register_alias(&file.package, use_decl)?;
            self.import(&use_decl.package)?;
        }

        for decl in &file.decls {
            self.register_type(path, &file.package, decl)
                .chain_err(|| {
                    let pos = decl.pos();

                    parser::find_line(&path, pos.0)
                        .map(|(line_string, line)| {
                            ErrorKind::DeclError(path.to_owned(), line_string, line, decl.clone())
                        })
                        .unwrap_or_else(|e| ErrorKind::Parser(e.into()).into())
                })?;
        }

        Ok(())
    }

    pub fn import(&mut self, package: &ast::Package) -> Result<()> {
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
