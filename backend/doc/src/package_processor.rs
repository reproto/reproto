//! Package processor.
//!
//! Build an overview of available packages.

use backend::Environment;
use backend::errors::*;
use core::{RpDecl, RpFile, RpVersionedPackage};
use doc_builder::DocBuilder;
use escape::Escape;
use macros::FormatAttribute;
use processor::Processor;

pub struct Data<'a> {
    pub package: &'a RpVersionedPackage,
    pub file: &'a RpFile,
}

macro_rules! types_section {
    ($slf:ident, $var:ident, $name:expr) => {
        if !$var.is_empty() {
            html!($slf, h2 {class => "kind"} ~ $name);

            html!($slf, table {} => {
                for v in $var {
                    html!($slf, tr {} => {
                        html!($slf, td {class => "package-item"} => {
                            $slf.full_name_without_package(&v.name)?;
                        });

                        html!($slf, td {class => "package-item-doc"} => {
                            $slf.doc(v.comment.iter().take(1))?;
                        });
                    });
                }
            });
        }
    }
}

define_processor!(PackageProcessor, Data<'env>, self,
    process => {
        use self::RpDecl::*;

        self.write_doc(|| {
            let mut types = Vec::new();
            let mut interfaces = Vec::new();
            let mut enums = Vec::new();
            let mut tuples = Vec::new();
            let mut services = Vec::new();

            for decl in self.body.file.for_each_decl() {
                match *decl.value() {
                    Type(ref ty) => types.push(ty),
                    Interface(ref interface) => interfaces.push(interface),
                    Enum(ref en) => enums.push(en),
                    Tuple(ref tuple) => tuples.push(tuple),
                    Service(ref service) => services.push(service),
                }
            }

            html!(self, section {class => "section-content"} => {
                html!(self, h1 {class => "section-title"} => {
                    html!(self, span {class => "kind"} ~ "package");
                    html!(self, span {class => "name-package"} ~
                          Escape(self.body.package.to_string().as_str()));
                });

                self.doc(self.body.file.comment.iter())?;

                types_section!(self, types, "Types");
                types_section!(self, interfaces, "Interfaces");
                types_section!(self, enums, "Enums");
                types_section!(self, tuples, "Tuples");
                types_section!(self, services, "Services");
            });

            Ok(())
        })
    };
);

impl<'env> PackageProcessor<'env> {}
