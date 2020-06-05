//! Module that adds fasterxml annotations to generated classes.

use crate::codegen::{Configure, ServiceAdded, ServiceCodegen};
use crate::flavored::{JavaEndpoint, RpEndpointHttp1, RpPathStep};
use crate::serialization::Serialization;
use crate::utils::Override;
use core::errors::Result;
use core::Loc;
use genco::java::{self, Argument, Class, Constructor, Field, Method, Modifier, VOID};
use genco::{nested, push, toks, Cons, IntoTokens, Java, Quoted, Tokens};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {}

pub struct Module {
    #[allow(dead_code)]
    config: Config,
}

impl Module {
    pub fn new(config: Config) -> Module {
        Module { config: config }
    }
}

impl Module {
    pub fn initialize(self, e: Configure, serialization: Serialization) {
        e.options
            .service_generators
            .push(Box::new(OkHttpServiceCodegen::new(serialization)));
    }
}

/// Model for a nested `Builder` class.
pub struct Builder<'a, 'el> {
    ty: Java<'el>,
    optional: Java<'el>,
    client: Field<'el>,
    base_url: Field<'el>,
    default_base_url: Option<&'el str>,
    ser: Field<'el>,
    serialization: &'a Serialization,
}

impl<'a, 'el> Builder<'a, 'el> {
    fn build(&self) -> Method<'el> {
        macro_rules! opt_check {
            ($d:expr, $v:expr) => {
                push!(
                    $d,
                    "final ",
                    $v.ty().as_value(),
                    " ",
                    $v.var(),
                    " = ",
                    required(&$v),
                    ";"
                )
            };
        }

        macro_rules! opt_default {
            ($d:expr, $v:expr, $default:expr) => {
                push!(
                    $d,
                    "final ",
                    $v.ty().as_value(),
                    " ",
                    $v.var(),
                    " = ",
                    default(&$v, $default),
                    ";"
                )
            };
        }

        let mut args = Tokens::new();

        args.append(self.client.var());
        args.append(self.base_url.var());
        args.append(self.ser.var());

        let mut build = Method::new("build");

        build.returns = self.ty.clone();

        if let Some(default_base_url) = self.default_base_url {
            let default = toks![
                self.base_url.ty(),
                ".parse(",
                default_base_url.quoted(),
                ")"
            ];

            opt_default!(build.body, self.base_url, default);
        } else {
            opt_check!(build.body, self.base_url);
        }

        if let Some(default_builder) = self.serialization.default_builder() {
            opt_default!(build.body, self.ser, default_builder);
        } else {
            opt_check!(build.body, self.ser);
        }

        push!(
            build.body,
            "return new ",
            self.ty,
            "(",
            args.join(", "),
            ");"
        );

        return build;

        fn required<'el>(field: &Field<'el>) -> Tokens<'el, Java<'el>> {
            let exc = java::imported("java.lang", "RuntimeException");
            let msg = format!("{}: is a required field", field.var());
            let exc = toks![exc, "(", msg.quoted(), ")"];

            toks!["this.", field.var(), ".orElseThrow(() -> new ", exc, ")"]
        }

        fn default<'el, T: Into<Tokens<'el, Java<'el>>>>(
            field: &Field<'el>,
            default: T,
        ) -> Tokens<'el, Java<'el>> {
            toks![
                "this.",
                field.var(),
                ".orElseGet(() -> ",
                default.into(),
                ")"
            ]
        }
    }

    fn field<N: Into<Cons<'el>>>(&self, name: N, ty: Java<'el>) -> Field<'el> {
        let mut f = Field::new(self.optional.with_arguments(vec![ty.clone()]), name);
        f.initializer(toks![self.optional.clone(), ".empty()"]);
        f.modifiers = vec![Modifier::Private];
        f
    }

    fn setter<N: Into<Cons<'el>>>(&self, name: N, ty: Java<'el>) -> Method<'el> {
        let name = name.into();
        let arg = Argument::new(ty.as_value(), name.clone());

        let mut f = Method::new(name.clone());

        push!(
            f.body,
            "this.",
            name,
            " = ",
            self.optional,
            ".of(",
            arg.var(),
            ");"
        );
        push!(f.body, "return this;");

        f.returns = java::local("OkHttpBuilder");
        f.arguments.push(arg);

        f
    }
}

impl<'a, 'el> IntoTokens<'el, Java<'el>> for Builder<'a, 'el> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        let mut builder = Class::new("OkHttpBuilder");
        builder.modifiers = vec![Modifier::Static, Modifier::Public];

        let base_url = self.field(self.base_url.var(), self.base_url.ty());
        let ser = self.field(self.ser.var(), self.ser.ty());

        builder.fields.push(base_url.clone());
        builder.fields.push(ser.clone());

        builder
            .methods
            .push(self.setter(self.base_url.var(), self.base_url.ty()));
        builder
            .methods
            .push(self.setter(self.ser.var(), self.ser.ty()));

        let mut c = Constructor::new();

        c.arguments
            .push(Argument::new(self.client.ty(), self.client.var()));

        push!(
            c.body,
            "this.",
            self.client.var(),
            " = ",
            self.client.var(),
            ";"
        );

        builder.constructors.push(c);

        builder.fields.push(self.client.clone());

        builder.body.push(self.build());
        builder.into_tokens()
    }
}

pub struct OkHttpServiceCodegen {
    serialization: Serialization,
    client: Java<'static>,
    request: Java<'static>,
    http_url: Java<'static>,
    callback: Java<'static>,
    call: Java<'static>,
    response: Java<'static>,
    io_exc: Java<'static>,
    optional: Java<'static>,
    completable_future: Java<'static>,
}

impl OkHttpServiceCodegen {
    pub fn new(serialization: Serialization) -> Self {
        Self {
            serialization,
            client: java::imported("okhttp3", "OkHttpClient"),
            request: java::imported("okhttp3", "Request"),
            http_url: java::imported("okhttp3", "HttpUrl"),
            callback: java::imported("okhttp3", "Callback"),
            call: java::imported("okhttp3", "Call"),
            response: java::imported("okhttp3", "Response"),
            io_exc: java::imported("java.io", "IOException"),
            optional: java::imported("java.util", "Optional"),
            completable_future: java::imported("java.util.concurrent", "CompletableFuture"),
        }
    }
}

impl OkHttpServiceCodegen {
    fn request<'el>(
        &self,
        mut method: Method<'el>,
        e: &'el Loc<JavaEndpoint>,
        http: &'el RpEndpointHttp1,
        client: Field<'el>,
        base_url: Field<'el>,
        ser: &Field<'el>,
    ) -> Result<Method<'el>> {
        let request_var = e
            .request
            .as_ref()
            .map(|r| toks![ser.clone(), ".encode(", r.safe_ident(), ")"])
            .unwrap_or_else(|| toks!["null"]);

        method.body.push({
            let mut t = Tokens::new();

            t.push(toks![
                "final ",
                self.http_url.clone(),
                " url_ = this.",
                base_url.var(),
                ".newBuilder()",
            ]);

            for step in &http.path.steps {
                let args = step_args(step)?;
                t.nested(toks![".addPathSegment(", args.join(" + "), ")"]);
            }

            t.nested(".build();");

            t
        });

        method.body.push({
            let mut t = Tokens::new();

            push!(
                t,
                "final ",
                self.request,
                " req_ = new ",
                self.request,
                ".Builder()"
            );

            let method = http.method.as_str();

            nested!(t, ".url(url_)");
            // TODO: actually provide the body
            nested!(t, ".method(", method.quoted(), ", ", request_var, ")");
            nested!(t, ".build();");

            t
        });

        push!(
            method.body,
            "final ",
            method.returns,
            " future_ = new ",
            method.returns,
            "();"
        );

        method.body.push({
            let mut t = Tokens::new();

            let new_call = toks!["this.", client.var(), ".newCall(req_)"];

            push!(t, new_call, ".enqueue(new ", self.callback, "() {");

            t.nested({
                let mut t = Tokens::new();

                t.push({
                    let e = Argument::new(self.io_exc.clone(), "e");

                    let mut m = Method::new("onFailure");
                    m.annotation(Override);
                    m.arguments.push(Argument::new(self.call.clone(), "call"));
                    m.arguments.push(e.clone());

                    push!(m.body, "future_.completeExceptionally(", e.var(), ");");

                    m
                });

                t.push({
                    let response = Argument::new(self.response.clone(), "response");

                    let mut m = Method::new("onResponse");
                    m.annotation(Override);
                    m.arguments.push(Argument::new(self.call.clone(), "call"));
                    m.arguments.push(response.clone());

                    // push!(m.body, "future.fail(", e.var(), ");");

                    m.body.push({
                        let mut t = Tokens::new();

                        let exc = toks![
                            "new IOException(",
                            "bad response: ".quoted(),
                            " + response)",
                        ];

                        t.push_into(|t| {
                            t.push("if (!response.isSuccessful()) {");
                            nested!(t, "future_.completeExceptionally(", exc, ");");
                            nested!(t, "return;");
                            t.push("}");
                        });

                        t.push({
                            let mut t = Tokens::new();

                            let input = "response.body().byteStream()";

                            let var = if let Some(r) = e.response.as_ref() {
                                t.push(self.serialization.decode(
                                    ser,
                                    r.ty(),
                                    input,
                                    "body",
                                    |e| {
                                        let mut t = Tokens::new();
                                        push!(t, "future_.completeExceptionally(", e, ");");
                                        push!(t, "return;");
                                        Ok(t)
                                    },
                                )?);

                                "body"
                            } else {
                                "null"
                            };

                            push!(t, "future_.complete(", var, ");");

                            t.join_line_spacing()
                        });

                        t.join_line_spacing()
                    });

                    m
                });

                t.join_line_spacing()
            });

            push!(t, "});");

            t
        });

        method.body.push("return future_;");

        method.body = method.body.join_line_spacing();
        return Ok(method);

        fn step_args<'el>(step: &'el RpPathStep) -> Result<Tokens<'el, Java<'el>>> {
            let mut args = Tokens::new();

            for part in &step.parts {
                match *part {
                    core::RpPathPart::Variable(ref arg) => {
                        let ty = arg.channel.ty();

                        if ty.is_primitive() {
                            args.append(toks![ty.as_boxed(), ".toString(", arg.safe_ident(), ")"]);
                        } else {
                            args.append(arg.safe_ident());
                        }
                    }
                    core::RpPathPart::Segment(ref s) => {
                        args.append(s.to_string().quoted());
                    }
                }
            }

            Ok(args)
        }
    }
}

impl ServiceCodegen for OkHttpServiceCodegen {
    fn generate(&self, e: ServiceAdded) -> Result<()> {
        let ServiceAdded { body, spec, .. } = e;

        let closable = java::imported("java.io", "Closeable");

        let client = Field::new(self.client.clone(), "client");
        let base_url = Field::new(self.http_url.clone(), "baseUrl");
        let ser = self.serialization.field();

        let ok_http = {
            let mut c = Class::new("OkHttp");
            c.implements = vec![closable];

            for e in &body.endpoints {
                if let Some(http) = e.http1.as_ref() {
                    let mut m = Method::new(e.safe_ident());
                    m.returns = self.completable_future.with_arguments(vec![http
                        .response
                        .as_ref()
                        .unwrap_or(&VOID)
                        .clone()]);
                    m.arguments.extend(e.arguments.iter().cloned());

                    c.methods.push(self.request(
                        m,
                        e,
                        http,
                        client.clone(),
                        base_url.clone(),
                        &ser,
                    )?);
                }
            }

            c.constructors.push({
                let mut c = Constructor::new();
                let client_arg = Argument::new(client.ty(), client.var());
                let base_arg = Argument::new(base_url.ty(), base_url.var());
                let ser_arg = Argument::new(ser.ty(), ser.var());

                push!(c.body, "this.", client.var(), " = ", client_arg.var(), ";");
                push!(c.body, "this.", base_url.var(), " = ", base_arg.var(), ";");
                push!(c.body, "this.", ser.var(), " = ", ser_arg.var(), ";");

                c.arguments.push(client_arg);
                c.arguments.push(base_arg);
                c.arguments.push(ser_arg);
                c
            });

            c.fields.push(client.clone());
            c.fields.push(base_url.clone());
            c.fields.push(ser.clone());

            c.methods.push({
                let io_exc = java::imported("java.io", "IOException");
                let mut m = Method::new("close");
                m.annotation(Override);
                m.throws = Some(toks![io_exc]);

                push!(
                    m.body,
                    client.var(),
                    ".dispatcher().executorService().shutdown();"
                );

                push!(m.body, client.var(), ".connectionPool().evictAll();");

                m.body.push_into(|t| {
                    push!(t, "if (", client.var(), ".cache() != null) {");
                    nested!(t, client.var(), ".cache().close();");
                    push!(t, "}");
                });

                m.body = m.body.join_line_spacing();

                m
            });

            c
        };

        let default_base_url = body.http.url.as_ref().map(|s| s.as_str());

        spec.body.push(ok_http);

        spec.body.push(Builder {
            ty: java::local("OkHttp"),
            optional: self.optional.clone(),
            client: client,
            base_url: base_url,
            default_base_url: default_base_url,
            ser: ser,
            serialization: &self.serialization,
        });

        Ok(())
    }
}
