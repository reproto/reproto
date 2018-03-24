//! Processor for service declarations.

use core::errors::*;
use core::flavored::RpTupleBody;
use doc_builder::DocBuilder;
use macros::FormatAttribute;
use processor::Processor;
use trans::Environment;

define_processor!(TupleProcessor, RpTupleBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-tuple"} => {
                self.section_title("tuple", &self.body.name)?;

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

impl<'p> TupleProcessor<'p> {}
