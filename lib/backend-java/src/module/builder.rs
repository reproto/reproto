//! Module that adds fasterxml annotations to generated classes.

use crate::codegen;
use crate::Options;
use genco::prelude::*;
use std::rc::Rc;

pub struct Module;

impl Module {
    pub fn initialize(self, options: &mut Options) {
        options.gen.class.push(Rc::new(Builder::new()));
    }
}

pub struct Builder {
    optional: Rc<java::Import>,
    runtime_exception: Rc<java::Import>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            optional: Rc::new(java::import("java.util", "Optional")),
            runtime_exception: Rc::new(java::import("java.lang", "RuntimeException")),
        }
    }
}

impl codegen::class::Codegen for Builder {
    fn generate(&self, e: codegen::class::Args<'_>) {
        e.inner.push(quote! {
            public static class Builder {
                #(for f in e.fields join (#<push>) {
                    private #(f.optional_type()) #(f.safe_ident());
                })

                private Builder() {
                    #(for f in e.fields join (#<push>) {
                        this.#(f.safe_ident()) = #(&*self.optional).empty();
                    })
                }

                public #(e.ident) build() {
                    #(for f in e.fields join (#<push>) {
                        #(if f.is_required() {
                            final #(&f.ty) #(f.safe_ident()) = this.#(f.safe_ident())
                                .orElseThrow(() -> new #(&*self.runtime_exception)(#_(#(&f.ident): missing required value)));
                        })
                    })

                    return new #(e.ident)(
                        #(for f in e.fields join (,#<push>) {
                            #(if f.is_optional() {
                                this.#(f.safe_ident())
                            } else {
                                #(f.safe_ident())
                            })
                        })
                    );
                }

                #(for f in e.fields join (#<line>) {
                    public Builder #(f.safe_ident())(final #(&f.ty) #(f.safe_ident())) {
                        this.#(f.safe_ident()) = #(&*self.optional).of(#(f.safe_ident()));
                        return this;
                    }
                })
            }

            #(java::block_comment(&["Construct a new builder."]))
            public static Builder builder() {
                return new Builder();
            }
        });
    }
}
