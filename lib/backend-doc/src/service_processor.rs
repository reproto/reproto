//! Processor for service declarations.

use backend::Environment;
use backend::errors::*;
use core::{Loc, RpEndpoint, RpServiceBody, WithPos};
use doc_builder::DocBuilder;
use escape::Escape;
use macros::FormatAttribute;
use processor::Processor;

define_processor!(ServiceProcessor, RpServiceBody, self,
    process => {
        self.write_doc(|| {
            let id = self.body.name.join("_");

            html!(self, section {id => &id, class => "section-content section-service"} => {
                self.section_title("service", &self.body.name)?;

                self.doc(&self.body.comment)?;

                for endpoint in self.body.endpoints.values() {
                    self.endpoint(endpoint)?;
                }

                self.nested_decls(self.body.decls.iter())?;
            });

            Ok(())
        })
    };

    current_package => &self.body.name.package;
);

impl<'p> ServiceProcessor<'p> {
    fn endpoint(&self, endpoint: &RpEndpoint) -> Result<()> {
        let id = format!(
            "{}_{}",
            self.body.name,
            endpoint.id_parts(Self::fragment_filter).join("_")
        );

        html!(self, h2 {class => "endpoint-title", id => id} => {
            self.name_until(&self.body.name)?;

            html!(self, span {class => "endpoint-id"} ~ Escape(endpoint.id.as_str()));
            html!(self, span {} ~ Escape("("));

            let mut it = endpoint.arguments.values().peekable();

            while let Some(&(ref name, ref channel)) = it.next() {
                html!(self, span {class => "endpoint-request-type"} => {
                    html!(self, span {class => "name"} ~ Escape(name.as_str()));
                    html!(self, span {class => "sep"} ~ Escape(":"));

                    if channel.is_streaming() {
                        html!(self, span {class => "keyword"} ~ Escape("stream"));
                    }

                    let (req, pos) = Loc::borrow_pair(channel);
                    self.write_type(req.ty()).with_pos(pos)?;
                });
            }

            html!(self, span {} ~ Escape(")"));

            if let Some(response) = endpoint.response.as_ref().take().as_ref() {
                html!(self, span {class => "keyword"} ~ "&rarr;");

                html!(self, span {class => "endpoint-response-type"} => {
                    if response.is_streaming() {
                        html!(self, span {class => "endpoint-stream"} ~ Escape("stream"));
                    }

                    let (res, pos) = Loc::borrow_pair(response);
                    self.write_type(res.ty()).with_pos(pos)?;
                });
            }

            if endpoint.id.as_str() != endpoint.name.as_str() {
                html!(self, span {class => "keyword"} ~ Escape("as"));
                html!(self, span {} ~ Escape(endpoint.name.as_str()));
            }
        });

        self.doc(&endpoint.comment)?;
        Ok(())
    }
}
