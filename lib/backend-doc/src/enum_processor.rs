//! Processor for service declarations.

use crate::doc_builder::DocBuilder;
use crate::escape::Escape;
use crate::macros::FormatAttribute;
use crate::processor::Processor;
use reproto_core::errors::Result;
use reproto_core::flavored::*;

define_processor!(EnumProcessor, RpEnumBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-enum"} => {
                self.section_title("enum", &self.body.name)?;
                self.doc(&self.body.comment)?;

                self.variants_overview(&self.body.variants)?;
                self.nested_decls_overview(&self.body.decls)?;

                if !self.body.variants.is_empty() {
                    html!(self, h2 {} ~ "Variants");
                    self.variants(&self.body.variants)?;
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

            html!(self, h3 {id => format!("variant.{}", id)} => {
                html!(self, span {class => "kind"} ~ "variant");
                self.full_name_without_package(&variant.name)?;
                html!(self, span {class => "keyword"} ~ "as");

                match variant.value {
                    RpVariantValue::String(string) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(format!("\"{}\"", string).as_str()));
                    }
                    RpVariantValue::Number(number) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(number.to_string().as_str()));
                    }
                }
            });

            self.doc(variant.comment)?;
        }

        Ok(())
    }

    fn variants_overview<'b, I>(&self, variants: I) -> Result<()>
    where
        I: IntoIterator<Item = RpVariantRef<'b>>,
    {
        let mut it = variants.into_iter().peekable();

        if it.peek().is_none() {
            return Ok(());
        }

        for variant in it {
            let id = variant.name.join("_");

            html!(self, h3 {} => {
                html!(self, span {class => "kind"} ~ "variant");

                html!(self, a {href => format!("#variant.{}", id)} => {
                    self.full_name_without_package(&variant.name)?;
                });

                html!(self, span {class => "keyword"} ~ "as");

                match variant.value {
                    RpVariantValue::String(string) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(format!("\"{}\"", string).as_str()));
                    }
                    RpVariantValue::Number(number) => {
                        html!(self, span {class => "variant-ordinal"} ~
                            Escape(number.to_string().as_str()));
                    }
                }
            });
        }

        Ok(())
    }
}
