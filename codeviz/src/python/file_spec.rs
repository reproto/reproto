use super::element_spec::{AsElementSpec, ElementSpec};
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::name::ImportedName;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub elements: Elements,
}

impl FileSpec {
    pub fn new() -> FileSpec {
        FileSpec { elements: Elements::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }

    pub fn format(&self) -> String {
        let mut out = String::new();

        let mut imports = BTreeSet::new();

        self.elements.imports(&mut imports);

        let modules: BTreeSet<(String, Option<String>)> =
            imports.into_iter().map(|imported| (imported.module, imported.alias)).collect();

        if !modules.is_empty() {
            for (module, alias) in modules {
                out.push_str("import ");
                out.push_str(&module);

                if let Some(ref alias) = alias {
                    out.push_str(" as ");
                    out.push_str(alias);
                }

                out.push('\n');
            }

            out.push('\n');
        }

        let elements = self.elements.clone().join(ElementSpec::Spacing).as_element_spec();

        for line in elements.format("", "  ") {
            out.push_str(&line);
            out.push('\n');
        }

        out
    }
}

impl ImportReceiver for BTreeSet<ImportedName> {
    fn receive(&mut self, name: &ImportedName) {
        self.insert(name.clone());
    }
}
