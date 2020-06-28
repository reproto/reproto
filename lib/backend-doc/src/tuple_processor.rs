//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use core::errors::Result;
use core::flavored::RpTupleBody;

define_processor!(TupleProcessor, RpTupleBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-tuple"} => {
                self.section_title("tuple", &self.body.name)?;
                self.doc(&self.body.comment)?;

                html!(self, div {class => "section-body"} => {
                    self.fields_overview(&self.body.fields)?;
                    self.nested_decls_overview(&self.body.decls)?;

                    if !self.body.fields.is_empty() {
                        html!(self, h2 {} ~ "Fields");
                        self.fields(&self.body.fields)?;
                    }

                    if !self.body.decls.is_empty() {
                        html!(self, h2 {} ~ "Nested");
                        self.nested_decls(&self.body.decls)?;
                    }
                });
            });

            Ok(())
        })
    };

    current_package => &self.body.name.package;
);

impl<'p> TupleProcessor<'p> {}
