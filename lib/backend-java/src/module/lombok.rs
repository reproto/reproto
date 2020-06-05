//! Module that adds lombok annotations to generated classes.

use crate::codegen::class;
use crate::Options;
use genco::prelude::*;
use std::rc::Rc;

pub struct Module;

impl Module {
    pub fn initialize(self, options: &mut Options) {
        // lombok builds these automatically.
        options.build_getters = false;
        options.build_constructor = false;
        options.build_hash_code = false;
        options.build_equals = false;
        options.build_to_string = false;

        options.gen.class.push(Rc::new(Lombok::new()));
    }
}

pub struct Lombok {
    data: Rc<java::Import>,
}

impl Lombok {
    pub fn new() -> Lombok {
        Lombok {
            data: Rc::new(java::import("lombok", "Data")),
        }
    }
}

impl class::Codegen for Lombok {
    fn generate(&self, e: class::Args<'_>) {
        e.annotations.push(quote! {
            @#(&*self.data)
        });
    }
}
