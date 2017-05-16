use parser::ast;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::btree_map::Entry;

use backend::TypeId;
use parser;

use errors::*;

const EXT: &str = "reproto";

pub struct Environment {
    paths: Vec<PathBuf>,
    visited: HashSet<ast::Package>,
    pub types: BTreeMap<TypeId, ast::Decl>,
    pub used: BTreeMap<(ast::Package, String), ast::Package>,
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

    pub fn import(&mut self, package: &ast::Package) -> Result<()> {
        if self.visited.contains(package) {
            return Ok(());
        }

        self.visited.insert(package.clone());

        let mut files: Vec<ast::File> = Vec::new();

        for path in &self.paths {
            let mut path = path.clone();

            for part in &package.parts {
                path.push(part);
            }

            path.set_extension(EXT);

            if !path.is_file() {
                continue;
            }

            let file = parser::parse_file(&path)
                    .chain_err(|| format!("Failed to parse: {}", path.display()))?;

            if file.package != *package {
                return Err(format!("Expected package ({}) in file {}, but was ({})",
                                   package,
                                   path.display(),
                                   file.package)
                    .into());
            }

            for import in &file.imports {
                if let Some(used) = import.parts.iter().last().map(Clone::clone) {
                    self.used.insert((package.clone(), used), import.clone());
                }
            }

            files.push(file);
        }

        if files.len() == 0 {
            return Err(format!("No files matching package ({})", *package).into());
        }

        for file in &files {
            for import in &file.imports {
                self.import(&import)?;
            }

            for decl in &file.decls {
                let key = (package.clone(), decl.name());

                match self.types.entry(key) {
                    Entry::Vacant(entry) => {
                        entry.insert(decl.clone());
                    }
                    Entry::Occupied(entry) => {
                        entry.into_mut().merge(decl)?;
                    }
                };
            }
        }

        Ok(())
    }
}
