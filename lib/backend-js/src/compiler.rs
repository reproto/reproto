use crate::flavored::*;
use crate::utils::{is_defined, is_not_defined};
use crate::{FileSpec, Options, EXT};
use backend::PackageProcessor;
use genco::prelude::*;
use genco::tokens::FormatInto;
use naming::Naming;
use relative_path::RelativePathBuf;
use reproto_core::errors::Result;
use reproto_core::{Handle, Span, Spanned};
use std::rc::Rc;
use trans::Translated;

pub(crate) struct Compiler<'a> {
    pub(crate) env: &'a Translated<JavaScriptFlavor>,
    handle: &'a dyn Handle,
    to_lower_snake: naming::ToLowerSnake,
    values: Tokens<JavaScript>,
    enum_name: Tokens<JavaScript>,
}

impl<'a> Compiler<'a> {
    pub(crate) fn new(
        env: &'a Translated<JavaScriptFlavor>,
        _: Options,
        handle: &'a dyn Handle,
    ) -> Self {
        Self {
            env,
            handle,
            to_lower_snake: naming::to_lower_snake(),
            values: quote!(values),
            enum_name: quote!(name),
        }
    }

    pub(crate) fn compile(&self) -> Result<()> {
        use genco::fmt;

        let files = self.do_populate_files(|_, new, out| {
            if !new {
                out.0.line();
            }

            Ok(())
        })?;

        let handle = self.handle();

        for (package, out) in files {
            let full_path = self.setup_module_path(&package)?;

            log::debug!("+module: {}", full_path);

            let path = RelativePathBuf::from(format!("{}.js", package.join("/")));

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let mut config = js::Config::default();

            if let Some(parent) = path.parent() {
                config = config.with_module_path(parent.to_owned());
            }

            let fmt =
                fmt::Config::from_lang::<JavaScript>().with_indentation(fmt::Indentation::Space(2));

            out.0.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }

    /// Build a function that throws an exception if the given value `toks` is None.
    fn throw_if_null<T>(&self, out: &mut js::Tokens, toks: T, field: &Spanned<RpField>)
    where
        T: Copy + FormatInto<JavaScript>,
    {
        quote_in! { *out =>
            if ($(is_not_defined(toks))) {
                throw new Error($(quoted(format!("{}: is a required field", field.name()))));
            }
        }
    }

    fn encode_method<'el, B, I>(
        &self,
        out: &mut Tokens<JavaScript>,
        fields: I,
        builder: B,
        extra: Option<Tokens<JavaScript>>,
    ) where
        B: FormatInto<JavaScript>,
        I: IntoIterator<Item = &'el Spanned<RpField>>,
    {
        quote_in! { *out =>
            encode() {
                const data = $builder;

                $(if let Some(extra) = extra {
                    $extra
                })

                $(for field in fields join ($['\n']) {
                    $(ref out => {
                        let field_toks = quote!(this.$(field.safe_ident()));

                        if field.is_optional() {
                            quote_in! { *out =>
                                if ($(is_defined(&field_toks))) {
                                    data[$(quoted(field.name()))] = $(field.ty.encode(field_toks));
                                }
                            }
                        } else {
                            quote_in! { *out =>
                                $(ref o => self.throw_if_null(o, &field_toks, field))

                                data[$(quoted(field.name()))] = $(field.ty.encode(field_toks));
                            }
                        }
                    })
                })

                return data;
            }
        }
    }

    fn encode_tuple_method<'el, I>(&self, out: &mut js::Tokens, fields: I)
    where
        I: IntoIterator<Item = &'el Spanned<RpField>>,
    {
        let mut values = Vec::new();

        quote_in! { *out =>
            encode() {
                $(for field in fields join ($['\n']) {
                    $(ref out => {
                        let access = quote!(this.$(field.safe_ident()));
                        self.throw_if_null(out, &access, field);
                        values.push(field.ty.encode(access));
                    })
                })

                return [$(for v in values join (, ) => $v)];
            }
        }
    }

    fn decode_enum_method(&self, out: &mut js::Tokens, name: &Name, field: &Spanned<RpField>) {
        let members = &quote!($name.$(&self.values));

        quote_in! { *out =>
            static decode(data) {
                for (let i = 0, l = $members.length; i < l; i++) {
                    const member = $members[i];

                    if (member.$(field.safe_ident()) === data) {
                        return member;
                    }
                }

                throw new Error($(quoted(format!("no value matching: "))) + data);
            }
        }
    }

    fn decode_method<'el, F, I, O>(&self, out: &mut js::Tokens, fields: I, name: &Name, var_fn: F)
    where
        F: Fn(usize, &'el Spanned<RpField>) -> O,
        I: IntoIterator<Item = &'el Spanned<RpField>>,
        O: FormatInto<JavaScript> + Copy,
    {
        let mut arguments = Vec::<Rc<String>>::new();

        quote_in! { *out =>
            static decode(data) {
                $(for (i, field) in fields.into_iter().enumerate() join ($['\n']) {
                    $(ref o {
                        let var_name = &Rc::new(format!("v_{}", field.ident));
                        arguments.push(var_name.clone());

                        let var = var_fn(i, field);

                        if field.is_optional() {
                            quote_in! { *o =>
                                let $var_name = data[$var];

                                if ($(is_defined(var_name))) {
                                    $(ref t => field.ty.decode(t, quote!($var_name)))
                                } else {
                                    $var_name = null;
                                }
                            }
                        } else {
                            quote_in! { *o =>
                                let $var_name = data[$var];

                                if ($(is_not_defined(var_name))) {
                                    throw new Error($var + ": required field");
                                }

                                $(ref t => field.ty.decode(t, quote!($var_name)))
                            }
                        }
                    })
                })

                return new $name($(for a in arguments join (, ) => $a));
            }
        }
    }

    fn field_by_name<'o>(
        _i: usize,
        field: &'o Spanned<RpField>,
    ) -> impl FormatInto<JavaScript> + 'o + Copy {
        quoted(field.name())
    }

    fn field_by_index(i: usize, _field: &Spanned<RpField>) -> impl FormatInto<JavaScript> + Copy {
        display(i)
    }

    fn build_constructor<'el, I>(&self, out: &mut js::Tokens, fields: I)
    where
        I: IntoIterator<Item = &'el Spanned<RpField>>,
    {
        let mut arguments = Vec::new();
        let mut assign = Vec::new();

        for field in fields {
            arguments.push(field.safe_ident());
            assign.push(quote!(this.$(field.safe_ident()) = $(field.safe_ident());));
        }

        quote_in! { *out =>
            constructor($(for a in arguments join (, ) => $a)) {
                $['\r']$(for a in assign join ($['\r']) => $a)
            }
        }
    }

    fn build_enum_constructor(&self, out: &mut js::Tokens, field: &RpField) {
        quote_in! { *out =>
            constructor($(&self.enum_name), $(field.safe_ident())) {
                this.$(&self.enum_name) = $(&self.enum_name);
                this.$(field.safe_ident()) = $(field.safe_ident());
            }
        }
    }

    fn enum_encode(&self, out: &mut js::Tokens, field: &Spanned<RpField>) {
        quote_in! { *out =>
            encode() {
                return this.$(field.safe_ident());
            }
        }
    }

    fn build_getters<'el, I>(&self, fields: I) -> Vec<Tokens<JavaScript>>
    where
        I: IntoIterator<Item = &'el Spanned<RpField>>,
    {
        let mut result = Vec::new();

        for field in fields {
            let name = &Rc::new(self.to_lower_snake.convert(&field.ident));

            result.push(quote! {
                function get_$(name)() {
                    return this.$name;
                }
            });
        }

        result
    }
}

impl<'a> PackageProcessor<'a, JavaScriptFlavor> for Compiler<'a> {
    type Out = FileSpec;
    type DeclIter = trans::translated::DeclIter<'a, JavaScriptFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &dyn Handle {
        self.handle
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &RpTupleBody) -> Result<()> {
        quote_in! { out.0 =>
            export class $(&body.name) {
                $(ref o => self.build_constructor(o, &body.fields))

                $(if false {
                    $(for getter in self.build_getters(&body.fields) join ($['\n']) {
                        $getter
                    })
                })

                $(ref o => self.decode_method(o, &body.fields, &body.name, Self::field_by_index))

                $(ref o => self.encode_tuple_method(o, &body.fields))

                $(if backend::code_contains!(&body.codes, RpContext::Js) {
                    $(ref o => backend::code_in!(o, &body.codes, RpContext::Js))
                })
            }
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &RpEnumBody) -> Result<()> {
        let mut values = Vec::new();

        let variant_field =
            Spanned::new(RpField::new("value", body.enum_type.clone()), Span::empty());

        quote_in! { out.0 =>
            export class $(&body.name) {
                $(ref o => self.build_enum_constructor(o, &variant_field))

                $(ref o => self.enum_encode(o, &variant_field))

                $(ref o => self.decode_enum_method(o, &body.name, &variant_field))

                $(if backend::code_contains!(&body.codes, RpContext::Js) {
                    $(ref o => backend::code_in!(o, &body.codes, RpContext::Js))
                })
            }

            $(ref o => for v in body.variants.iter() {
                values.push(quote!($(&body.name).$(v.ident())));

                quote_in! { *o =>
                    $['\r']
                    $(&body.name).$(v.ident()) = new $(&body.name)($(quoted(v.ident())), $(match v.value {
                        RpVariantValue::String(string) => {
                            $(quoted(string))
                        }
                        RpVariantValue::Number(number) => {
                            $(display(number))
                        }
                    }));
                }
            })

            $(&body.name).$(&self.values) = [$(for v in values join (, ) => $v)];
        }

        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &RpTypeBody) -> Result<()> {
        quote_in! { out.0 =>
            export class $(&body.name) {
                $(ref o => self.build_constructor(o, &body.fields))

                $(if false {
                    $(for getter in self.build_getters(&body.fields) {
                        $getter
                    })
                })

                $(ref o => self.decode_method(o, &body.fields, &body.name, Self::field_by_name))

                $(ref o => self.encode_method(o, &body.fields, "{}", None))

                $(if backend::code_contains!(&body.codes, RpContext::Js) {
                    $(ref o => backend::code_in!(o, &body.codes, RpContext::Js))
                })
            }
        }

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &RpInterfaceBody) -> Result<()> {
        quote_in! { out.0 =>
            export class $(&body.name) {
                $(match &body.sub_type_strategy {
                    RpSubTypeStrategy::Tagged { tag, .. } => {
                        $(ref o => decode(o, &body, tag.as_str()))
                    }
                    RpSubTypeStrategy::Untagged => {
                        $(ref o => decode_untagged(o, body))
                    }
                })

                $(if backend::code_contains!(&body.codes, RpContext::Js) {
                    $(ref o => backend::code_in!(o, &body.codes, RpContext::Js))
                })
            }

            $(for sub_type in &body.sub_types {
                export class $(&sub_type.name) {
                    $(ref o => self.build_constructor(o, body.fields.iter().chain(sub_type.fields.iter())))

                    $(if false {
                        $(for getter in self.build_getters(body.fields.iter().chain(sub_type.fields.iter())) {
                            $getter
                        })
                    })

                    $(ref o => {
                        self.decode_method(
                            o,
                            body.fields.iter().chain(sub_type.fields.iter()),
                            &sub_type.name,
                            Self::field_by_name,
                        )
                    })

                    $(match &body.sub_type_strategy {
                        RpSubTypeStrategy::Tagged { tag, .. } => {
                            $(ref o => {
                                self.encode_method(
                                    o,
                                    body.fields.iter().chain(sub_type.fields.iter()),
                                    "{}",
                                    Some(quote!(data[$(quoted(tag))] = $(quoted(sub_type.name()));))
                                )
                            })
                        }
                        RpSubTypeStrategy::Untagged => {
                            $(ref o => {
                                self.encode_method(o, body.fields.iter().chain(sub_type.fields.iter()), "{}", None)
                            })
                        }
                    })

                    $(if backend::code_contains!(&sub_type.codes, RpContext::Js) {
                        $(ref o => backend::code_in!(o, &sub_type.codes, RpContext::Js))
                    })
                }
            })
        }

        return Ok(());

        fn decode(out: &mut js::Tokens, body: &RpInterfaceBody, tag: &str) {
            quote_in! { *out =>
                static decode(data) {
                    const f_tag = data[$(quoted(tag))];

                    if ($(is_not_defined("f_tag"))) {
                        throw new Error($(quoted(format!("missing tag field: {}", tag))));
                    }

                    $(for sub_type in body.sub_types.iter() {
                        if (f_tag === $(quoted(sub_type.name()))) {
                            return $(&sub_type.name).decode(data);
                        }
                    })

                    throw new Error("bad sub-type: " + f_tag);
                }
            }
        }

        fn decode_untagged(out: &mut js::Tokens, body: &RpInterfaceBody) {
            quote_in! { *out =>
                static decode(data) {
                    var all = true;
                    var keys = {};

                    for (const k in data) {
                        keys[k] = true;
                    }

                    $(for sub_type in body.sub_types.iter() {
                        if ($(for f in sub_type.discriminating_fields() join ( && ) => ($(quoted(f.name())) in keys))) {
                            return $(&sub_type.name).decode(data);
                        }
                    })

                    throw new Error("no legal field combinations found");
                }
            }
        }
    }
}
