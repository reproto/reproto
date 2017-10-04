//! Module that adds fasterxml annotations to generated classes.

use super::*;
use genco::{Cons, Java, Quoted, Tokens};
use genco::java::{Class, imported};

pub struct Module {
    annotation: Java<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module { annotation: imported("java.beans", "ConstructorProperties") }
    }
}

impl Listeners for Module {
    fn class_added<'a>(&self, names: &[Cons<'a>], spec: &mut Class<'a>) -> Result<()> {
        let args: Tokens<Java> = names.iter().cloned().map(Quoted::quoted).collect();
        let a = toks![self.annotation.clone(), "({", args.join(", "), "})"];

        for c in &mut spec.constructors {
            c.annotation(a.clone());
        }

        Ok(())
    }
}
