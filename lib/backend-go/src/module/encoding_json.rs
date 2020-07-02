//! encoding/json module for Go

use crate::flavored::*;
use crate::{
    EnumAdded, EnumCodegen, FieldAdded, FieldCodegen, InterfaceAdded, InterfaceCodegen, Options,
    TupleAdded, TupleCodegen,
};
use backend::Initializer;
use core::errors::Result;
use genco::prelude::*;
use std::rc::Rc;

pub(crate) struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        let codegen = Rc::new(Codegen::new());
        options.field_gens.push(Box::new(codegen.clone()));
        options.enum_gens.push(Box::new(codegen.clone()));
        options.tuple_gens.push(Box::new(codegen.clone()));
        options.interface_gens.push(Box::new(codegen.clone()));
        Ok(())
    }
}

struct Codegen {
    new_error: go::Import,
    unmarshal: go::Import,
    marshal: go::Import,
    raw_message: go::Import,
}

impl Codegen {
    pub fn new() -> Codegen {
        Self {
            new_error: go::import("errors", "New"),
            unmarshal: go::import("encoding/json", "Unmarshal"),
            marshal: go::import("encoding/json", "Marshal"),
            raw_message: go::import("encoding/json", "RawMessage"),
        }
    }
}

impl FieldCodegen for Codegen {
    fn generate(&self, e: FieldAdded) -> Result<()> {
        let FieldAdded { tags, field, .. } = e;

        tags.push_str("json", field.name());

        if field.is_optional() {
            tags.push_str("json", "omitempty");
        }

        return Ok(());
    }
}

impl EnumCodegen for Codegen {
    fn generate(&self, e: EnumAdded) -> Result<()> {
        let EnumAdded {
            container,
            name,
            body,
            ..
        } = e;

        quote_in! { *container =>
            #(ref t => unmarshal_json(t, self, name, body))

            #(ref t => marshal_json(t, self, name, body))
        }

        return Ok(());

        fn unmarshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpEnumBody) {
            quote_in! { *t =>
                func (this *#name) UnmarshalJSON(b []byte) error {
                    var s #(&body.enum_type)

                    if err := #(&c.unmarshal)(b, &s); err != nil {
                        return err
                    }

                    switch s {
                    #(match &body.variants {
                        RpVariants::String { variants } => {
                            #(for v in variants {
                                case #(quoted(v.value.as_str())):
                                    *this = #(name)_#(v.ident.as_str())
                            })
                        }
                        RpVariants::Number { variants } => {
                            #(for v in variants {
                                case #(v.value.to_string()):
                                    *this = #(name)_#(v.ident.as_str())
                            })
                        }
                    })
                    default:
                        return #(&c.new_error)("bad value")
                    }

                    return nil
                }
            }
        }

        fn marshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpEnumBody) {
            quote_in! { *t =>
                func (this #name) MarshalJSON() ([]byte, error) {
                    var s #(&body.enum_type)

                    switch this {
                    #(match &body.variants {
                        RpVariants::String { variants } => {
                            #(for v in variants {
                                case #(name)_#(v.ident.as_str()):
                                    s = #(quoted(v.value.as_str()))
                            })
                        }
                        RpVariants::Number { variants } => {
                            #(for v in variants {
                                case #(name)_#(v.ident.as_str()):
                                    s = #(v.value.to_string())
                            })
                        }
                    })
                    default:
                        return nil, #(&c.new_error)("bad value")
                    }

                    return #(&c.marshal)(s)
                }
            }
        }
    }
}

impl TupleCodegen for Codegen {
    fn generate(&self, e: TupleAdded) -> Result<()> {
        let TupleAdded {
            container,
            name,
            body,
            ..
        } = e;

        quote_in! { *container =>
            #(ref t => unmarshal_json(t, self, name, body))

            #(ref t => marshal_json(t, self, name, body))
        }

        return Ok(());

        fn unmarshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpTupleBody) {
            quote_in! { *t =>
                func (this *#(name)) UnmarshalJSON(b []byte) error {
                    var array []#(&c.raw_message)

                    if err := #(&c.unmarshal)(b, &array); err != nil {
                        return err
                    }

                    #(for (i, f) in body.fields.iter().enumerate() join (#<line>) {
                        var #(f.safe_ident()) #(&f.ty)

                        if err := #(&c.unmarshal)(array[#i], &#(f.safe_ident())); err != nil {
                            return err
                        }

                        this.#(f.safe_ident()) = #(f.safe_ident())
                    })

                    return nil
                }
            }
        }

        fn marshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpTupleBody) {
            quote_in! { *t =>
                func (this #name) MarshalJSON() ([]byte, error) {
                    var array []#(&c.raw_message)

                    #(for f in &body.fields join (#<line>) {
                        #(f.safe_ident()), err := #(&c.marshal)(this.#(f.safe_ident()))

                        if err != nil {
                            return nil, err
                        }

                        array = append(array, #(f.safe_ident()))
                    })

                    return #(&c.marshal)(array)
                }
            }
        }
    }
}

impl InterfaceCodegen for Codegen {
    fn generate(&self, e: InterfaceAdded) -> Result<()> {
        let InterfaceAdded {
            container,
            name,
            body,
            ..
        } = e;

        quote_in! { *container =>
            #(ref t => unmarshal_json(t, self, name, body))

            #(ref t => marshal_json(t, self, name, body))
        }

        return Ok(());

        fn unmarshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpInterfaceBody) {
            quote_in! { *t =>
                func (this *#name) UnmarshalJSON(b []byte) error {
                    #(match &body.sub_type_strategy {
                        RpSubTypeStrategy::Tagged { tag } => {
                            #(ref t => unmarshal_tagged(t, c, body, tag))
                        }
                        RpSubTypeStrategy::Untagged => {
                            #(ref t => unmarshal_untagged(t, c, body))
                        }
                    })
                }
            };

            fn unmarshal_sub_type(t: &mut Tokens<Go>, c: &Codegen, sub_type: &RpSubType) {
                quote_in! { *t =>
                    sub := #(&sub_type.name){}

                    if err = #(&c.unmarshal)(b, &sub); err != nil {
                        return err
                    }

                    this.Value = &sub
                    return nil
                }
            }

            /// Unmarshal the envelope and extract the type field.
            fn unmarshal_tagged(
                t: &mut Tokens<Go>,
                c: &Codegen,
                body: &RpInterfaceBody,
                tag: &str,
            ) {
                quote_in! { *t =>
                    var err error
                    var ok bool
                    env := make(map[string]#(&c.raw_message))

                    if err := #(&c.unmarshal)(b, &env); err != nil {
                        return err
                    }

                    var raw_tag #(&c.raw_message)

                    if raw_tag, ok = env[#(quoted(tag))]; !ok {
                        return #(&c.new_error)("missing tag")
                    }

                    var tag string

                    if err = #(&c.unmarshal)(raw_tag, &tag); err != nil {
                        return err
                    }

                    switch (tag) {
                    #(for sub_type in &body.sub_types {
                        case #(quoted(sub_type.name())):
                            #(ref t => unmarshal_sub_type(t, c, sub_type))
                    })
                    default:
                        return #(&c.new_error)("bad tag")
                    }
                }
            }

            fn unmarshal_untagged(t: &mut Tokens<Go>, c: &Codegen, body: &RpInterfaceBody) {
                quote_in! { *t =>
                    var err error
                    env := make(map[string]#(&c.raw_message))

                    if err := #(&c.unmarshal)(b, &env); err != nil {
                        return err
                    }

                    keys := make(map[string]bool)

                    for k := range env {
                        keys[k] = true
                    }

                    var all bool
                    all = true

                    #(for sub_type in &body.sub_types join (#<line>) {
                        for _, k := range([]string{#(for r in sub_type.discriminating_fields() join (, ) => #(quoted(r.name())))}) {
                            if _, all = keys[k]; !all {
                                break
                            }
                        }

                        if all {
                            #(ref t => unmarshal_sub_type(t, c, sub_type))
                        }
                    })

                    return #(&c.new_error)("no combination of fields found")
                }
            }
        }

        fn marshal_json(t: &mut Tokens<Go>, c: &Codegen, name: &GoName, body: &RpInterfaceBody) {
            quote_in! { *t =>
                func (this #name) MarshalJSON() ([]byte, error) {
                    #(match body.sub_type_strategy {
                        RpSubTypeStrategy::Tagged { ref tag } => {
                            #(ref t => marshal_tagged(t, c, body, tag))
                        }
                        RpSubTypeStrategy::Untagged => {
                            #(ref t => marshal_untagged(t, c, body))
                        }
                    })
                }
            };

            /// Marshal the envelope and extract the type field.
            fn marshal_tagged(t: &mut Tokens<Go>, c: &Codegen, body: &RpInterfaceBody, tag: &str) {
                quote_in! { *t =>
                    var b []byte
                    var err error
                    env := make(map[string]#(&c.raw_message))

                    switch v := this.Value.(type) {
                    #(for sub_type in &body.sub_types {
                        case *#(&sub_type.name):
                            if b, err = #(&c.marshal)(v); err != nil {
                                return nil, err
                            }

                            if err = #(&c.unmarshal)(b, &env); err != nil {
                                return nil, err
                            }

                            if env[#(quoted(tag))], err = #(&c.marshal)(#(quoted(sub_type.name()))); err != nil {
                                return nil, err
                            }

                            return #(&c.marshal)(env)
                    })
                    default:
                        return nil, #(&c.new_error)(#_(#(&body.name): no sub-type set))
                    }
                }
            }

            /// Marshal the sub-type immediately.
            fn marshal_untagged(t: &mut Tokens<Go>, c: &Codegen, body: &RpInterfaceBody) {
                quote_in! { *t =>
                    switch v := this.Value.(type) {
                    #(for sub_type in &body.sub_types {
                        case *#(&sub_type.name):
                            return #(&c.marshal)(v)
                    })
                    default:
                        return nil, #(&c.new_error)(#_(#(&body.name): no sub-type set))
                    }
                }
            }
        }
    }
}
