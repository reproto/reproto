//! Module that adds the @ConstructorProperties annotation to every constructor.

use codegen::ClassCodegen;
use core::errors::*;
use genco::{Java, Quoted, Tokens};
use genco::java::imported;
use listeners::{ClassAdded, Configure, Listeners};

pub struct ConstructorProperties {
    annotation: Java<'static>,
}

impl ConstructorProperties {
    pub fn new() -> ConstructorProperties {
        ConstructorProperties {
            annotation: imported("java.beans", "ConstructorProperties"),
        }
    }
}

impl ClassCodegen for ConstructorProperties {
    fn generate(&self, e: ClassAdded) -> Result<()> {
        let args: Tokens<Java> = e.names.iter().cloned().map(Quoted::quoted).collect();
        let a = toks![self.annotation.clone(), "({", args.join(", "), "})"];

        for c in &mut e.spec.constructors {
            c.annotation(a.clone());
        }

        Ok(())
    }
}

pub struct Module;

impl Listeners for Module {
    fn configure<'a>(&self, e: Configure<'a>) {
        e.options
            .class_generators
            .push(Box::new(ConstructorProperties::new()));
    }
}
