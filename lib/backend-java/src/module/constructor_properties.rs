//! Module that adds the @ConstructorProperties annotation.

use crate::codegen::class_constructor;
use crate::Options;
use genco::prelude::*;
use std::rc::Rc;

pub struct Module;

impl Module {
    pub fn initialize(self, options: &mut Options) {
        options
            .gen
            .class_constructor
            .push(Rc::new(ConstructorProperties::new()));
    }
}

pub struct ConstructorProperties {
    annotation: Rc<java::Import>,
}

impl ConstructorProperties {
    fn new() -> ConstructorProperties {
        ConstructorProperties {
            annotation: Rc::new(java::import("java.beans", "ConstructorProperties")),
        }
    }
}

impl class_constructor::Codegen for ConstructorProperties {
    fn generate(&self, e: class_constructor::Args<'_>) {
        e.annotations.push(quote! {
            @#(&*self.annotation)({#(for f in e.fields => #_(#(&f.ident)))})
        });
    }
}
