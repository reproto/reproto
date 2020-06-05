//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::escape::Escape;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use core::errors::Result;
use core::flavored::{RpEnumBody, RpVariantRef};

define_processor!(EnumProcessor, RpEnumBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-enum"} => {
                self.section_title("enum", &self.body.name)?;
                self.doc(&self.body.comment)?;
                self.variants(self.body.variants.iter())?;
                self.nested_decls(self.body.decls.iter())?;
            });

            Ok(())
        })
    };

    current_package => &self.body.name.package;
);

impl<'p> EnumProcessor<'p> {
    fn variants<'b, I>(&self, variants: I) -> Result<()>
    where
        I: IntoIterator<Item = RpVariantRef<'b>>,
    {
        let mut it = variants.into_iter().peekable();

        if it.peek().is_none() {
            return Ok(());
        }

        for variant in it {
            let id = variant.name.join("_");

            html!(self, h3 {id => id} => {
                html!(self, span {class => "kind"} ~ "variant");
                self.full_name_without_package(&variant.name)?;
                html!(self, span {class => "keyword"} ~ "as");

                match variant.value {
                    core::RpVariantValue::String(string) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(format!("\"{}\"", string).as_str()));
                    }
                    core::RpVariantValue::Number(number) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(number.to_string().as_str()));
                    }
                }
            });

            self.doc(variant.comment)?;
        }

        Ok(())
    }
}
