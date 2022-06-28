//! C# backend for reproto

use crate::flavored::*;
use crate::processor::Processor;
use crate::Options;
use genco::prelude::*;
use genco::tokens::from_fn;
use naming::Naming as _;
use reproto_core::errors::Result;
use reproto_core::{Handle, RelativePathBuf, Spanned};
use std::rc::Rc;
use trans::Translated;

pub struct Compiler {
    opt: Options,
    env: Rc<Translated<CsharpFlavor>>,
    string_builder: csharp::Import,
    string: csharp::Import,
    object: csharp::Import,
    to_upper_snake: naming::ToUpperSnake,
}

impl Processor for Compiler {}

impl Compiler {
    pub(crate) fn new(env: Rc<Translated<CsharpFlavor>>, opt: Options) -> Compiler {
        Compiler {
            opt,
            env,
            string_builder: csharp::import("System.Text", "StringBuilder"),
            string: csharp::import("System", "String"),
            object: csharp::import("System", "Object"),
            to_upper_snake: naming::to_upper_snake(),
        }
    }

    pub(crate) fn compile(&self, handle: &dyn Handle) -> Result<()> {
        for decl in self.env.toplevel_decl_iter() {
            self.compile_decl(handle, decl)?;
        }

        Ok(())
    }

    fn compile_decl(&self, handle: &dyn Handle, decl: &RpDecl) -> Result<()> {
        use genco::fmt;

        let namespace = decl.name().package.join(".");

        let parts = namespace.split('.').collect::<Vec<_>>();

        let path = parts
            .iter()
            .cloned()
            .fold(RelativePathBuf::new(), |p, part| p.join(part));

        if !handle.is_dir(&path) {
            log::debug!("+dir: {}", path);
            handle.create_dir_all(&path)?;
        }

        let path = path.join(format!("{}.cs", decl.ident()));
        log::debug!("+class: {}", path);

        let file = quote! {
            #(ref t => self.process_decl(t, decl)?)
        };

        let config = csharp::Config::default().with_namespace(namespace);

        let fmt = fmt::Config::from_lang::<csharp::Csharp>();

        let mut f = handle.create(&path)?;
        let mut w = fmt::IoWriter::new(&mut f);

        file.format_file(&mut w.as_formatter(&fmt), &config)?;
        Ok(())
    }

    fn process_enum(&self, t: &mut csharp::Tokens, body: &RpEnumBody) -> Result<()> {
        let mut annotations = Vec::new();
        self.opt.gen.enum_type(&body.variants, &mut annotations);

        quote_in! { *t =>
            #(csharp::block_comment(&body.comment))
            #(for a in annotations join (#<push>) => #a)
            public enum #(&body.ident)#(self.enum_type(body.enum_type)) {
                #(for v in &body.variants join (,#<push>) {
                    #(self.variant(body.enum_type, v))
                })
            }
        }

        Ok(())
    }

    fn process_tuple(&self, t: &mut csharp::Tokens, body: &RpTupleBody) -> Result<()> {
        let mut annotations = Vec::new();
        let mut inner = Vec::new();
        self.opt
            .gen
            .tuple(&body.ident, &body.fields, &mut annotations, &mut inner);

        quote_in! { *t =>
            #(csharp::block_comment(&body.comment))
            #(for a in annotations join (#<push>) => #a)
            public class #(&body.ident) {
                #(for f in &body.fields join (#<line>) => #(self.field(f)))

                #(self.constructor(&body.ident, &body.fields))

                #(self.equals(&body.ident, &body.fields))

                #(self.get_hash_code(&body.fields))

                #(self.to_string(&body.ident, &body.fields))

                #(for i in inner join (#<line>) => #i)

                #(for d in &body.decls {
                    #(ref t => self.process_decl(t, d)?)
                })
            }
        }

        Ok(())
    }

    fn process_type(&self, t: &mut csharp::Tokens, body: &RpTypeBody) -> Result<()> {
        let mut annotations = Vec::new();
        self.opt.gen.class(&mut annotations);

        quote_in! { *t =>
            #(csharp::block_comment(&body.comment))
            #(for a in annotations join (#<push>) => #a)
            public class #(&body.ident) {
                #(for f in &body.fields join (#<line>) => #(self.field(f)))

                #(self.constructor(&body.ident, &body.fields))

                #(self.equals(&body.ident, &body.fields))

                #(self.get_hash_code(&body.fields))

                #(self.to_string(&body.ident, &body.fields))

                #(for d in &body.decls {
                    #(ref t => self.process_decl(t, d)?)
                })
            }
        }

        Ok(())
    }

    fn process_interface(&self, t: &mut csharp::Tokens, body: &RpInterfaceBody) -> Result<()> {
        let mut annotations = Vec::new();
        let mut tag_annotations = Vec::new();
        let mut inner = Vec::new();

        self.opt.gen.interface(
            &body.ident,
            &body.sub_type_strategy,
            &body.sub_types,
            &mut annotations,
            &mut tag_annotations,
            &mut inner,
        );

        quote_in! { *t =>
            #(csharp::block_comment(&body.comment))
            #(for a in annotations => #a)
            public abstract class #(&body.ident) {
                #(self.interface_sub_type_strategy(&body.ident, &body.sub_type_strategy, &tag_annotations))

                #(for i in inner join (#<line>) => #i)

                #(for sub_type in &body.sub_types {
                    #(self.sub_type(body, sub_type))
                })

                #(for d in &body.decls {
                    #(ref t => self.process_decl(t, d)?)
                })
            }
        }

        Ok(())
    }

    fn process_service(&self, _: &mut csharp::Tokens, _: &RpServiceBody) -> Result<()> {
        Ok(())
    }

    fn process_decl(&self, t: &mut csharp::Tokens, decl: &RpDecl) -> Result<()> {
        match decl {
            RpDecl::Interface(interface) => {
                self.process_interface(t, interface)?;
            }
            RpDecl::Type(ty) => {
                self.process_type(t, ty)?;
            }
            RpDecl::Tuple(ty) => {
                self.process_tuple(t, ty)?;
            }
            RpDecl::Enum(ty) => {
                self.process_enum(t, ty)?;
            }
            RpDecl::Service(ty) => {
                self.process_service(t, ty)?;
            }
        }

        Ok(())
    }

    /// Generated a tagged constructor.
    fn tagged_constructor<'f>(
        &'f self,
        ident: &'f str,
        tag: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Csharp> + 'f {
        let mut ann = Vec::new();
        self.opt.gen.class_constructor(&mut ann);

        let mut tag_ann = Vec::new();
        self.opt
            .gen
            .interface_tag_constructor_arg(tag, &mut tag_ann);

        quote_fn! {
            #(for a in ann join (#<push>) => #a)
            public #ident (
                #(for a in tag_ann => #a#<space>)#(&self.string) TypeField#(if !fields.is_empty() => ,)
                #(for f in fields join (,#<push>) => #(self.constructor_arg(f)))
            ) : base(TypeField) {
                #(for f in fields join (#<push>) {
                    this.#(&f.var) = #(&f.var);
                })
            }
        }
    }

    fn sub_type_constructor<'f>(
        &'f self,
        ident: &'f str,
        sub_type_strategy: &'f RpSubTypeStrategy,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Csharp> + 'f {
        quote_fn! {
            #(match sub_type_strategy {
                RpSubTypeStrategy::Tagged { tag } => {
                    #(self.tagged_constructor(ident, tag, fields))
                }
                RpSubTypeStrategy::Untagged => #(self.constructor(ident, fields)),
            })
        }
    }

    fn sub_type<'f>(
        &'f self,
        body: &'f RpInterfaceBody,
        sub_type: &'f RpSubType,
    ) -> impl FormatInto<Csharp> + 'f {
        let fields = body
            .fields
            .iter()
            .chain(&sub_type.fields)
            .cloned()
            .collect::<Vec<_>>();

        let mut annotations = Vec::new();
        self.opt.gen.class(&mut annotations);

        quote_fn! {
            #(csharp::block_comment(&sub_type.comment))
            #(for a in annotations join (#<push>) => #a)
            public class #(&sub_type.ident) : #(&body.ident) {
                #(for f in &fields join (#<line>) => #(self.field(f)))

                #(self.sub_type_constructor(&sub_type.ident, &body.sub_type_strategy, &fields))

                #(self.equals(&sub_type.ident, &fields))

                #(self.get_hash_code(&fields))

                #(self.to_string(&sub_type.ident, &fields))
            }
        }
    }

    fn interface_sub_type_strategy<'f>(
        &'f self,
        ident: &'f str,
        strategy: &'f RpSubTypeStrategy,
        ann: &'f [csharp::Tokens],
    ) -> impl FormatInto<Csharp> + 'f {
        from_fn(move |t| match strategy {
            RpSubTypeStrategy::Tagged { .. } => {
                quote_in! { *t =>
                    #(for a in ann join (#<push>) => #a)
                    private #(&self.string) TypeField {
                        get;
                    }

                    public #ident(#(&self.string) TypeField) {
                        this.TypeField = TypeField;
                    }
                }
            }
            RpSubTypeStrategy::Untagged => {}
        })
    }

    /// Format the enum type.
    fn enum_type(&self, enum_type: EnumType) -> impl FormatInto<Csharp> + '_ {
        quote_fn! {
            #(match enum_type {
                EnumType::Long => ( : long),
                EnumType::Int | EnumType::String => (),
            })
        }
    }

    /// Format a constructor argument.
    fn constructor_arg<'f>(&'f self, f: &'f Spanned<Field>) -> impl FormatInto<Csharp> + 'f {
        let mut ann = Vec::new();
        self.opt.gen.class_constructor_arg(f, &mut ann);

        quote_fn! {
            #(for a in ann join (#<push>) => #a) #(f.field_type()) #(&f.var)
        }
    }

    fn constructor<'f>(
        &'f self,
        ident: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Csharp> + 'f {
        let mut annotations = Vec::new();
        self.opt.gen.class_constructor(&mut annotations);

        quote_fn! {
            #(for a in annotations join (#<push>) => #a)
            public #ident (
                #(for f in fields join (,#<push>) => #(self.constructor_arg(f)))
            ) {
                #(for f in fields join (#<push>) {
                    this.#(&f.var) = #(&f.var);
                })
            }
        }
    }

    /// Process the variant value.
    fn variant_value<'f>(
        &'f self,
        enum_type: EnumType,
        variant: RpVariantRef<'f>,
    ) -> impl FormatInto<Csharp> + 'f {
        quote_fn! {
            #(match (enum_type, variant.value) {
                (EnumType::Long, RpVariantValue::Number(number)) => ( = #(display(number))L),
                (EnumType::Int, RpVariantValue::Number(number)) => ( = #(display(number))),
                _ => {},
            })
        }
    }

    /// Process the variant.
    fn variant<'f>(
        &'f self,
        enum_type: EnumType,
        variant: RpVariantRef<'f>,
    ) -> impl FormatInto<Csharp> + 'f {
        let mut annotations = Vec::new();
        let mut value = None;
        self.opt
            .gen
            .enum_variant(variant, &mut value, &mut annotations);

        let name = display(self.to_upper_snake.display(&**variant.ident));

        quote_fn! {
            #(csharp::block_comment(variant.comment))
            #(for a in annotations join (#<push>) => #a)
            #name#(self.variant_value(enum_type, variant))
        }
    }

    fn field<'f>(&'f self, f: &'f Spanned<Field>) -> impl FormatInto<Csharp> + 'f {
        let mut annotations = Vec::new();
        self.opt.gen.class_field(f, &mut annotations);

        quote_fn! {
            #(csharp::block_comment(&f.comment))
            #(for a in annotations join (#<push>) => #a)
            public #(f.field_type()) #(&f.var) {
                get;
            }
        }
    }

    /// Build a GetHashCode function.
    fn get_hash_code<'f>(&'f self, fields: &'f [Spanned<Field>]) -> impl FormatInto<Csharp> + 'f {
        quote_fn! {
            public override int GetHashCode()  {
                int result = 1;
                #(for f in fields join (#<push>) {
                    result = result * 31 + this.#(&f.var).GetHashCode();
                })
                return result;
            }
        }
    }

    /// Build an Equals function.
    fn equals<'f>(
        &'f self,
        ident: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Csharp> + 'f {
        quote_fn! {
            public override bool Equals(#(&self.object) other)  {
                #ident o = other as #ident;

                if (o == null) {
                    return false;
                }

                #(for f in fields join (#<line>) {
                    #(if f.ty.is_nullable() {
                        if (this.#(&f.var) == null) {
                            if (o.#(&f.var) != null) {
                                return false;
                            }
                        } else {
                            if (!this.#(&f.var).Equals(o.#(&f.var))) {
                                return false;
                            }
                        }
                    } else {
                        if (!this.#(&f.var).Equals(o.#(&f.var))) {
                            return false;
                        }
                    })
                })

                return true;
            }
        }
    }

    /// Build a ToString function.
    fn to_string<'f>(
        &'f self,
        name: &'f str,
        fields: &'f [Spanned<Field>],
    ) -> impl FormatInto<Csharp> + 'f {
        from_fn(move |t| {
            if fields.is_empty() {
                quote_in! { *t =>
                    public override String ToString() {
                        return #_(#name());
                    }
                }

                return;
            }

            let string_builder = &self.string_builder;

            quote_in! { *t =>
                public override String ToString() {
                    #string_builder b = new #string_builder();

                    b.Append(#_(#name#("(")));
                    #(for f in fields join (#<push>b.Append(", ");#<push>) {
                        b.Append(#_(#(&f.ident)=));
                        b.Append(this.#(&f.var));
                    })
                    b.Append(")");

                    return b.ToString();
                }
            }
        })
    }
}
