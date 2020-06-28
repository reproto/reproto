//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use core::errors::Result;
use core::flavored::{RpInterfaceBody, RpSubType};

define_processor!(InterfaceProcessor, RpInterfaceBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-interface"} => {
                self.section_title("interface", &self.body.name)?;
                self.doc(&self.body.comment)?;

                self.fields_overview(&self.body.fields)?;

                for sub_type in &self.body.sub_types {
                    self.sub_type_overview(sub_type)?;
                }

                self.nested_decls_overview(&self.body.decls)?;

                if !self.body.fields.is_empty() {
                    html!(self, h2 {} ~ "Fields");
                    self.fields(&self.body.fields)?;
                }

                if !self.body.sub_types.is_empty() {
                    html!(self, h2 {} ~ "Sub Types");
                    for sub_type in &self.body.sub_types {
                        self.sub_type(sub_type)?;
                    }
                }

                if !self.body.decls.is_empty() {
                    html!(self, h2 {} ~ "Nested");
                    self.nested_decls(&self.body.decls)?;
                }
            });

            Ok(())
        })
    };

    current_package => &self.body.name.package;
);

impl<'p> InterfaceProcessor<'p> {
    fn sub_type(&self, sub_type: &RpSubType) -> Result<()> {
        let id = sub_type.name.join("_");

        html!(self, h2 {id => id, class => "sub-type-title", id => format!("subtype.{}", sub_type.ident)} => {
            html!(self, span {class => "kind"} ~ "subtype");
            self.full_name_without_package(&sub_type.name)?;
        });

        self.doc(&self.body.comment)?;

        let fields = self.body.fields.iter().chain(sub_type.fields.iter());
        self.fields(fields)?;
        self.nested_decls(sub_type.decls.iter())?;
        Ok(())
    }

    fn sub_type_overview(&self, sub_type: &RpSubType) -> Result<()> {
        let id = sub_type.name.join("_");

        html!(self, h2 {id => id, class => "sub-type-title"} => {
            html!(self, span {class => "kind"} ~ "subtype");

            html!(self, a {href => format!("#subtype.{}", sub_type.ident)} => {
                self.full_name_without_package(&sub_type.name)?;
            });
        });

        self.doc(self.body.comment.iter().take(1))?;
        Ok(())
    }
}
