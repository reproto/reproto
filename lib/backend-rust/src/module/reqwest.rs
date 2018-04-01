//! gRPC module for Rust.

use backend::{Initializer, PackageUtils};
use core::errors::Result;
use core::{self, Loc};
use flavored::{RpEndpointHttp1, RpPackage, RpVersionedPackage, RustEndpoint};
use genco::rust::{imported, local};
use genco::{Cons, IntoTokens, Quoted, Rust, Tokens};
use std::rc::Rc;
use utils::Comments;
use {Options, Root, RootCodegen, RustFileSpec, Service, ServiceCodegen, SCOPE_SEP};

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Options) -> Result<()> {
        let package_utils = options.package_utils.clone();

        let utils_package = RpPackage::new(vec!["reproto".to_string()]);
        let utils_package = RpVersionedPackage::new(utils_package, None);

        let utils_pkg = Rc::new(package_utils.package(&utils_package).join(SCOPE_SEP));
        let result = imported(utils_pkg.clone(), "Result");
        let path_encode = imported(utils_pkg.clone(), "PathEncode");

        options
            .service
            .push(Box::new(ReqwestService::new(result, path_encode)));

        options
            .root
            .push(Box::new(ReqwestUtils::new(utils_package)));

        Ok(())
    }
}

struct ReqwestUtils {
    utils_package: RpVersionedPackage,
}

impl ReqwestUtils {
    pub fn new(utils_package: RpVersionedPackage) -> Self {
        Self { utils_package }
    }

    fn reproto<'el>(&self) -> Result<RustFileSpec<'el>> {
        let mut f = RustFileSpec::default();

        let mut errors = Vec::new();
        errors.push((imported("reqwest", "Error"), "ReqwestError"));
        errors.push((imported("reqwest", "UrlError"), "UrlError"));
        errors.push((imported("std::fmt", "Error"), "FormatError"));

        f.0.push({
            let mut t = Tokens::new();

            push!(t, "pub enum Error {");

            for &(ref ty, ref variant) in &errors {
                nested!(t, variant.clone(), "(", ty.clone(), "),");
            }

            push!(t, "}");

            t
        });

        f.0.push({
            let mut t = Tokens::new();
            let result = imported("std::result", "Result");

            push!(t, "pub type Result<T> = ", result, "<T, Error>;");

            t
        });

        for &(ref ty, ref variant) in &errors {
            f.0.push({
                let mut t = Tokens::new();

                push!(t, "impl From<", ty.clone(), "> for Error {");

                t.nested({
                    let mut t = Tokens::new();

                    push!(t, "fn from(value: ", ty.clone(), ") -> Self {");
                    nested!(t, "Error::", variant.clone(), "(value)");
                    push!(t, "}");

                    t
                });

                push!(t, "}");

                t
            });
        }

        f.0.push({
            let mut t = Tokens::new();

            let display = imported("std::fmt", "Display");
            let fmt = imported("std::fmt", "Formatter");
            let result = imported("std::fmt", "Result");
            let encode = imported("reqwest::header::parsing", "http_percent_encode");

            push!(t, "pub struct PathEncode<T>(pub T);");

            t.push({
                let mut t = Tokens::new();

                push!(t, "impl<T> ", display, " for PathEncode<T>");
                push!(t, "where");
                nested!(t, "T: ", display);
                push!(t, "{");

                t.nested({
                    let mut t = Tokens::new();

                    push!(t, "fn fmt(&self, fmt: &mut ", fmt, ") -> ", result, " {");
                    nested!(t, encode, "(fmt, self.0.to_string().as_bytes())");
                    push!(t, "}");

                    t
                });

                push!(t, "}");

                t
            });

            t.join_line_spacing()
        });

        Ok(f)
    }
}

impl RootCodegen for ReqwestUtils {
    fn generate(&self, root: Root) -> Result<()> {
        let Root { files, .. } = root;
        let package = self.utils_package.clone();
        files.insert(package, self.reproto()?);
        Ok(())
    }
}

struct ReqwestService {
    result: Rust<'static>,
    path_encode: Rust<'static>,
    client: Rust<'static>,
}

impl ReqwestService {
    pub fn new(result: Rust<'static>, path_encode: Rust<'static>) -> Self {
        Self {
            result,
            path_encode,
            client: imported("reqwest", "Client"),
        }
    }
}

impl ServiceCodegen for ReqwestService {
    fn generate(&self, service: Service) -> Result<()> {
        let Service {
            body,
            container,
            name,
            attributes,
            ..
        } = service;

        let name = Cons::from(format!("{}_Reqwest", name));
        let url_ty = imported("reqwest", "Url");

        container.push({
            let mut t = Tokens::new();

            t.push_unless_empty(attributes.clone());
            push!(t, "pub struct ", name, "{");
            nested!(t, "client: ", self.client, ",");
            nested!(t, "url: ", url_ty, ",");
            push!(t, "}");

            t
        });

        container.push({
            let mut t = Tokens::new();

            push!(t, "impl ", name, " {");

            t.push_unless_empty({
                let mut t = Tokens::new();

                // constructor.
                t.nested({
                    let option = local("Option");

                    let mut t = Tokens::new();

                    let mut args = Tokens::new();
                    args.append(toks!["client: ", self.client.clone()]);

                    match body.http.url {
                        Some(_) => {
                            args.append(toks![
                                "url: ",
                                option.with_arguments(vec![url_ty.clone()]),
                            ]);
                        }
                        None => {
                            args.append(toks!["url: ", url_ty.clone()]);
                        }
                    };

                    let s = self.result.with_arguments(vec![local("Self")]);

                    push!(t, "pub fn new(", args.join(", "), ") -> ", s, " {");

                    t.nested({
                        let mut t = Tokens::new();

                        t.push_into(|t| match body.http.url {
                            Some(ref url) => {
                                let url = Loc::value(url).clone().quoted();

                                push!(t, "let url = match url {");
                                nested!(t, "Some(url) => url,");
                                nested!(t, "None => ", url_ty, "::parse(", url, ")?,");
                                push!(t, "};");
                            }
                            None => {
                                push!(t, "let url = Some(url);");
                            }
                        });

                        t.push_into(|t| {
                            push!(t, "Ok(Self {");
                            nested!(t, "client,");
                            nested!(t, "url,");
                            push!(t, "})");
                        });

                        t.join_line_spacing()
                    });

                    push!(t, "}");

                    t
                });

                // endpoint methods.
                for e in &body.endpoints {
                    let http = match e.http1.as_ref() {
                        Some(http) => http,
                        None => continue,
                    };

                    t.nested({
                        let mut t = Tokens::new();

                        t.push_unless_empty(Comments(&e.comment));
                        t.push(Endpoint {
                            result: &self.result,
                            path_encode: &self.path_encode,
                            e,
                            http,
                        });

                        t
                    });
                }

                t.join_line_spacing()
            });

            push!(t, "}");

            t
        });

        Ok(())
    }
}

struct Endpoint<'a, 'el: 'a> {
    result: &'a Rust<'static>,
    path_encode: &'a Rust<'static>,
    e: &'el RustEndpoint,
    http: &'el RpEndpointHttp1,
}

impl<'a, 'el: 'a> IntoTokens<'el, Rust<'el>> for Endpoint<'a, 'el> {
    fn into_tokens(self) -> Tokens<'el, Rust<'el>> {
        let Endpoint {
            result,
            path_encode,
            e,
            http,
        } = self;

        let mut t = Tokens::new();

        // import trait
        t.register(imported("std::fmt", "Write").qualified());

        let mut args = Tokens::new();

        args.append("&self");

        for a in &e.arguments {
            args.append({
                let mut t = Tokens::new();

                t.append(a.safe_ident());
                t.append(": ");
                t.append(a.channel.ty());

                t
            });
        }

        let args = args.join(", ");

        let res = if let Some(ref res) = http.response {
            toks![result.clone(), "<", res, ">"]
        } else {
            toks![result.clone(), "<()>"]
        };

        push!(t, "pub fn ", e.safe_ident(), "(", args, ") -> ", res, " {");

        t.nested({
            let mut t = Tokens::new();

            let method = match http.method {
                core::RpHttpMethod::Get => "Get",
                core::RpHttpMethod::Post => "Post",
                core::RpHttpMethod::Put => "Put",
                core::RpHttpMethod::Update => "Update",
                core::RpHttpMethod::Delete => "Delete",
                core::RpHttpMethod::Patch => "Patch",
                core::RpHttpMethod::Head => "Head",
            };

            let m = toks![imported("reqwest", "Method"), "::", method];

            if let Some(ref http_path) = e.http.path {
                let mut p = Tokens::new();

                push!(p, "let mut path_ = String::new();");

                for step in &http_path.steps {
                    push!(p, "path_.push_str(", "/".quoted(), ");");

                    for part in &step.parts {
                        match *part {
                            core::RpPathPart::Variable(ref arg) => {
                                let var = toks![path_encode.clone(), "(", arg.safe_ident(), ")"];
                                push!(p, "write!(path_, ", "{}".quoted(), ", ", var, ")?;");
                            }
                            core::RpPathPart::Segment(ref s) => {
                                push!(p, "path_.push_str(", s.as_str().quoted(), ");");
                            }
                        }
                    }
                }

                t.push(p);
                push!(t, "let url_ = self.url.join(&path_)?;");
            } else {
                push!(t, "let url_ = self.url.clone();");
            }

            let req = toks!["self.client.request(", m, ", url_)"];

            push!(t, "let mut req_ = ", req, ";");

            if let Some(ref req) = e.request {
                push!(t, "req_.json(&", req.safe_ident(), ");");
            }

            if e.response.is_some() {
                push!(t, "let mut res_ = req_.send()?;");
                push!(t, "let body_ = res_.json()?;");
                push!(t, "Ok(body_)");
            } else {
                push!(t, "req_.send()?;");
                push!(t, "Ok(())");
            }

            t.join_line_spacing()
        });

        push!(t, "}");

        t
    }
}