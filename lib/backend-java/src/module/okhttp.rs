//! Module that adds fasterxml annotations to generated classes.

use codegen::ServiceCodegen;
use core::{RpEndpoint, RpPathPart};
use core::errors::*;
use genco::{Cons, IntoTokens, Java, Quoted, Tokens};
use genco::java::{Argument, Class, Constructor, Field, Method, Modifier, imported, local, optional};
use listeners::{Configure, EndpointExtra, Listeners, ServiceAdded};
use utils::Override;

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

/// Model for a nested `Builder` class.
pub struct Builder<'el> {
    ty: Java<'el>,
    optional: Java<'el>,
    client_field: Field<'el>,
    fields: Vec<(Cons<'el>, Java<'el>)>,
}

impl<'el> Builder<'el> {
    fn build(&self) -> Method<'el> {
        let mut args = Tokens::new();

        args.append(self.client_field.var());

        for &(ref name, _) in &self.fields {
            args.append(toks!["this.", name.clone()]);
        }

        let mut build = Method::new("build");

        build.returns = self.ty.clone();

        build.body.push(toks![
            "return new ",
            self.ty.clone(),
            "(",
            args.join(", "),
            ");",
        ]);

        build
    }

    fn field<N: Into<Cons<'el>>>(&self, name: N, ty: Java<'el>) -> Field<'el> {
        let mut f = Field::new(ty, name);
        f.initializer(toks![self.optional.clone(), ".empty()"]);
        f.modifiers = vec![Modifier::Private];
        f
    }

    fn setter<N: Into<Cons<'el>>>(&self, name: N, ty: Java<'el>) -> Method<'el> {
        let name = name.into();
        let arg = Argument::new(ty.as_value(), name.clone());

        let mut f = Method::new(name.clone());

        f.body.push(toks![
            "this.",
            name.clone(),
            " = ",
            self.optional.clone(),
            ".of(",
            arg.var(),
            ");",
        ]);
        f.body.push("return this;");

        f.returns = local("OkHttpBuilder");
        f.arguments.push(arg);

        f
    }
}

impl<'el> IntoTokens<'el, Java<'el>> for Builder<'el> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        let mut builder = Class::new("OkHttpBuilder");
        builder.modifiers = vec![Modifier::Static, Modifier::Public];

        for &(ref name, ref ty) in &self.fields {
            builder.fields.push(self.field(name.clone(), ty.clone()));
            builder.methods.push(self.setter(name.clone(), ty.clone()));
        }

        let mut c = Constructor::new();

        c.arguments.push(Argument::new(
            self.client_field.ty(),
            self.client_field.var(),
        ));
        c.body.push(toks![
            "this.",
            self.client_field.var(),
            " = ",
            self.client_field.var(),
            ";",
        ]);

        builder.constructors.push(c);

        builder.fields.push(self.client_field.clone());
        builder.body.push(self.build());
        builder.into_tokens()
    }
}

pub struct OkHttpServiceCodegen {
    string: Java<'static>,
    client: Java<'static>,
    request: Java<'static>,
    http_url: Java<'static>,
}

impl OkHttpServiceCodegen {
    pub fn new() -> Self {
        Self {
            string: imported("java.lang", "String"),
            client: imported("okhttp3", "OkHttpClient"),
            request: imported("okhttp3", "Request"),
            http_url: imported("okhttp3", "HttpUrl"),
        }
    }
}

impl OkHttpServiceCodegen {
    fn request<'el>(
        &self,
        mut method: Method<'el>,
        endpoint: &'el RpEndpoint,
        extra: &EndpointExtra<'el>,
    ) -> Result<Method<'el>> {
        let EndpointExtra { ref arguments, .. } = *extra;

        let mut builder = Tokens::new();

        if let Some(ref method) = endpoint.http.method {
            println!("method = {:?}", method);
        }

        builder.push(toks![
            "final ",
            self.http_url.clone(),
            " url = new ",
            self.http_url.clone(),
            ".Builder()",
        ]);

        if let Some(ref path) = endpoint.http.path {
            for step in &path.steps {
                let mut args = Tokens::new();

                for part in &step.parts {
                    match *part {
                        RpPathPart::Variable(ref s) => {
                            let arg = arguments
                                .iter()
                                .find(|a| a.var().as_ref() == s)
                                .ok_or_else(|| format!("Missing argument: {}", s))?;

                            let ty = arg.ty();

                            if ty.is_primitive() {
                                args.append(toks![ty.as_boxed(), ".toString(", s.as_str(), ")"]);
                            } else {
                                args.append(s.as_str());
                            }
                        }
                        RpPathPart::Segment(ref s) => {
                            args.append(s.to_string().quoted());
                        }
                    }
                }

                builder.nested(toks![".addPathSegment(", args.join(" + "), ")"]);
            }
        }

        builder.nested(".build();");

        builder.push(toks!["new ", self.request.clone(), ".Builder()"]);
        builder.nested(toks![".url(url)"]);
        builder.nested(toks![".build();"]);

        method.body.push(builder);
        method.body.push(
            "throw new IllegalStateException(\"not \
             implemented\");",
        );

        Ok(method)
    }
}

impl ServiceCodegen for OkHttpServiceCodegen {
    fn generate(&self, e: ServiceAdded) -> Result<()> {
        let ServiceAdded {
            backend,
            body,
            spec,
            extra,
            ..
        } = e;

        let mut c = Class::new("OkHttp");
        c.implements = vec![local(spec.name())];

        let client_field = Field::new(self.client.clone(), "client");

        for (endpoint, extra) in body.endpoints.values().zip(extra.iter()) {
            let EndpointExtra {
                ref name,
                ref response_ty,
                ref arguments,
                ..
            } = *extra;

            if !endpoint.has_http_support() {
                let mut m = Method::new(name.clone());
                m.annotation(Override);
                m.returns = response_ty.clone();
                m.arguments.extend(arguments.iter().cloned());

                // FIXME: compile-time error?
                m.body.push(
                    "throw new RuntimeException(\"endpoint does not support HTTP\");",
                );

                c.methods.push(m);
                continue;
            }

            let mut m = Method::new(name.clone());
            m.annotation(Override);
            m.returns = response_ty.clone();
            m.arguments.extend(arguments.iter().cloned());

            c.methods.push(self.request(m, endpoint, extra)?);
        }

        let mut builder_fields = Vec::new();

        builder_fields.push(Field::new(
            optional(
                self.string.clone(),
                backend.optional.with_arguments(vec![self.string.clone()]),
            ),
            "baseUrl",
        ));

        c.constructors.push({
            let mut c = Constructor::new();
            let client_arg = Argument::new(client_field.ty(), client_field.var());

            c.body.push(toks![
                "this.",
                client_field.var(),
                " = ",
                client_arg.var(),
                ";",
            ]);

            for f in &builder_fields {
                c.body.push(toks!["this.", f.var(), " = ", f.var(), ";"]);
            }

            c.arguments.push(client_arg);
            c.arguments.extend(builder_fields.iter().map(
                |f| Argument::new(f.ty(), f.var()),
            ));
            c
        });

        let builder = Builder {
            ty: local(c.name()),
            optional: backend.optional.clone(),
            client_field: client_field.clone(),
            fields: builder_fields
                .iter()
                .map(|f| (f.var(), f.ty()))
                .collect::<Vec<_>>(),
        };

        c.fields.push(client_field);
        c.fields.extend(builder_fields.iter().cloned());

        spec.body.push(c);
        spec.body.push(builder);
        Ok(())
    }
}

impl Listeners for Module {
    fn configure(&self, e: Configure) {
        e.options.service_generators.push(Box::new(
            OkHttpServiceCodegen::new(),
        ));
    }
}
