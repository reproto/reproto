//! Module that adds fasterxml annotations to generated classes.

use crate::codegen::{ServiceAdded, ServiceCodegen};
use crate::flavored::*;
use crate::utils::BlockComment;
use crate::Options;
use backend::Initializer;
use genco::prelude::*;
use reproto_core::errors::Result;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {}

pub(crate) struct Module {
    #[allow(dead_code)]
    config: Config,
}

impl Module {
    pub(crate) fn new(config: Config) -> Module {
        Module { config }
    }
}

struct RequestsServiceCodegen {
    requests: python::ImportModule,
}

impl RequestsServiceCodegen {
    pub fn new() -> RequestsServiceCodegen {
        Self {
            requests: python::import_module("requests"),
        }
    }
}

impl ServiceCodegen for RequestsServiceCodegen {
    fn generate(
        &self,
        ServiceAdded {
            body, type_body, ..
        }: ServiceAdded,
    ) -> Result<()> {
        quote_in! { *type_body =>
            class $(&body.name)_Requests:
                def __init__(self, **kw):
                    url = kw.pop("url", None)

                    $(if let Some(ref url) = body.http.url {
                        if url is None:
                            url = $(quoted(url.as_str()))
                    } else {
                        if url is None:
                            raise Exception("Missing 'url' argument")
                    })

                    session = kw.pop("session", None)

                    if session is None:
                        session = $(&self.requests)

                    self.url = url
                    self.session = session

                $(for e in &body.endpoints join ($['\n']) {
                    $(ref t =>
                        if !e.has_http_support() {
                            continue;
                        }

                        let method = e
                            .http
                            .method
                            .as_ref()
                            .unwrap_or(&RpHttpMethod::Get)
                            .as_str();

                        quote_in! { *t =>
                            def $(e.safe_ident())(self, $(for a in &e.arguments => $(a.safe_ident()))):
                                $(BlockComment(&e.comment))
                                $(if let Some(ref http_path) = e.http.path {
                                    path = list()

                                    path.append(self.url)
                                    $(for step in &http_path.steps join ($['\r']) =>
                                        path.append("/")
                                        $(for part in &step.parts join ($['\r']) {
                                            path.append($(match part {
                                                RpPathPart::Variable(a) => str($(a.safe_ident())),
                                                RpPathPart::Segment(s) => $(quoted(s.to_string())),
                                            }))
                                        })
                                    )

                                    url = "/".join(path)
                                } else {
                                    url = self.url
                                })

                                r = self.session.request($method, url=url$(if let Some(ref body) = e.http.body {
                                    , json=$(body.safe_ident()).encode()
                                }))

                                r.raise_for_status()

                                $(if let Some(res) = &e.response =>
                                    $(match e.http.accept {
                                        RpAccept::Json => {
                                            data = r.json();

                                            $(if let Some(d) = res.ty().decode("data", 0) {
                                                $d
                                            })

                                            return data
                                        }
                                        RpAccept::Text => {
                                            return r.text
                                        }
                                    })
                                )
                        }
                    )
                })
        }

        Ok(())
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Options) -> Result<()> {
        options
            .service_generators
            .push(Box::new(RequestsServiceCodegen::new()));

        Ok(())
    }
}
