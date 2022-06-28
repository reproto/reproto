//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::escape::Escape;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use reproto_core::errors::Result;
use reproto_core::flavored::*;

pub struct Data<'a> {
    pub entries: Vec<(&'a RpVersionedPackage, &'a RpFile)>,
}

define_processor!(IndexProcessor, Data<'session>, self,
    process => {
        self.write_doc(|| {
            html!(self, section {class => "section-content"} => {
                html!(self, h1 {class => "section-title"} ~ "Index");

                html!(self, h2 {class => "kind"} ~ "Packages");

                html!(self, table {} => {
                    for (package, file) in self.body.entries.iter().cloned() {
                        html!(self, tr {} => {
                            html!(self, td {class => "package-item"} => {
                                let package_url = self.package_url(package);
                                html!(self, a {class => "name-package", href => package_url} ~
                                        Escape(package.to_string().as_str()));
                            });

                            html!(self, td {class => "package-item-doc"} => {
                                self.doc(file.comment.iter().take(1))?;
                            });
                        });
                    }
                });
            });

            Ok(())
        })
    };
);

impl<'session> IndexProcessor<'session> {}
