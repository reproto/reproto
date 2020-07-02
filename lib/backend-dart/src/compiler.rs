//! Backend for Dart

use crate::flavored::{
    DartFlavor, RpEnumBody, RpField, RpInterfaceBody, RpName, RpServiceBody, RpTupleBody,
    RpTypeBody, Type,
};
use crate::utils::Comments;
use crate::{EXT, TYPE_SEP};
use backend::PackageProcessor;
use core::errors::*;
use core::{Handle, Spanned};
use genco::prelude::*;
use genco::tokens::ItemStr;
use trans::Translated;

pub struct Compiler<'el> {
    pub env: &'el Translated<DartFlavor>,
    handle: &'el dyn Handle,
    map_of_strings: Type,
    list_of_dynamic: Type,
}

impl<'el> Compiler<'el> {
    pub fn new(env: &'el Translated<DartFlavor>, handle: &'el dyn Handle) -> Compiler<'el> {
        let map_of_strings = Type::map(Type::String, Type::Dynamic);
        let list_of_dynamic = Type::list(Type::Dynamic);

        Compiler {
            env,
            handle,
            map_of_strings,
            list_of_dynamic,
        }
    }

    /// Convert the type name
    fn convert_type_name(&self, name: &RpName) -> ItemStr {
        ItemStr::from(name.join(TYPE_SEP))
    }

    pub fn compile(&self) -> Result<()> {
        use genco::fmt;

        let files = self.do_populate_files(|_, new, out| {
            if !new {
                out.line();
            }

            Ok(())
        })?;

        let handle = self.handle();

        for (package, out) in files {
            let full_path = self.setup_module_path(&package)?;

            log::debug!("+module: {}", full_path);

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let config = dart::Config::default();
            let fmt = fmt::Config::from_lang::<Dart>().with_indentation(fmt::Indentation::Space(2));

            out.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }

    /// Build field declarations for the given fields.
    fn type_fields(&self, t: &mut dart::Tokens, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            #(for field in fields join (#<push>) {
                #(Comments(&field.comment))
                #(&field.ty) #(field.safe_ident());
            })
        }
    }

    /// Build a decode function.
    fn decode_fn(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        let mut vars = Vec::new();

        quote_in! { *t =>
            static #name decode(dynamic _dataDyn) {
                if (!(_dataDyn is #(&self.map_of_strings))) {
                    throw #_(expected #(&self.map_of_strings) but got $_dataDyn);
                }

                #(&self.map_of_strings) _data = _dataDyn;

                #(for field in fields join (#<line>) {
                    #(ref t {
                        let id = field.safe_ident();
                        let id_dyn = &format!("{}_dyn", field.safe_ident());
                        vars.push(id);

                        let (d, e) = field.ty.decode(quote!(#id_dyn));

                        quote_in!{ *t =>
                            var #id_dyn = _data[#(quoted(field.name()))];

                            #(if field.is_optional() {
                                #(&field.ty) #id = null;

                                if (#id_dyn != null) {
                                    #e
                                    #id = #d;
                                }
                            } else {
                                if (#id_dyn == null) {
                                    throw "expected value but was null";
                                }

                                #e
                                final #(&field.ty) #id = #d;
                            })
                        }
                    })
                })

                return #name(#(for v in vars join (, ) => #v));
            }
        }
    }

    /// Build a tuple decode function.
    fn decode_tuple_fn(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        let mut vars = Vec::new();

        let len = fields.clone().into_iter().count();

        quote_in! { *t =>
            static #name decode(dynamic _dataDyn) {
                if (!(_dataDyn is #(&self.list_of_dynamic))) {
                    throw #_(expected #(&self.list_of_dynamic) but got $_dataDyn);
                }

                #(&self.list_of_dynamic) _data = _dataDyn;

                if (_data.length != #len) {
                    throw #_(expected array of length #len, but was $(_data.length));
                }

                #(for (i, field) in fields.into_iter().enumerate() join (#<line>) {
                    #(ref t {
                        let id = field.safe_ident();
                        let id_dyn = &format!("{}_dyn", field.safe_ident());
                        let i = i.to_string();

                        let (d, e) = field.ty.decode(quote!(#id_dyn));

                        quote_in!{ *t =>
                            var #id_dyn = _data[#i];

                            #(if field.is_optional() {
                                #(&field.ty) #id = null;

                                if (#id_dyn != null) {
                                    #e
                                    #id = #d;
                                }
                            } else {
                                if (#id_dyn == null) {
                                    throw "expected value but was null";
                                }

                                #e
                                final #(&field.ty) #id = #d;
                            })
                        }

                        vars.push(id);
                    })
                })

                return #name(#(for v in vars join (, ) => #v));
            }
        }
    }

    /// Build a decode function for an interface.
    fn decode_interface_fn(&self, t: &mut dart::Tokens, name: &str, body: &RpInterfaceBody) {
        quote_in! { *t =>
            static #name decode(dynamic _dataDyn) {
                if (!(_dataDyn is #(&self.map_of_strings))) {
                    throw #_(expected #(&self.map_of_strings) but got $_dataDyn);
                }

                #(&self.map_of_strings) _data = _dataDyn;

                #(match &body.sub_type_strategy {
                    core::RpSubTypeStrategy::Tagged { tag, .. } => {
                        var tag = _data[#(quoted(tag.as_str()))];

                        switch (tag) {
                            #(for s in &body.sub_types {
                                case #(quoted(s.name())):
                                    return #(self.convert_type_name(&s.name)).decode(_data);
                            })
                            default:
                                throw #_(bad tag: $tag);
                        }
                    }
                    core::RpSubTypeStrategy::Untagged => {
                        var keys = Set.of(_data.keys);

                        #(for s in &body.sub_types {
                            #(ref t {
                                quote_in!{ *t =>
                                    if (keys.containsAll(<String>[#(
                                        for f in s.discriminating_fields() join (, ) => #(quoted(f.name()))
                                    )])) {
                                        return #(self.convert_type_name(&s.name)).decode(_data);
                                    }
                                }
                            })
                        })
                    }
                })
            }
        };
    }

    /// Build an encode function.
    fn encode_fn(
        &self,
        t: &mut dart::Tokens,
        name: &str,
        fields: &[Spanned<RpField>],
        tag: Option<&str>,
    ) {
        quote_in! { *t =>
            #(&self.map_of_strings) encode() {
                #(&self.map_of_strings) _data = Map();

                #(if let Some(tag) = tag {
                    _data[#(quoted(tag))] = #(quoted(name));
                })

                #(for field in fields join (#<line>) {
                    #(ref t {
                        let id = &quote!(this.#(field.safe_ident()));
                        let encoded = field.ty.encode(id.clone());

                        quote_in!{ *t =>
                            #(if field.is_optional() {
                                if (#id != null) {
                                    _data[#(quoted(field.name()))] = #encoded;
                                }
                            } else {
                                _data[#(quoted(field.name()))] = #encoded;
                            })
                        }
                    })
                })

                return _data;
            }
        }
    }

    /// Build an encode function to encode tuples.
    fn encode_tuple_fn(&self, t: &mut dart::Tokens, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            #(&self.list_of_dynamic) encode() {
                #(&self.list_of_dynamic) _data = List();

                #(for field in fields {
                    _data.add(#(field.ty.encode(quote!(this.#(field.safe_ident())))));
                })

                return _data;
            }
        }
    }

    /// Setup a constructor based on the number of fields.
    fn constructor(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            #name(#(for field in fields join (, ) {
                this.#(field.safe_ident())
            }));
        }
    }
}

impl<'el> PackageProcessor<'el, DartFlavor, Spanned<RpName>> for Compiler<'el> {
    type Out = dart::Tokens;
    type DeclIter = trans::translated::DeclIter<'el, DartFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'el dyn Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &Spanned<RpName>) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &RpTupleBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            #(Comments(&body.comment))
            class #name {
                #(ref t => self.type_fields(t, &body.fields))

                #(ref t => self.constructor(t, name, &body.fields))

                #(ref t => self.decode_tuple_fn(t, name, &body.fields))

                #(ref t => self.encode_tuple_fn(t, &body.fields))
            }
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &RpEnumBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            #(Comments(&body.comment))
            class #name {
                final _value;
                const #name._new(this._value);

                toString() => #_(#name.$_value);

                #(for v in &body.variants join (#<push>) {
                    #(Comments(v.comment))
                    #(match v.value {
                        core::RpVariantValue::String(string) => {
                            static const #(v.ident()) = const #name._new(#(quoted(string)));
                        }
                        core::RpVariantValue::Number(number) => {
                            static const #(v.ident()) = const #name._new(#(display(number)));
                        }
                    })
                })

                static #name decode(dynamic data) {
                    if (!(data is #(&body.enum_type))) {
                        throw #_(expected $(#(&body.enum_type)) but got $data);
                    }

                    switch (data as #(&body.enum_type)) {
                        #(for v in &body.variants join (#<push>) {
                            case #(match v.value {
                                core::RpVariantValue::String(string) => #(quoted(string)),
                                core::RpVariantValue::Number(number) => #(display(number)),
                            }):
                                return #name.#(v.ident());
                        })
                        default:
                          throw #_(unexpected #name value: $data);
                    }
                }

                #(&body.enum_type) encode() {
                    return _value;
                }
            }
        };

        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &RpTypeBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            #(Comments(&body.comment))
            class #name {
                #(ref t => self.type_fields(t, &body.fields))

                #(ref t => self.constructor(t, name, &body.fields))

                #(ref t => self.decode_fn(t, name, &body.fields))

                #(ref t => self.encode_fn(t, name, &body.fields, None))
            }
        };

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &RpInterfaceBody) -> Result<()> {
        let super_name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            #(Comments(&body.comment))
            abstract class #super_name {
                #(ref t => self.decode_interface_fn(t, super_name, body))

                #(&self.map_of_strings) encode();

                #(if backend::code_contains!(&body.codes, core::RpContext::Dart) {
                    #(ref t => backend::code_in!(t, &body.codes, core::RpContext::Dart))
                })
            }

            #(for s in &body.sub_types join (#<line>) {
                #(ref t {
                    let name = &self.convert_type_name(&s.name);
                    let fields = body.fields.iter().chain(s.fields.iter()).cloned().collect::<Vec<_>>();

                    quote_in! { *t =>
                        #(Comments(&s.comment))
                        class #name extends #super_name {
                            #(ref t => self.type_fields(t, &fields))

                            #(ref t => self.constructor(t, name, &fields))

                            #(ref t => self.decode_fn(t, name, &fields))

                            #(match &body.sub_type_strategy {
                                core::RpSubTypeStrategy::Tagged { tag, .. } => {
                                    #(ref t => self.encode_fn(
                                        t,
                                        s.name(),
                                        &fields,
                                        Some(tag.as_str()),
                                    ))
                                }
                                core::RpSubTypeStrategy::Untagged => {
                                    #(ref t => self.encode_fn(t, s.name(), &fields, None))
                                }
                            })

                            #(if backend::code_contains!(&s.codes, core::RpContext::Dart) {
                                #(ref t => backend::code_in!(t, &s.codes, core::RpContext::Dart))
                            })
                        }
                    }
                })
            })
        }

        Ok(())
    }

    fn process_service(&self, _: &mut Self::Out, _: &RpServiceBody) -> Result<()> {
        Ok(())
    }
}
