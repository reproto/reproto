//! Module that adds the @ConstructorProperties annotation to every constructor.

use backend::errors::*;
use genco::{Java, Quoted, Tokens};
use genco::java::imported;
use listeners::{ClassAdded, Listeners};

pub struct Module {
    annotation: Java<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module { annotation: imported("java.beans", "ConstructorProperties") }
    }
}

impl Listeners for Module {
    fn class_added<'a>(&self, e: &mut ClassAdded) -> Result<()> {
        let args: Tokens<Java> = e.names.iter().cloned().map(Quoted::quoted).collect();
        let a = toks![self.annotation.clone(), "({", args.join(", "), "})"];

        for c in &mut e.spec.constructors {
            c.annotation(a.clone());
        }

        Ok(())
    }
}
