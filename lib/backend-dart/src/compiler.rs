//! Backend for Dart

use crate::flavored::*;
use crate::utils::Comments;
use crate::{EXT, TYPE_SEP};
use backend::PackageProcessor;
use genco::prelude::*;
use genco::tokens::{static_literal, ItemStr};
use reproto_core::errors::Result;
use reproto_core::{Handle, Spanned};
use trans::Translated;

pub struct Compiler<'a> {
    pub env: &'a Translated<DartFlavor>,
    handle: &'a dyn Handle,
    map_of_strings: Type,
    list_of_dynamic: Type,
}

impl<'a> Compiler<'a> {
    pub fn new(env: &'a Translated<DartFlavor>, handle: &'a dyn Handle) -> Self {
        let map_of_strings = Type::map(Type::String, Type::Dynamic);
        let list_of_dynamic = Type::list(Type::Dynamic);

        Self {
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
            $(for field in fields join ($['\r']) {
                $(Comments(&field.comment))
                $(&field.ty) $(field.safe_ident());
            })
        }
    }

    /// Build a decode function.
    fn decode_fn(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        let mut vars = Vec::new();
        let data = static_literal("data");

        quote_in! { *t =>
            static $name decode(dynamic $data) {
                if (!($data is $(&self.map_of_strings))) {
                    throw $[str](expected $[const](&self.map_of_strings) but got $($data));
                }

                $(&self.map_of_strings) _data = $data;

                $(for field in fields join ($['\n']) {
                    $(ref t {
                        let id = field.safe_ident();
                        let id_dyn = &format!("{}_dyn", field.safe_ident());
                        vars.push(id);

                        let (d, e) = field.ty.decode(quote!($id_dyn));

                        quote_in!{ *t =>
                            var $id_dyn = _data[$(quoted(field.name()))];

                            $(if field.is_optional() {
                                $(&field.ty) $id = null;

                                if ($id_dyn != null) {
                                    $e
                                    $id = $d;
                                }
                            } else {
                                if ($id_dyn == null) {
                                    throw "expected value but was null";
                                }

                                $e
                                final $(&field.ty) $id = $d;
                            })
                        }
                    })
                })

                return $name($(for v in vars join (, ) => $v));
            }
        }
    }

    /// Build a tuple decode function.
    fn decode_tuple_fn(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        let mut vars = Vec::new();

        let len = fields.clone().into_iter().count();
        let data = static_literal("data");

        quote_in! { *t =>
            static $name decode(dynamic $data) {
                if (!($data is $(&self.list_of_dynamic))) {
                    throw $[str](expected $[const](&self.list_of_dynamic) but got $($data));
                }

                $(&self.list_of_dynamic) _data = $data;

                if (_data.length != $len) {
                    throw $[str](expected array of length $[const](len), but was $(_data.length));
                }

                $(for (i, field) in fields.into_iter().enumerate() join ($['\n']) {
                    $(ref t {
                        let id = field.safe_ident();
                        let id_dyn = &format!("{}_dyn", field.safe_ident());
                        let i = i.to_string();

                        let (d, e) = field.ty.decode(quote!($id_dyn));

                        quote_in!{ *t =>
                            var $id_dyn = _data[$i];

                            $(if field.is_optional() {
                                $(&field.ty) $id = null;

                                if ($id_dyn != null) {
                                    $e
                                    $id = $d;
                                }
                            } else {
                                if ($id_dyn == null) {
                                    throw "expected value but was null";
                                }

                                $e
                                final $(&field.ty) $id = $d;
                            })
                        }

                        vars.push(id);
                    })
                })

                return $name($(for v in vars join (, ) => $v));
            }
        }
    }

    /// Build a decode function for an interface.
    fn decode_interface_fn(&self, t: &mut dart::Tokens, name: &str, body: &RpInterfaceBody) {
        let data = static_literal("data");

        quote_in! { *t =>
            static $name decode(dynamic $data) {
                if (!($data is $(&self.map_of_strings))) {
                    throw $[str](expected $[const](&self.map_of_strings) but got $($data));
                }

                $(&self.map_of_strings) _data = $data;

                $(match &body.sub_type_strategy {
                    RpSubTypeStrategy::Tagged { tag, .. } => {
                        var tag = _data[$(quoted(tag.as_str()))];

                        switch (tag) {
                            $(for s in &body.sub_types {
                                case $(quoted(s.name())):
                                    return $(self.convert_type_name(&s.name)).decode(_data);
                            })
                            default:
                                throw $[str](bad tag: $tag);
                        }
                    }
                    RpSubTypeStrategy::Untagged => {
                        var keys = Set.of(_data.keys);

                        $(for s in &body.sub_types {
                            $(ref t {
                                quote_in!{ *t =>
                                    if (keys.containsAll(<String>[$(
                                        for f in s.discriminating_fields() join (, ) => $(quoted(f.name()))
                                    )])) {
                                        return $(self.convert_type_name(&s.name)).decode(_data);
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
            $(&self.map_of_strings) encode() {
                $(&self.map_of_strings) _data = Map();

                $(if let Some(tag) = tag {
                    _data[$(quoted(tag))] = $(quoted(name));
                })

                $(for field in fields join ($['\n']) {
                    $(ref t {
                        let id = &quote!(this.$(field.safe_ident()));
                        let encoded = field.ty.encode(id.clone());

                        quote_in!{ *t =>
                            $(if field.is_optional() {
                                if ($id != null) {
                                    _data[$(quoted(field.name()))] = $encoded;
                                }
                            } else {
                                _data[$(quoted(field.name()))] = $encoded;
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
            $(&self.list_of_dynamic) encode() {
                $(&self.list_of_dynamic) _data = List();

                $(for field in fields {
                    _data.add($(field.ty.encode(quote!(this.$(field.safe_ident())))));
                })

                return _data;
            }
        }
    }

    /// Setup a constructor based on the number of fields.
    fn constructor(&self, t: &mut dart::Tokens, name: &ItemStr, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            $name($(for field in fields join (, ) {
                this.$(field.safe_ident())
            }));
        }
    }
}

impl<'el> PackageProcessor<'el, DartFlavor> for Compiler<'el> {
    type Out = dart::Tokens;
    type DeclIter = trans::translated::DeclIter<'el, DartFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &dyn Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &Spanned<RpName>) -> Result<()> {
        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &RpTupleBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            $(Comments(&body.comment))
            class $name {
                $(ref t => self.type_fields(t, &body.fields))

                $(ref t => self.constructor(t, name, &body.fields))

                $(ref t => self.decode_tuple_fn(t, name, &body.fields))

                $(ref t => self.encode_tuple_fn(t, &body.fields))
            }
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &RpEnumBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);
        let value = static_literal("_value");
        let data = static_literal("data");

        quote_in! { *out =>
            $(Comments(&body.comment))
            class $name {
                final $value;
                const $name._new(this.$value);

                toString() => $[str]($[const](name).$[const](value));

                $(for v in &body.variants join ($['\r']) {
                    $(Comments(v.comment))
                    $(match v.value {
                        RpVariantValue::String(string) => {
                            static const $(v.ident()) = const $name._new($(quoted(string)));
                        }
                        RpVariantValue::Number(number) => {
                            static const $(v.ident()) = const $name._new($(display(number)));
                        }
                    })
                })

                static $name decode(dynamic $data) {
                    if (!($data is $(&body.enum_type))) {
                        throw $[str](expected $[const](&body.enum_type) but got $($data));
                    }

                    switch ($data as $(&body.enum_type)) {
                        $(for v in &body.variants join ($['\r']) {
                            case $(match v.value {
                                RpVariantValue::String(string) => $(quoted(string)),
                                RpVariantValue::Number(number) => $(display(number)),
                            }):
                                return $name.$(v.ident());
                        })
                        default:
                            throw $[str](unexpected $[const](name) value: $data);
                    }
                }

                $(&body.enum_type) encode() {
                    return $value;
                }
            }
        };

        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &RpTypeBody) -> Result<()> {
        let name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            $(Comments(&body.comment))
            class $name {
                $(ref t => self.type_fields(t, &body.fields))

                $(ref t => self.constructor(t, name, &body.fields))

                $(ref t => self.decode_fn(t, name, &body.fields))

                $(ref t => self.encode_fn(t, name, &body.fields, None))
            }
        };

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &RpInterfaceBody) -> Result<()> {
        let super_name = &self.convert_type_name(&body.name);

        quote_in! { *out =>
            $(Comments(&body.comment))
            abstract class $super_name {
                $(ref t => self.decode_interface_fn(t, super_name, body))

                $(&self.map_of_strings) encode();

                $(if backend::code_contains!(&body.codes, RpContext::Dart) {
                    $(ref t => backend::code_in!(t, &body.codes, RpContext::Dart))
                })
            }

            $(for s in &body.sub_types join ($['\n']) {
                $(ref t {
                    let name = &self.convert_type_name(&s.name);
                    let fields = body.fields.iter().chain(s.fields.iter()).cloned().collect::<Vec<_>>();

                    quote_in! { *t =>
                        $(Comments(&s.comment))
                        class $name extends $super_name {
                            $(ref t => self.type_fields(t, &fields))

                            $(ref t => self.constructor(t, name, &fields))

                            $(ref t => self.decode_fn(t, name, &fields))

                            $(match &body.sub_type_strategy {
                                RpSubTypeStrategy::Tagged { tag, .. } => {
                                    $(ref t => self.encode_fn(
                                        t,
                                        s.name(),
                                        &fields,
                                        Some(tag.as_str()),
                                    ))
                                }
                                RpSubTypeStrategy::Untagged => {
                                    $(ref t => self.encode_fn(t, s.name(), &fields, None))
                                }
                            })

                            $(if backend::code_contains!(&s.codes, RpContext::Dart) {
                                $(ref t => backend::code_in!(t, &s.codes, RpContext::Dart))
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
