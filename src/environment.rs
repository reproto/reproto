use parser::ast;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::btree_map::{Entry, OccupiedEntry};
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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

fn _find_line(path: &PathBuf, pos: ast::Pos) -> Result<(String, usize)> {
    let file = File::open(path)?;
    let mut current_pos: usize = 0;
    let mut lines: usize = 0;
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        let line = line?;
        lines += 1;

        if current_pos >= pos.0 {
            return Ok((line, lines));
        }

        current_pos += line.len() + 1;
    }

    Err("bad file position".into())
}

fn handle_occupied<'a>(_path: &PathBuf,
                       entry: OccupiedEntry<'a, TypeId, ast::Decl>,
                       decl: &ast::Decl)
                       -> Result<()> {
    entry.into_mut().merge(decl)?;
    Ok(())
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

    fn register_type(&mut self,
                     path: &PathBuf,
                     package: &ast::Package,
                     decl: &ast::Decl)
                     -> Result<()> {
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

    pub fn import(&mut self, package: &ast::Package) -> Result<()> {
        if self.visited.contains(package) {
            return Ok(());
        }

        self.visited.insert(package.clone());

        let mut files: Vec<(PathBuf, ast::File)> = Vec::new();

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

            debug!("in: {}", path.display());

            let file = parser::parse_file(&path)
                    .chain_err(|| format!("Failed to parse: {}", path.display()))?;

            if file.package != *package {
                return Err(format!("Expected package ({}) in file {}, but was ({})",
                                   package,
                                   path.display(),
                                   file.package)
                    .into());
            }

            files.push((path.clone(), file));
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

        for &(ref path, ref file) in &files {
            for use_decl in &file.uses {
                self.register_alias(package, use_decl)?;
                self.import(&use_decl.package)?;
            }

            for decl in &file.decls {
                self.register_type(path, package, decl)?;
            }
        }

        Ok(())
    }
}
