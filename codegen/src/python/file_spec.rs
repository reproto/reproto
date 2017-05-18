use super::element_spec::{AsElementSpec, ElementSpec};
use super::imports::ImportReceiver;
use super::name::ImportedName;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub elements: Vec<ElementSpec>,
}

impl FileSpec {
    pub fn new() -> FileSpec {
        FileSpec { elements: Vec::new() }
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element.as_element_spec());
    }

    pub fn format(&self) -> String {
        let mut out = String::new();

        let mut imports = BTreeSet::new();

        imports.import_all(&self.elements);

        let modules: BTreeSet<String> =
            imports.into_iter().map(|imported| imported.module).collect();

        if !modules.is_empty() {
            for module in modules {
                out.push_str("import ");
                out.push_str(&module);
                out.push('\n');
            }

            out.push('\n');
        }

        for element in &self.elements {
            for line in element.format("", "  ") {
                out.push_str(&line);
                out.push('\n');
            }
        }

        out
    }
}

impl ImportReceiver for BTreeSet<ImportedName> {
    fn receive(&mut self, name: &ImportedName) {
        self.insert(name.clone());
    }
}
