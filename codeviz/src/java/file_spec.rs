use super::_type::ClassType;
use super::element_spec::{AsElementSpec, ElementSpec};
use super::elements::Elements;
use super::imports::{Imports, ImportReceiver};
use super::statement::Statement;

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub package: String,
    pub elements: Elements,
}

impl FileSpec {
    pub fn new(package: &str) -> FileSpec {
        FileSpec {
            package: package.to_owned(),
            elements: Elements::new(),
        }
    }

    pub fn push<E>(&mut self, element: E)
        where E: AsElementSpec
    {
        self.elements.push(element);
    }

    pub fn format(&self) -> String {
        let mut file = Elements::new();

        let mut package = Statement::new();
        package.push("package ");
        package.push(&self.package);
        package.push(";");

        file.push(package);

        let mut receiver: BTreeSet<ClassType> = BTreeSet::new();

        self.elements.imports(&mut receiver);

        let imports: BTreeSet<ClassType> = receiver.into_iter()
            .filter(|t| t.package != "java.lang")
            .filter(|t| t.package != self.package)
            .map(|t| t.to_raw())
            .collect();

        if !imports.is_empty() {
            let mut imported = Elements::new();

            for t in imports {
                let mut import = Statement::new();

                import.push("import ");
                import.push(t.package);
                import.push(".");
                import.push(t.name);
                import.push(";");

                imported.push(import);
            }

            file.push(imported);
        }

        for element in &self.elements.elements {
            file.push(element);
        }

        let file = file.join(ElementSpec::Spacing).as_element_spec();

        let mut out = String::new();

        for line in file.format("", "  ") {
            out.push_str(&line);
            out.push('\n');
        }

        out
    }
}

impl ImportReceiver for BTreeSet<ClassType> {
    fn receive(&mut self, ty: &ClassType) {
        self.insert(ty.clone());
    }
}
