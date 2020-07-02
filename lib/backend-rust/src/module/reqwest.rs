//! gRPC module for Rust.

use crate::flavored::{RpEndpointHttp1, RpPackage, RpPathSpec, RpServiceBody, RustEndpoint, Type};
use crate::utils::Comments;
use crate::{Options, Root, RootCodegen, Service, ServiceCodegen, SCOPE_SEP};
use core::errors::Result;
use genco::prelude::*;
use genco::tokens::{FormatInto, ItemStr, Tokens};
use std::rc::Rc;

pub(crate) fn initialize(options: &mut Options) -> Result<()> {
    let utils_package = options.packages.new("reproto")?;

    let imported_utils_package = Rc::new(format!("crate::{}", utils_package.join(SCOPE_SEP)));
    let result = Type::from(rust::import(imported_utils_package.clone(), "Result"));
    let path_encode = Type::from(rust::import(imported_utils_package.clone(), "PathEncode"));

    options
        .service
        .push(Box::new(ReqwestService::new(result, path_encode)));

    options
        .root
        .push(Box::new(ReqwestUtils::new(utils_package)));

    Ok(())
}

struct ReqwestUtils {
    utils_package: RpPackage,
}

impl ReqwestUtils {
    pub fn new(utils_package: RpPackage) -> Self {
        Self { utils_package }
    }

    fn reproto(&self) -> Result<rust::Tokens> {
        let mut t = rust::Tokens::new();

        let errors = vec![
            (rust::import("reqwest", "Error"), "ReqwestError"),
            (rust::import("url", "ParseError"), "UrlParseError"),
            (rust::import("std::fmt", "Error"), "FormatError"),
        ];

        // basic impl and conversions.
        {
            let result = rust::import("std::result", "Result");

            quote_in! { t =>
                #[derive(Debug)]
                pub enum Error {
                    #(for (ty, v) in errors.iter().cloned() join (,#<push>) => #v(#ty))
                }

                pub type Result<T, E = Error> = #result<T, E>;

                #(for (ref ty, variant) in errors.iter().cloned() join (#<line>) =>
                    impl From<#ty> for Error {
                        fn from(value: #ty) -> Self {
                            Error::#variant(value)
                        }
                    }
                )
            };
        }

        // fmt::Display implementation for Error
        {
            let display = rust::import("std::fmt", "Display");
            let formatter = rust::import("std::fmt", "Formatter");
            let result = rust::import("std::fmt", "Result");

            t.line();

            quote_in! { t =>
                impl #display for Error {
                    fn fmt(&self, fmt: &mut #formatter) -> #result {
                        match self {
                            #(for (_, variant) in &errors =>
                                #<push>Error::#(*variant)(e) => e.fmt(fmt),
                            )
                        }
                    }
                }
            };
        }

        {
            let display = &rust::import("std::fmt", "Display");
            let fmt = &rust::import("std::fmt", "Formatter");
            let result = &rust::import("std::fmt", "Result");
            let encode = &rust::import("percent_encoding", "utf8_percent_encode");
            let ascii_set = &rust::import("percent_encoding", "NON_ALPHANUMERIC");

            t.line();

            quote_in! { t =>
                pub struct PathEncode<T>(pub T);

                impl<T> #display for PathEncode<T>
                where
                    T: #display
                {
                    fn fmt(&self, fmt: &mut #fmt) -> #result {
                        write!(fmt, "{}", #encode(&self.0.to_string(), #ascii_set))
                    }
                }
            };
        }

        Ok(t)
    }
}

impl RootCodegen for ReqwestUtils {
    fn generate(&self, root: Root) -> Result<()> {
        let Root { files, .. } = root;
        files.insert(self.utils_package.clone(), self.reproto()?);
        Ok(())
    }
}

struct ReqwestService {
    result: Type,
    path_encode: Type,
    client: Type,
}

impl ReqwestService {
    pub fn new(result: Type, path_encode: Type) -> Self {
        Self {
            result,
            path_encode,
            client: rust::import("reqwest", "Client").into(),
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

        let name = &ItemStr::from(format!("{}_Reqwest", name));
        let url_ty = &Type::from(rust::import("reqwest", "Url"));

        quote_in! { *container =>
            #attributes
            #[allow(non_camel_case_types)]
            pub struct #name {
                client: #(&self.client),
                url: #url_ty,
            }

            impl #name {
                #(Constructor {
                    result: &self.result,
                    client: &self.client,
                    body,
                    url_ty: &url_ty,
                })

                #(for e in &body.endpoints join (#<line>) =>
                    #(ref tokens =>
                        let http = match e.http1.as_ref() {
                            Some(http) => http,
                            None => continue,
                        };

                        quote_in! { *tokens =>
                            #(Comments(&e.comment))
                            #(Endpoint {
                                result: &self.result,
                                path_encode: &self.path_encode,
                                e,
                                http,
                            })
                        };
                    )
                )
            }
        };

        Ok(())
    }
}

/// Builds a constructor for the service struct.
struct Constructor<'el> {
    body: &'el RpServiceBody,
    result: &'el Type,
    client: &'el Type,
    url_ty: &'el Type,
}

impl<'el> FormatInto<Rust> for Constructor<'el> {
    fn format_into(self, t: &mut Tokens<Rust>) {
        let Constructor {
            body,
            result,
            client,
            url_ty,
            ..
        } = self;

        let option_url_ty = match body.http.url {
            Some(_) => Type::option(url_ty.clone()),
            None => url_ty.clone(),
        };

        quote_in! { *t =>
            pub fn new(client: #client, url: #(&option_url_ty)) -> #result<Self> {
                #(ref t => {
                    if let Some(url) = &body.http.url {
                        quote_in! { *t =>
                            let url = match url {
                                Some(url) => url,
                                None => #(url_ty)::parse(#(quoted(&**url)))?,
                            };
                        }
                    }
                })

                Ok(Self { client, url })
            }
        };
    }
}

/// Write full path to a string.
struct WritePath<'el> {
    var: &'el str,
    path: &'el RpPathSpec,
    path_encode: &'el Type,
}

impl<'el> FormatInto<Rust> for WritePath<'el> {
    fn format_into(self, tokens: &mut Tokens<Rust>) {
        let WritePath {
            var,
            path,
            path_encode,
        } = self;

        quote_in! { *tokens =>
            #(for step in &path.steps join (#<push>) =>
                #var.push_str("/");
                #(for part in &step.parts join (#<push>) =>
                    #(match part {
                        core::RpPathPart::Variable(arg) => {
                            write!(#var, "{}", #(path_encode)(#(arg.safe_ident())))?;
                        }
                        core::RpPathPart::Segment(s) => {
                            #var.push_str(#(quoted(s.as_str())));
                        }
                    })
                )
            )
        }
    }
}

/// Build an endpoint method for the service struct.
struct Endpoint<'el> {
    result: &'el Type,
    path_encode: &'el Type,
    e: &'el RustEndpoint,
    http: &'el RpEndpointHttp1,
}

impl<'el> FormatInto<Rust> for Endpoint<'el> {
    fn format_into(self, t: &mut Tokens<Rust>) {
        use core::RpHttpMethod::*;

        let Endpoint {
            result,
            path_encode,
            e,
            http,
        } = self;

        // import trait
        t.register(rust::import("std::fmt", "Write"));

        let res = if let Some(res) = &http.response {
            quote!(#result<#res>)
        } else {
            quote!(#result<()>)
        };

        let args = e
            .arguments
            .iter()
            .map(|a| quote!(#(a.safe_ident()): #(a.channel.ty())));

        let method = match http.method {
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Update => "UPDATE",
            Delete => "DELETE",
            Patch => "PATCH",
            Head => "HEAD",
        };

        let method_ty = rust::import("reqwest", "Method");

        quote_in! { *t =>
            pub async fn #(e.safe_ident())(&self, #(for a in args join(, ) => #a)) -> #res {
                use std::fmt::Write as _;

                #(if let Some(path) = &e.http.path {
                    let mut path_ = String::new();

                    #(WritePath {
                        var: "path_",
                        path,
                        path_encode,
                    })

                    let url_ = self.url.join(&path_)?;
                } else {
                    let url_ = self.url.clone();
                })

                #(if let Some(req) = &e.request {
                    let req_ = self.client
                        .request(#method_ty::#method, url_)
                        .json(&#(req.safe_ident()));
                } else {
                    let req_ = self.client
                        .request(#method_ty::#method, url_);
                })

                #(if e.response.is_some() {
                    let res_ = req_.send().await?;
                    let body_ = res_.json().await?;
                    Ok(body_)
                } else {
                    req_.send().await?;
                    Ok(())
                })
            }
        }
    }
}
