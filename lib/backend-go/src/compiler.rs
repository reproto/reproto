//! Backend for Go

use crate::flavored::*;
use crate::{EnumAdded, FieldAdded, FileSpec, InterfaceAdded, Options, Tags, TupleAdded, EXT};
use backend::PackageProcessor;
use core::errors::Result;
use core::{Handle, RelativePathBuf, Spanned};
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr};
use trans::{self, Translated};

/// Documentation comments.
pub struct Comments<I>(pub I);

impl<I> FormatInto<Go> for Comments<I>
where
    I: IntoIterator,
    I::Item: Into<ItemStr>,
{
    fn format_into(self, t: &mut Tokens<Go>) {
        for line in self.0.into_iter() {
            t.push();
            t.append(ItemStr::Static("//"));
            t.space();
            t.append(line.into());
        }
    }
}

pub(crate) struct Compiler<'a> {
    pub(crate) env: &'a Translated<GoFlavor>,
    options: Options,
    handle: &'a dyn Handle,
}

impl<'a> Compiler<'a> {
    pub(crate) fn new(
        env: &'a Translated<GoFlavor>,
        options: Options,
        handle: &'a dyn Handle,
    ) -> Self {
        Self {
            env,
            options,
            handle,
        }
    }

    fn process_struct(
        &self,
        t: &mut Tokens<Go>,
        name: &GoName,
        comment: &[String],
        fields: &[Spanned<RpField>],
    ) -> Result<()> {
        quote_in! { *t =>
            #(Comments(comment))
            type #name struct {
                #(for f in fields.into_iter() join (#<push>) {
                    #(ref t => {
                        let mut tags = Tags::new();

                        for g in &self.options.field_gens {
                            g.generate(FieldAdded {
                                tags: &mut tags,
                                field: f,
                            })?;
                        }

                        quote_in! { *t =>
                            #(Comments(&f.comment))
                            #(f.safe_ident()) #(if f.is_optional() {
                                *#(&f.ty)
                            } else {
                                #(&f.ty)
                            }) #(tags)
                        }
                    })
                })
            }
        };

        Ok(())
    }

    pub fn compile(&self) -> Result<()> {
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

            let mut w = fmt::IoWriter::new(handle.create(&full_path)?);
            let config = go::Config::default().with_package(package.join("_"));
            let fmt = fmt::Config::from_lang::<Go>().with_indentation(fmt::Indentation::Space(2));

            out.0.format_file(&mut w.as_formatter(&fmt), &config)?;
        }

        Ok(())
    }
}

impl<'el> PackageProcessor<'el, GoFlavor> for Compiler<'el> {
    type Out = FileSpec;
    type DeclIter = trans::translated::DeclIter<'el, GoFlavor>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &dyn Handle {
        self.handle
    }

    fn default_process(&self, _out: &mut Self::Out, _: &GoName) -> Result<()> {
        Ok(())
    }

    fn resolve_full_path(&self, package: &RpPackage) -> Result<RelativePathBuf> {
        let mut full_path = RelativePathBuf::from(package.join("_")).join("lib");
        full_path.set_extension(self.ext());
        Ok(full_path)
    }

    fn process_type(&self, out: &mut Self::Out, body: &RpTypeBody) -> Result<()> {
        self.process_struct(&mut out.0, &body.name, &body.comment, &body.fields)?;

        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &RpTupleBody) -> Result<()> {
        quote_in! { out.0 =>
            #(Comments(&body.comment))
            type #(&body.name) struct {
                #(for f in &body.fields {
                    #(Comments(&f.comment))
                    #(f.safe_ident()) #(if f.is_optional() {
                        *#(&f.ty)
                    } else {
                        #(&f.ty)
                    })
                })
            }

            #(for g in &self.options.tuple_gens join (#<line>) {
                #(ref container => g.generate(TupleAdded {
                    container,
                    name: &body.name,
                    body,
                })?)
            })
        }

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &RpEnumBody) -> Result<()> {
        quote_in! { out.0 =>
            #(Comments(&body.comment))
            type #(&body.name) int

            const (
                #(ref t => {
                    let mut it = body.variants.iter();

                    quote_in! { *t =>
                        #(if let Some(v) = it.next() {
                            #(&body.name)_#(v.ident.as_str()) #(&body.name) = iota
                        })
                        #(for v in it join (#<push>) {
                            #(&body.name)_#(v.ident.as_str())
                        })
                    }
                })
            )

            #(for g in &self.options.enum_gens join (#<line>) {
                #(ref container => g.generate(EnumAdded {
                    container,
                    name: &body.name,
                    body,
                })?)
            })
        }

        Ok(())
    }

    fn process_interface(&self, out: &mut Self::Out, body: &RpInterfaceBody) -> Result<()> {
        quote_in! { out.0 =>
            #(Comments(&body.comment))
            type #(&body.name) struct {
                Value interface {
                    Is#(&body.name)()
                }
            }

            #(for sub_type in &body.sub_types join (#<line>) {
                #(ref t {
                    let fields = body.fields
                        .iter()
                        .chain(sub_type.fields.iter())
                        .cloned()
                        .collect::<Vec<_>>();

                    self.process_struct(t, &sub_type.name, &sub_type.comment, &fields)?;
                })

                func (this #(&sub_type.name)) Is#(&body.name)() {
                }
            })

            #(for g in &self.options.interface_gens join (#<line>) {
                #(ref container => g.generate(InterfaceAdded {
                    container,
                    name: &body.name,
                    body,
                })?)
            })
        }

        Ok(())
    }
}
