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

fn find_line(path: &PathBuf, pos: ast::Pos) -> Result<(String, usize)> {
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

fn handle_occupied<'a>(path: &PathBuf,
                       entry: OccupiedEntry<'a, TypeId, ast::Decl>,
                       decl: &ast::Decl)
                       -> Result<()> {
    if let ast::Decl::Type(_) = *entry.get() {
        let (line_string, line) = find_line(path, decl.pos())?;

        let error = ErrorKind::ConflictingTypeDecl(path.to_owned(),
                                                   line_string,
                                                   line,
                                                   entry.get().clone(),
                                                   decl.clone())
            .into();

        return Err(error);
    }

    if let ast::Decl::Type(ref type_decl) = *decl {
        let (line_string, line) = find_line(path, type_decl.pos)?;

        let error = ErrorKind::ConflictingTypeDecl(path.to_owned(),
                                                   line_string,
                                                   line,
                                                   entry.get().clone(),
                                                   decl.clone())
            .into();

        return Err(error);
    }

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

    pub fn import(&mut self, package: &ast::Package) -> Result<()> {
        if self.visited.contains(package) {
            return Ok(());
        }

        self.visited.insert(package.clone());

        let mut files: Vec<(PathBuf, ast::File)> = Vec::new();

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

            files.push((path, file));
        }

        if files.len() == 0 {
            return Err(format!("No files matching package ({})", *package).into());
        }

        for &(ref path, ref file) in &files {
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
                        handle_occupied(path, entry, decl)?;
                    }
                };
            }
        }

        Ok(())
    }
}
