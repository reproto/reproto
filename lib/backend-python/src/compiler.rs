//! Python Compiler

use crate::codegen::{ServiceAdded, ServiceCodegen};
use crate::flavored::*;
use crate::utils::BlockComment;
use crate::{Options, EXT, INIT_PY};
use backend::PackageProcessor;
use core::errors::Result;
use core::{Handle, RelativePathBuf, Spanned};
use genco::prelude::*;
use naming::{self, Naming};
use std::collections::BTreeMap;
use std::slice;
use trans::{self, Translated};

pub(crate) struct Compiler<'el> {
    pub(crate) env: &'el Translated<PythonFlavor>,
    variant_field: Spanned<RpField>,
    to_lower_snake: naming::ToLowerSnake,
    enum_enum: python::Import,
    service_generators: Vec<Box<dyn ServiceCodegen>>,
    handle: &'el dyn Handle,
}

impl<'el> Compiler<'el> {
    pub(crate) fn new(
        env: &'el Translated<PythonFlavor>,
        variant_field: Spanned<RpField>,
        options: Options,
        handle: &'el dyn Handle,
    ) -> Compiler<'el> {
        Compiler {
            env,
            variant_field,
            to_lower_snake: naming::to_lower_snake(),
            enum_enum: python::import("enum", "Enum").qualified(),
            service_generators: options.service_generators,
            handle,
        }
    }

    /// Compile the given backend.
    pub(crate) fn compile(&self) -> Result<()> {
        use genco::fmt;

        let files = self.populate_files()?;

        let handle = self.handle();

        for (package, out) in files {
            let full_path = self.setup_module_path(&package)?;

            log::debug!("+module: {}", full_path);

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let config = python::Config::default();
            let fmt =
                fmt::Config::from_lang::<Python>().with_indentation(fmt::Indentation::Space(2));

            out.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }

    fn encode_method(
        &self,
        t: &mut python::Tokens,
        fields: &[Spanned<RpField>],
        builder: python::Tokens,
        extra: Option<python::Tokens>,
    ) {
        quote_in! { *t =>
            def encode(self):
                data = #(builder.clone())()

                #(if let Some(extra) = extra {
                    #extra
                })

                #(for field in fields join (#<line>) =>
                    #(ref t => {
                        let v = &quote!(self.#(field.safe_ident()));

                        if field.is_optional() {
                            quote_in! { *t =>
                                if #v is not None:
                                    data[#(quoted(field.name()))] = #(field.ty.encode(v.clone()))
                            }
                        } else {
                            quote_in! { *t =>
                                if #v is None:
                                    raise Exception(#(quoted(format!("missing required field: {}", field.ident))))

                                data[#(quoted(field.name()))] = #(field.ty.encode(v.clone()))
                            }
                        }
                    })
                )

                return data
        }
    }

    fn encode_tuple_method<'a, I>(&self, t: &mut python::Tokens, fields: I)
    where
        I: IntoIterator<Item = &'a Spanned<RpField>>,
    {
        let mut args = Vec::new();

        quote_in! { *t =>
            def encode(self):
                #(for field in fields.into_iter() join (#<line>) {
                    if self.#(field.safe_ident()) is None:
                        raise Exception(#(quoted(format!("missing required field: {}", field.ident))))

                    #(field.safe_ident()) = #(field.ty.encode(quote!(self.#(field.safe_ident()))))
                    #(ref _ => args.push(field.safe_ident()))
                })

                return (#(for v in args join (, ) => #v))
        }
    }

    fn repr_method(&self, t: &mut python::Tokens, name: &Name, fields: &[Spanned<RpField>]) {
        use std::fmt::Write;

        let mut it = fields.into_iter().peekable();

        if it.peek().is_none() {
            quote_in! { *t =>
                def __repr__(self):
                    return #_(<#name>)
            };

            return;
        }

        let mut vars = Vec::<python::Tokens>::new();
        let mut format = String::new();

        write!(format, "<{} ", name.ident).unwrap();

        while let Some(field) = it.next() {
            write!(format, "{}:{{!r}}", field.ident).unwrap();
            vars.push(quote!(self.#(field.safe_ident())));

            if it.peek().is_some() {
                format.push_str(", ");
            }
        }

        write!(format, "{}", ">").unwrap();

        quote_in! { *t =>
            def __repr__(self):
                return #(quoted(format)).format(#(for v in vars join (, ) => #v))
        };
    }

    fn decode_method<F, I>(
        &self,
        out: &mut python::Tokens,
        name: &'el Name,
        fields: I,
        variable_fn: F,
    ) where
        F: Fn(usize, &'el RpField) -> python::Tokens,
        I: IntoIterator<Item = &'el Spanned<RpField>>,
    {
        let mut args = Vec::new();

        quote_in! { *out =>
            @staticmethod
            def decode(data):
                #(for (i, field) in fields.into_iter().enumerate() join (#<line>) =>
                    #(ref t =>
                        let n = &format!("f_{}", field.ident);
                        let var = &variable_fn(i, field);

                        if field.is_optional() {
                            quote_in! { *t =>
                                #n = None

                                if #var in data:
                                    #n = data[#var]

                                    #(if let Some(d) = field.ty.decode(n.clone(), 0) {
                                        if #n is not None:
                                            #d
                                    })
                            }
                        } else {
                            quote_in! { *t =>
                                #n = data[#(variable_fn(i, field))]

                                #(if let Some(d) = field.ty.decode(n.clone(), 0) {
                                    #d
                                })
                            }
                        }

                        args.push(n.clone());
                    )
                    #<line>
                )
                return #name(#(for a in args join (, ) => #a))
        }
    }

    fn build_constructor(&self, t: &mut python::Tokens, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            def __init__(self#(for f in fields => , #(f.safe_ident()))):
                #(if fields.is_empty() {
                    pass
                } else {
                    #(for f in fields join (#<push>) {
                        self.__#(&f.ident) = #(f.safe_ident())
                    })
                })
        }
    }

    /// Construct property accessors for reading and mutating the underlying value.
    ///
    /// This allows documentation to be generated and be made accessible for the various fields.
    fn build_accessors(&self, t: &mut python::Tokens, fields: &[Spanned<RpField>]) {
        quote_in! { *t =>
            #(for field in fields join (#<line>) {
                #(ref t {
                    let var = field.safe_ident();
                    let name = &self.to_lower_snake.convert(var);

                    quote_in! { *t =>
                        @property
                        def #name(self):
                            #(BlockComment(&field.comment))
                            return self.__#(&field.ident)

                        @#name.setter
                        def #name(self, #var):
                            self.__#(&field.ident) = #var
                    }
                })
            })
        }
    }

    fn enum_variants(&self, t: &mut python::Tokens, body: &RpEnumBody) {
        let mut args = Tokens::new();

        let mut it = body.variants.iter().peekable();

        while let Some(v) = it.next() {
            quote_in! { args =>
                (#(quoted(v.ident())), #(match &v.value {
                    RpVariantValue::String(string) => {
                        #(quoted(*string))
                    }
                    RpVariantValue::Number(number) => {
                        #(number.to_string())
                    }
                }))
            }

            if it.peek().is_some() {
                args.append(", ");
            }
        }

        quote_in!( *t =>
            #(&body.name) = #(&self.enum_enum)(#(quoted(&body.name)), [#args], type=#(&body.name))
        )
    }
}

impl<'el> PackageProcessor<'el, PythonFlavor> for Compiler<'el> {
    type Out = python::Tokens;
    type DeclIter = trans::translated::DeclIter<'el, PythonFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'el dyn Handle {
        self.handle
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        quote_in! { *out =>
            class #(&body.name):
                #(ref t => self.build_constructor(t, &body.fields))

                #(ref t => self.build_accessors(t, &body.fields))

                #(ref t => self.decode_method(t, &body.name, &body.fields, |i, _| quote!(#i)))

                #(ref t => self.encode_tuple_method(t, &body.fields))

                #(ref t => self.repr_method(t, &body.name, &body.fields))

                #(if backend::code_contains!(&body.codes, RpContext::Python) =>
                    #(ref t => backend::code_in!(t, &body.codes, RpContext::Python))
                )
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        quote_in! { *out =>
            class #(&body.name):
                #(ref t => self.build_constructor(t, slice::from_ref(&self.variant_field)))

                #(ref t => self.build_accessors(t, slice::from_ref(&self.variant_field)))

                #(ref t => encode_method(t, &self.variant_field))

                #(ref t => decode_method(t, &self.variant_field))

                #(ref t => self.repr_method(t, &body.name, slice::from_ref(&self.variant_field)))

                #(if backend::code_contains!(&body.codes, RpContext::Python) =>
                    #(ref t => backend::code_in!(t, &body.codes, RpContext::Python))
                )
        }

        return Ok(());

        fn encode_method(t: &mut python::Tokens, field: &Spanned<RpField>) {
            quote_in! { *t =>
                def encode(self):
                    return self.#(field.safe_ident())
            }
        }

        fn decode_method(t: &mut python::Tokens, field: &Spanned<RpField>) {
            quote_in! { *t =>
                @classmethod
                def decode(cls, data):
                    for value in cls.__members__.values():
                        if value.#(field.safe_ident()) == data:
                            return value

                    raise Exception("data does not match enum")
            }
        }
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        quote_in! { *out =>
            class #(&body.name):
                #(ref t => self.build_constructor(t, &body.fields))

                #(ref t => self.build_accessors(t, &body.fields))

                #(ref t => self.decode_method(t, &body.name, &body.fields, |_, field| {
                    quote!(#(quoted(field.name())))
                }))

                #(ref t => self.encode_method(t, &body.fields, quote!(dict), None))

                #(ref t => self.repr_method(t, &body.name, &body.fields))

                #(if backend::code_contains!(&body.codes, RpContext::Python) =>
                    #(ref t => backend::code_in!(t, &body.codes, RpContext::Python))
                )
        }

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        quote_in! { *out =>
            class #(&body.name):
                #(match &body.sub_type_strategy {
                    RpSubTypeStrategy::Tagged { tag, .. } => {
                        #(ref t => decode_from_tag(t, &body, tag))
                    }
                    RpSubTypeStrategy::Untagged => {
                        #(ref t => decode_from_untagged(t, &body))
                    }
                })

                #(if backend::code_contains!(&body.codes, RpContext::Python) =>
                    #(ref t => backend::code_in!(t, &body.codes, RpContext::Python))
                )

            #(for sub_type in &body.sub_types join (#<line>) {
                #(ref t {
                    let fields = body
                        .fields
                        .iter()
                        .chain(sub_type.fields.iter())
                        .cloned()
                        .collect::<Vec<_>>();

                    quote_in! { *t =>
                        class #(&sub_type.name)(#(&body.name)):
                            TYPE = #(quoted(sub_type.name()))

                            #(ref t => self.build_constructor(t, &fields))

                            #(ref t => self.build_accessors(t, &fields))

                            #(ref t => self.decode_method(t, &sub_type.name, &fields, |_, field| {
                                quote!(#(quoted(field.ident.clone())))
                            }))

                            #(match &body.sub_type_strategy {
                                RpSubTypeStrategy::Tagged { tag, .. } => {
                                    #(ref t => self.encode_method(
                                        t,
                                        &fields,
                                        quote!(dict),
                                        Some(quote!(data[#(quoted(tag.as_str()))] = #(quoted(sub_type.name())))),
                                    ))
                                }
                                RpSubTypeStrategy::Untagged => {
                                    #(ref t => self.encode_method(t, &fields, quote!(dict), None))
                                }
                            })

                            #(ref t => self.repr_method(t, &sub_type.name, &fields))

                            #(if backend::code_contains!(&sub_type.codes, RpContext::Python) =>
                                #(ref t => backend::code_in!(t, &sub_type.codes, RpContext::Python))
                            )
                    }
                })
            })
        }

        return Ok(());

        fn decode_from_tag(t: &mut python::Tokens, body: &RpInterfaceBody, tag: &str) {
            quote_in! { *t =>
                @staticmethod
                def decode(data):
                    if #(quoted(tag)) not in data:
                        raise Exception(#_(missing tag field #(tag)))

                    f_tag = data[#(quoted(tag))]

                    #(for sub_type in &body.sub_types join (#<line>) =>
                        if f_tag == #(quoted(sub_type.name())):
                            return #(&sub_type.name).decode(data)
                    )

                    raise Exception("no sub type matching tag: " + f_tag)
            }
        }

        fn decode_from_untagged(t: &mut python::Tokens, body: &RpInterfaceBody) {
            quote_in! { *t =>
                @staticmethod
                def decode(data):
                    keys = set(data.keys())

                    #(for sub_type in &body.sub_types join (#<line>) =>
                        if keys >= #(quoted_tags(sub_type.discriminating_fields())):
                            return #(&sub_type.name).decode(data)
                    )

                    raise Exception("no sub type matching the given fields: " + repr(keys))
            }
        }

        /// Return a set of quoted tags.
        fn quoted_tags<'a, F>(fields: F) -> python::Tokens
        where
            F: IntoIterator<Item = &'a Spanned<RpField>>,
        {
            let mut tags = Tokens::new();
            let mut c = 0;

            let mut it = fields.into_iter().peekable();

            while let Some(field) = it.next() {
                tags.append(quoted(field.name()));

                if it.peek().is_some() {
                    tags.append(",");
                    tags.space();
                }

                c += 1;
            }

            match c {
                0 => quote![set()],
                1 => quote![set((#tags,))],
                _ => quote![set((#tags))],
            }
        }
    }

    fn process_service(&self, out: &mut Self::Out, body: &'el RpServiceBody) -> Result<()> {
        for g in &self.service_generators {
            g.generate(ServiceAdded {
                body,
                type_body: out,
            })?;
        }

        Ok(())
    }

    fn populate_files(&self) -> Result<BTreeMap<RpPackage, python::Tokens>> {
        let mut enums = Vec::new();

        let mut files = self.do_populate_files(|decl, new, out| {
            if !new {
                out.line();
            }

            if let RpDecl::Enum(ref body) = *decl {
                enums.push(body);
            }

            Ok(())
        })?;

        // Process picked up enums.
        // These are added to the end of the file to declare enums:
        // https://docs.python.org/3/library/enum.html
        for body in enums {
            if let Some(tokens) = files.get_mut(&body.name.package) {
                tokens.line();
                self.enum_variants(tokens, &body);
            } else {
                return Err(format!("missing file for package: {}", &body.name.package).into());
            }
        }

        Ok(files)
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<RelativePathBuf> {
        let handle = self.handle();

        let mut full_path = RelativePathBuf::new();
        let mut iter = package.parts().peekable();

        while let Some(part) = iter.next() {
            full_path = full_path.join(part);

            if iter.peek().is_none() {
                continue;
            }

            if !handle.is_dir(&full_path) {
                log::debug!("+dir: {}", full_path);
                handle.create_dir_all(&full_path)?;
            }

            let init_path = full_path.join(INIT_PY);

            if !handle.is_file(&init_path) {
                log::debug!("+init: {}", init_path);
                handle.create(&init_path)?;
            }
        }

        full_path.set_extension(self.ext());
        Ok(full_path)
    }
}
