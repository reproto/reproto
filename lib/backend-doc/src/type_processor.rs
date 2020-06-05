//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use core::errors::Result;
use core::flavored::RpTypeBody;

define_processor!(TypeProcessor, RpTypeBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-type"} => {
                self.section_title("type", &self.body.name)?;

                html!(self, div {class => "section-body"} => {
                    self.doc(&self.body.comment)?;
                    self.fields(self.body.fields.iter())?;
                    self.nested_decls(self.body.decls.iter())?;
                });
            });

            Ok(())
        })
    };

    current_package => &self.body.name.package;
);

impl<'p> TypeProcessor<'p> {}
