//! Module that adds fasterxml annotations to generated classes.

use backend::Initializer;
use codegen::{ServiceAdded, ServiceCodegen};
use core;
use core::errors::Result;
use genco::python::imported;
use genco::{Python, Quoted, Tokens};
use utils::{BlockComment, IfNoneRaise, IfNoneThen};
use Options;

#[derive(Debug, Deserialize)]
pub enum Version {
    #[serde(rename = "1")]
    Version1,
    #[serde(rename = "2")]
    Version2,
}

impl Default for Version {
    fn default() -> Self {
        Version::Version1
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    version: Version,
}

pub struct Module {
    #[allow(dead_code)]
    config: Config,
}

impl Module {
    pub fn new(config: Config) -> Module {
        Module { config: config }
    }
}

struct RequestsServiceCodegen {
    requests: Python<'static>,
}

impl RequestsServiceCodegen {
    pub fn new() -> RequestsServiceCodegen {
        Self {
            requests: imported("requests"),
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
        type_body.push(toks!["class ", &body.name, "_Requests:"]);
        type_body.nested({
            let mut t = Tokens::new();

            t.push({
                let mut args = Tokens::new();
                args.append("self");
                args.append("**kw");

                // Use default URL if available.
                if let Some(ref url) = body.http.url {
                    args.append(toks!["url=", url.as_str().quoted()]);
                } else {
                    args.append("url");
                }

                args.append(toks!["session=", self.requests.clone()]);

                let mut t = Tokens::new();

                t.push(toks!["def __init__(self, **kw):"]);

                t.nested({
                    let mut t = Tokens::new();
                    t.push(toks!["url = kw.pop(", "url".quoted(), ", None)"]);

                    if let Some(ref url) = body.http.url {
                        t.push(IfNoneThen("url", url.as_str().quoted()));
                    } else {
                        t.push(IfNoneRaise("url", "Missing 'url' argument"));
                    }

                    t.push(toks!["session = kw.pop(", "session".quoted(), ", None)"]);
                    t.push(IfNoneThen("session", self.requests.clone()));

                    t.push({
                        let mut t = Tokens::new();
                        t.push("self.url = url");
                        t.push("self.session = session");
                        t
                    });

                    t.join_line_spacing()
                });

                t
            });

            for e in &body.endpoints {
                if !e.has_http_support() {
                    continue;
                }

                t.push({
                    let mut t = Tokens::new();

                    let mut path = Tokens::new();

                    if let Some(ref http_path) = e.http.path {
                        for step in &http_path.steps {
                            path.push(toks!["path.append(\"/\")"]);

                            for part in &step.parts {
                                let var = match *part {
                                    core::RpPathPart::Variable(ref arg) => {
                                        toks!["str(", arg.safe_ident(), ")"]
                                    }
                                    core::RpPathPart::Segment(ref s) => {
                                        toks![s.to_string().quoted()]
                                    }
                                };

                                path.push(toks!["path.append(", var, ")"]);
                            }
                        }
                    }

                    let path = {
                        if path.is_empty() {
                            path
                        } else {
                            let mut full = Tokens::new();
                            full.push("path = list()");
                            full.push("path.append(self.url)");
                            full.extend(path);
                            full
                        }
                    };

                    let method = e.http
                        .method
                        .as_ref()
                        .unwrap_or(&core::RpHttpMethod::Get)
                        .as_str();

                    let mut args = Tokens::new();
                    args.append("self");
                    args.extend(e.arguments.iter().map(|a| a.safe_ident().into()));

                    t.push(toks!["def ", e.safe_ident(), "(", args.join(", "), "):"]);
                    t.nested(BlockComment(&e.comment));

                    t.nested({
                        let mut t = Tokens::new();

                        let mut args = Tokens::new();

                        args.append(method.quoted());

                        if path.is_empty() {
                            args.append("self.url");
                        } else {
                            t.push(path);
                            t.push("url = \"\".join(path)");
                            args.append("url");
                        };

                        if let Some(ref body) = e.http.body {
                            args.append(toks!["json=", body.safe_ident(), ".encode()"]);
                        }

                        t.push(toks!["r = self.session.request(", args.join(", "), ")"]);
                        t.push(toks!["r.raise_for_status()"]);

                        if let Some(res) = e.response.as_ref() {
                            match e.http.accept {
                                core::RpAccept::Json => {
                                    push!(t, "data = r.json()");

                                    if let Some(d) = res.ty().decode("data", 0) {
                                        t.push(d);
                                    }

                                    push!(t, "return data");
                                }
                                core::RpAccept::Text => {
                                    t.push("return r.text");
                                }
                            }
                        }

                        t.join_line_spacing()
                    });

                    t
                });
            }

            t.join_line_spacing()
        });

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
