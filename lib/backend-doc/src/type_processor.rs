//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use reproto_core::errors::Result;
use reproto_core::flavored::*;

define_processor!(TypeProcessor, RpTypeBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-type"} => {
                self.section_title("type", &self.body.name)?;
                self.doc(&self.body.comment)?;

                html!(self, div {class => "section-body"} => {
                    self.fields_overview(&self.body.fields)?;
                    self.nested_decls_overview(&self.body.decls)?;

                    html!(self, h2 {} ~ "Fields");
                    self.fields(&self.body.fields)?;

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

impl<'p> TypeProcessor<'p> {}
