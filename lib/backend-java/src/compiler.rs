//! Java backend for reproto

use crate::flavored::*;
use crate::Options;
use genco::fmt;
use genco::prelude::*;
use genco::tokens::from_fn;
use naming::Naming;
use reproto_core::errors::Result;
use reproto_core::{Handle, RelativePathBuf, Spanned};
use trans::Translated;

#[allow(unused)]
pub(crate) struct Compiler<'a> {
    env: &'a Translated<JavaFlavor>,
    options: Options,
    to_upper: naming::ToUpperCamel,
    suppress_warnings: java::Import,
    string_builder: java::Import,
    objects: java::Import,
    object: java::Import,
    string: java::Import,
    illegal_argument: java::Import,
}

impl<'a> Compiler<'a> {
    pub(crate) fn new(env: &'a Translated<JavaFlavor>, options: Options) -> Self {
        Self {
            env,
            options,
            to_upper: naming::to_upper_camel(),
            objects: java::import("java.util", "Objects"),
            suppress_warnings: java::import("java.lang", "SuppressWarnings"),
            string_builder: java::import("java.lang", "StringBuilder"),
            object: java::import("java.lang", "Object"),
            string: java::import("java.lang", "String"),
            illegal_argument: java::import("java.lang", "IllegalArgumentException"),
        }
    }

    pub(crate) fn compile(&self, handle: &dyn Handle) -> Result<()> {
        for decl in self.env.toplevel_decl_iter() {
            let package = decl.name().package.join(".");

            let path = decl
                .name()
                .package
                .parts()
                .cloned()
                .fold(RelativePathBuf::new(), |p, part| p.join(part));

            if !handle.is_dir(&path) {
                log::debug!("+dir: {}", path);
                handle.create_dir_all(&path)?;
            }

            let path = path.join(format!("{}.java", decl.ident()));

            let mut out = java::Tokens::new();
            self.process_decl(&mut out, 0usize, decl)?;

            log::debug!("+class: {}", path);

            let fmt = fmt::Config::from_lang::<java::Java>();
            let config = java::Config::default().with_package(package);

            let mut file = handle.create(&path)?;
            let mut w = fmt::IoWriter::new(&mut file);

            out.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }

    fn field<'f>(&'f self, f: &'f Spanned<Field>) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            let mut ann = Vec::new();
            self.options.gen.class_field(f, &mut ann);

            quote_in! { *t =>
                $(for a in ann join ($['\r']) => $a)
                $(if self.options.immutable => final$[' '])$(f.field_type()) $(f.safe_ident())
            }
        })
    }

    fn constructor<'f>(
        &'f self,
        name: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_constructor {
                return;
            }

            let mut arguments = Vec::new();

            for f in fields {
                let mut ann = Vec::new();
                self.options.gen.class_constructor_arg(f, &mut ann);

                arguments.push(quote! {
                    $(for a in ann => $a$[' '])$(f.field_type()) $(f.safe_ident())
                });
            }

            let mut annotations = Vec::new();
            self.options.gen.class_constructor(fields, &mut annotations);

            quote_in! {*t =>
                $(for a in annotations join ($['\r']) => $a)
                public $name(
                    $(for a in arguments join (,$['\r']) => $a)
                ) {
                    $(for f in fields join ($['\r']) {
                        $(if !f.is_optional() && !f.ty.is_primitive() {
                            $(&self.objects).requireNonNull($(f.safe_ident()), $[str]($[const](&f.ident): must not be null));
                        })
                        this.$(f.safe_ident()) = $(f.safe_ident());
                    })
                }
            }
        })
    }

    fn process_enum(&self, t: &mut java::Tokens, depth: usize, body: &RpEnumBody) -> Result<()> {
        let mut inner = Vec::new();

        self.options
            .gen
            .enum_ty(&body.ident, &body.enum_type, &mut inner);

        quote_in! {*t =>
            $(java::block_comment(&body.comment))
            public $(if depth > 0 => static) enum $(&body.ident) {
                $(match &body.variants {
                    RpVariants::String { variants } => {
                        $(for variant in variants join (,$['\r']) {
                            $(self.to_upper.convert(variant.ident()))($(quoted(&variant.value)))
                        })
                    }
                    RpVariants::Number { variants } => {
                        $(for variant in variants join (,$['\r']) {
                            $(self.to_upper.convert(variant.ident()))($(match body.enum_type.as_primitive() {
                                Some(Primitive::Long) => $(display(&variant.value))L,
                                _ => $(display(&variant.value)),
                            }))
                        })
                    }
                });

                $(&body.enum_type) value;

                $(&body.ident)(final $(&body.enum_type) value) {
                    this.value = value;
                }

                $(for i in inner join ($['\n']) => $i)
            }
        }

        Ok(())
    }

    fn process_tuple(&self, t: &mut java::Tokens, depth: usize, body: &RpTupleBody) -> Result<()> {
        let mut inner = Vec::new();
        let mut annotations = Vec::new();
        self.options
            .gen
            .class(&body.ident, &body.fields, &mut inner, &mut annotations);
        self.options
            .gen
            .tuple(&body.ident, &body.fields, &mut inner, &mut annotations);

        quote_in! { *t =>
            $(java::block_comment(&body.comment))
            $(for a in annotations join ($['\r']) => $a)
            public $(if depth > 0 => static) class $(&body.ident) {
                $(for f in &body.fields join ($['\r']) {
                    $(self.field(f));
                })

                $(self.constructor(&body.ident, &body.fields))

                $(for f in &body.fields join ($['\n']) {
                    $(self.getter(f, false))

                    $(self.setter(f, false))
                })

                $(self.to_string(&body.ident, &body.fields))

                $(self.hash_code(&body.fields))

                $(self.equals(&body.ident, &body.fields))

                $(for i in inner join ($['\n']) => $i)

                $(code(&body.codes))

                $(for d in &body.decls join ($['\n']) {
                    $(ref t => self.process_decl(t, depth + 1, d)?)
                })
            }
        }

        Ok(())
    }

    fn process_type(&self, t: &mut java::Tokens, depth: usize, body: &RpTypeBody) -> Result<()> {
        let mut inner = Vec::new();
        let mut annotations = Vec::new();

        self.options
            .gen
            .class(&body.ident, &body.fields, &mut inner, &mut annotations);

        quote_in! { *t =>
            $(java::block_comment(&body.comment))
            $(for a in annotations join ($['\r']) => $a)
            public $(if depth > 0 => static) class $(&body.ident) {
                $(for f in &body.fields join ($['\r']) {
                    $(self.field(f));
                })

                $(self.constructor(&body.ident, &body.fields))

                $(for f in &body.fields join ($['\n']) {
                    $(self.getter(f, false))

                    $(self.setter(f, false))
                })

                $(self.to_string(&body.ident, &body.fields))

                $(self.hash_code(&body.fields))

                $(self.equals(&body.ident, &body.fields))

                $(for i in inner join ($['\n']) => $i)

                $(code(&body.codes))

                $(for d in &body.decls join ($['\n']) {
                    $(ref t => self.process_decl(t, depth + 1, d)?)
                })
            }
        };

        Ok(())
    }

    fn process_interface(
        &self,
        t: &mut java::Tokens,
        depth: usize,
        body: &RpInterfaceBody,
    ) -> Result<()> {
        let mut annotations = Vec::new();
        let mut inner = Vec::new();
        self.options.gen.interface(
            &body.ident,
            &body.sub_types,
            &body.sub_type_strategy,
            &mut annotations,
            &mut inner,
        );

        quote_in! { *t =>
            $(java::block_comment(&body.comment))
            $(for a in annotations join ($['\r']) => $a)
            public $(if depth > 0 => static) interface $(&body.ident) {
                $(for f in &body.fields join ($['\n']) {
                    $(self.getter_without_body(f))

                    $(self.setter_without_body(f))
                })

                $(code(&body.codes))

                $(for s in &body.sub_types join ($['\n']) {
                    $(ref t {
                        let fields = body.fields.iter().chain(&s.fields).cloned().collect::<Vec<_>>();
                        let mut inner = Vec::new();
                        let mut annotations = Vec::new();

                        self.options.gen.class(&s.ident, &fields, &mut inner, &mut annotations);
                        self.options.gen
                            .interface_sub_type(&body.sub_type_strategy, &mut annotations);

                        quote_in!{*t =>
                            $(java::block_comment(&s.comment))
                            $(for a in annotations join ($['\r']) => $a)
                            public static class $(&s.ident) implements $(&body.ident) {
                                $(for f in &fields join ($['\r']) {
                                    $(self.field(f));
                                })

                                $(self.constructor(&s.ident, &fields))

                                $(for f in &body.fields join ($['\n']) {
                                    $(self.getter(f, true))

                                    $(self.setter(f, true))
                                })

                                $(for f in &s.fields join ($['\n']) {
                                    $(self.getter(f, false))

                                    $(self.setter(f, false))
                                })

                                $(self.to_string(&s.ident, &fields))

                                $(self.hash_code(&fields))

                                $(self.equals(&s.ident, &fields))

                                $(for i in inner join ($['\n']) => $i)

                                $(code(&s.codes))

                                $(for d in &s.decls join ($['\n']) {
                                    $(ref t => self.process_decl(t, depth + 2, d)?)
                                })
                            }
                        }
                    })
                });

                $(for d in &body.decls join ($['\n']) {
                    $(ref t => self.process_decl(t, depth + 1, d)?)
                })

                $(for i in inner join ($['\n']) => $i)
            }
        }

        Ok(())
    }

    fn process_service(&self, _: &mut java::Tokens, _: usize, _: &RpServiceBody) -> Result<()> {
        Ok(())
    }

    fn process_decl(&self, t: &mut java::Tokens, depth: usize, decl: &RpDecl) -> Result<()> {
        match decl {
            RpDecl::Interface(interface) => {
                self.process_interface(t, depth, interface)?;
            }
            RpDecl::Type(ty) => {
                self.process_type(t, depth, ty)?;
            }
            RpDecl::Tuple(ty) => {
                self.process_tuple(t, depth, ty)?;
            }
            RpDecl::Enum(ty) => {
                self.process_enum(t, depth, ty)?;
            }
            RpDecl::Service(ty) => {
                self.process_service(t, depth, ty)?;
            }
        }

        Ok(())
    }

    /// Create a new setter method without a body.
    fn setter_without_body<'f>(&'f self, f: &'f Field) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_setters || self.options.immutable {
                return;
            }

            let name = self.to_upper.convert(&f.ident);

            quote_in! {*t =>
                public void set$name(final $(&f.ty) $(f.safe_ident()));
            }
        })
    }

    fn setter<'f>(&'f self, f: &'f Field, is_override: bool) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_setters || self.options.immutable {
                return;
            }

            let ident = f.safe_ident();
            let name = self.to_upper.convert(&f.ident);

            quote_in! { *t =>
                $(if is_override => @Override)
                public void set$name(final $(&f.ty) $ident) {
                    this.$ident = $ident;
                }
            }
        })
    }

    /// Create a new getter method without a body.
    fn getter_without_body<'f>(&'f self, f: &'f Field) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_getters {
                return;
            }

            let name = self.to_upper.convert(&f.ident);

            // Avoid `getClass`, a common built-in method for any Object.
            let name = match name.as_str() {
                "Class" => "Class_",
                accessor => accessor,
            };

            quote_in! {*t =>
                $(java::block_comment(&f.comment))
                public $(f.field_type()) get$name();
            }
        })
    }

    /// Build a new complete getter.
    fn getter<'f>(
        &'f self,
        f: &'f Spanned<Field>,
        is_override: bool,
    ) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_getters {
                return;
            }

            let mut ann = Vec::new();
            self.options.gen.class_getter(f, &mut ann);

            let ident = f.safe_ident();
            let name = self.to_upper.convert(&f.ident);

            // Avoid `getClass`, a common built-in method for any Object.
            let name = match name.as_str() {
                "Class" => "Class_",
                accessor => accessor,
            };

            quote_in! { *t =>
                $(java::block_comment(&f.comment))
                $(for a in ann join ($['\r']) => $a)
                $(if is_override => @Override)
                public $(f.field_type()) get$name() {
                    return this.$ident;
                }
            }
        })
    }

    /// Build a toString function.
    fn to_string<'f>(
        &'f self,
        name: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_to_string {
                return;
            }

            if fields.is_empty() {
                quote_in! { *t =>
                    @Override
                    public String toString() {
                        return $[str]($[const](name)());
                    }
                }

                return;
            }

            let string_builder = &self.string_builder;

            quote_in! { *t =>
                @Override
                public String toString() {
                    final $string_builder b = new $string_builder();

                    b.append($[str]($[const](name)$[const]("(")));
                    $(for f in fields join ($['\r']b.append(", ");$['\r']) {
                        b.append($[str]($[const](&f.ident)=));
                        b.append($(f.to_string(quote!(this.$(f.safe_ident())))));
                    })
                    b.append(")");

                    return b.toString();
                }
            }
        })
    }

    /// Build a hashCode function.
    fn hash_code<'f>(&'f self, fields: &'f [Spanned<Field>]) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_hash_code {
                return;
            }

            let string_builder = &self.string_builder;

            quote_in! { *t =>
                @Override
                public int hashCode() {
                    int result = 1;
                    final $string_builder b = new $string_builder();
                    $(for f in fields join ($['\r']) {
                        result = result * 31 + $(f.hash_code(quote!(this.$(f.safe_ident()))));
                    })
                    return result;
                }
            }
        })
    }

    /// Build a equals function.
    fn equals<'f>(
        &'f self,
        name: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Java> + 'f {
        from_fn(move |t| {
            if !self.options.build_equals {
                return;
            }

            quote_in! { *t =>
                @Override
                public boolean equals(final Object other_) {
                    if (other_ == null) {
                        return false;
                    }

                    if (!(other_ instanceof $name)) {
                        return false;
                    }

                    @SuppressWarnings("unchecked")
                    final $name o_ = ($name)other_;

                    $(for f in fields join ($['\n']) {
                        if ($(f.not_equals(quote!(this.$(f.safe_ident())), quote!(o_.$(f.safe_ident()))))) {
                            return false;
                        }
                    })

                    return true;
                }
            }
        })
    }
}

/// Helper macro to implement listeners opt loop.
fn code(codes: &[Spanned<RpCode>]) -> impl FormatInto<Java> + '_ {
    from_fn(move |t| {
        for c in codes {
            if let RpContext::Java { imports, .. } = &c.context {
                for import in imports {
                    if let Some(split) = import.rfind('.') {
                        let (package, name) = import.split_at(split);
                        let name = &name[1..];
                        t.register(java::import(package, name));
                    }
                }

                quote_in! {*t =>
                    $(for line in &c.lines join ($['\r']) => $line)
                }
            }
        }
    })
}
