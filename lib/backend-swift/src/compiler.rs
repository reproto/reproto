//! Backend for Swift

use crate::flavored::{
    Field, Name, RpEnumBody, RpInterfaceBody, RpTupleBody, RpTypeBody, SwiftFlavor,
};
use crate::{Options, EXT};
use backend::PackageProcessor;
use core::errors::Result;
use core::{Handle, Spanned};
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr};
use trans::{self, Packages, Translated};

/// Documentation comments.
pub struct Comments<I>(pub I);

impl<I> FormatInto<Swift> for Comments<I>
where
    I: IntoIterator,
    I::Item: Into<ItemStr>,
{
    fn format_into(self, t: &mut swift::Tokens) {
        for line in self.0 {
            t.push();
            t.append(ItemStr::Static("//"));
            t.space();
            t.append(line.into());
        }
    }
}

pub struct Compiler<'a> {
    pub env: &'a Translated<SwiftFlavor>,
    opt: Options,
    handle: &'a dyn Handle,
}

impl<'a> Compiler<'a> {
    pub fn new(
        env: &'a Translated<SwiftFlavor>,
        opt: Options,
        handle: &'a dyn Handle,
    ) -> Result<Compiler<'a>> {
        Ok(Self { env, opt, handle })
    }

    /// Set up a model structure for the given fields.
    fn model_struct(
        &self,
        t: &mut swift::Tokens,
        name: &Name,
        comment: &[String],
        fields: &[Spanned<Field>],
        extends: bool,
    ) -> Result<()> {
        let extends = match (extends, &self.opt.struct_model_extends) {
            (false, _) => None,
            (true, extends) if extends.is_empty() => None,
            (true, extends) => Some(quote!(: #(for e in extends join (, ) => #e))),
        };

        let mut container = Vec::new();
        self.opt.gen.struct_model_added(&mut container, fields);

        quote_in! { *t =>
            #(Comments(comment))
            public struct #(&name.name)#extends {
                #(for field in fields join (#<push>) {
                    #(Comments(&field.comment))
                    let #(field.safe_ident()): #(field.field_type())
                })

                #(for c in container join (#<line>) => #c)
            }
        };

        Ok(())
    }

    /// Build a model struct for the given set of fields.
    fn model_type(
        &self,
        t: &mut swift::Tokens,
        name: &Name,
        comment: &[String],
        fields: &[Spanned<Field>],
    ) -> Result<()> {
        self.model_struct(t, name, comment, fields, true)?;

        let mut container = Vec::new();
        self.opt.gen.type_added(&mut container, name, fields);

        if !container.is_empty() {
            for c in container {
                t.line();
                t.append(c);
            }
        }

        Ok(())
    }

    pub fn compile(&self, packages: &Packages) -> Result<()> {
        use genco::fmt;

        let mut files = self.do_populate_files(|_, new, out| {
            if !new {
                out.line();
            }

            Ok(())
        })?;

        let mut f = Vec::new();
        self.opt.gen.package_added(&mut f);

        for (package, out) in f {
            files.insert(packages.package(package)?, out);
        }

        let handle = self.handle();

        for (package, mut out) in files {
            let full_path = self.setup_module_path(&package)?;

            log::debug!("+module: {}", full_path);

            out.line();

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let config = swift::Config::default();
            let fmt =
                fmt::Config::from_lang::<Swift>().with_indentation(fmt::Indentation::Space(2));

            out.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }
}

impl<'a> PackageProcessor<'a, SwiftFlavor, Name> for Compiler<'a> {
    type Out = swift::Tokens;
    type DeclIter = trans::translated::DeclIter<'a, SwiftFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'a dyn Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &Name) -> Result<()> {
        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &RpTypeBody) -> Result<()> {
        self.model_type(out, &body.name, &body.comment, &body.fields)?;

        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &RpTupleBody) -> Result<()> {
        let mut containers = Vec::new();
        self.opt
            .gen
            .tuple_added(&mut containers, &body.name, &body.fields);

        quote_in! { *out =>
            #(ref o => self.model_struct(
                o,
                &body.name,
                &body.comment,
                &body.fields,
                false,
            )?)

            #(for c in containers join (#<line>) => #c)
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &RpEnumBody) -> Result<()> {
        let mut containers = Vec::new();
        self.opt.gen.enum_added(&mut containers, &body.name, body);

        quote_in! { *out =>
            public enum #(&body.name.name) {
                #(for v in &body.variants join (#<push>) {
                    #(Comments(v.comment))
                    case #(v.ident())
                })
            }

            #(for c in containers join (#<line>) => #c)
        }

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &RpInterfaceBody) -> Result<()> {
        let mut inner = Vec::new();
        self.opt.gen.interface_model_added(&mut inner, body);

        let mut extra = Vec::new();
        self.opt
            .gen
            .interface_added(&mut extra, self, &body.name, body);

        quote_in! { *out =>
            #(Comments(&body.comment))
            public enum #(body.name.name.clone()) {
                #(for sub_type in &body.sub_types join (#<push>) {
                    case #(&sub_type.ident)(#(sub_type.name.name.clone()))
                })

                #(for c in inner join (#<line>) => #c)
            }

            #(for c in extra join (#<line>) => #c)

            #(ref o => for sub_type in &body.sub_types {
                let fields = body
                    .fields
                    .iter()
                    .chain(sub_type.fields.iter())
                    .cloned()
                    .collect::<Vec<_>>();

                o.line();
                self.model_type(o, &sub_type.name, &sub_type.comment, &fields)?;
            })
        };

        Ok(())
    }
}
