use super::_type::ClassType;
use super::class_spec::ClassSpec;
use super::element_spec::ElementSpec;
use super::interface_spec::InterfaceSpec;
use super::section::{Section, Sections};
use super::imports::ImportReceiver;
use super::statement::Statement;

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct FileSpec {
    pub package: String,
    pub elements: Vec<ElementSpec>,
}

impl FileSpec {
    pub fn new(package: &str) -> FileSpec {
        FileSpec {
            package: package.to_owned(),
            elements: Vec::new(),
        }
    }

    pub fn push_class(&mut self, class: &ClassSpec) {
        self.elements.push(ElementSpec::Class(class.clone()))
    }

    pub fn push_interface(&mut self, interface: &InterfaceSpec) {
        self.elements.push(ElementSpec::Interface(interface.clone()))
    }

    pub fn format(&self) -> String {
        let mut sections = Sections::new();

        let mut package = Statement::new();
        package.push("package ");
        package.push(&self.package);

        sections.push(package);
        sections.push(Section::Spacing);

        let mut receiver: BTreeSet<ClassType> = BTreeSet::new();

        receiver.import_all(&self.elements);

        let imports: BTreeSet<ClassType> = receiver.into_iter()
            .filter(|t| t.package != "java.lang")
            .filter(|t| t.package != self.package)
            .map(|t| t.to_raw())
            .collect();

        if !imports.is_empty() {
            for t in imports {
                let mut import = Statement::new();
                import.push("import ");
                import.push(t.package);
                import.push(".");
                import.push(t.name);
                sections.push(import);
            }

            sections.push(Section::Spacing);
        }

        for element in &self.elements {
            element.add_to_sections(&mut sections);
        }

        let mut out = String::new();

        for line in sections.format(0usize, "", "  ") {
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
