//! Module that adds fasterxml annotations to generated classes.

use backend::{FromNaming, Naming, SnakeCase};
use backend::errors::*;
use core::{Loc, RpChannel, RpEndpoint};
use genco::{Cons, IntoTokens, Java, Quoted, Tokens};
use genco::java::{Argument, Class, Constructor, Field, Method, Modifier, imported, local};
use java_backend::JavaBackend;
use java_options::JavaOptions;
use listeners::{Listeners, ServiceAdded};
use std::borrow::Borrow;
use std::rc::Rc;

const CLIENT_STUB_NAME: &'static str = "ClientStub";
const SERVER_STUB_NAME: &'static str = "ServerStub";

pub enum MethodType {
    Unary,
    ClientStreaming,
    ServerStreaming,
    BidiStreaming,
    Unknown,
}

impl MethodType {
    pub fn variant_name(&self) -> &'static str {
        use self::MethodType::*;

        match *self {
            Unary => "UNARY",
            ClientStreaming => "CLIENT_STREAMING",
            ServerStreaming => "SERVER_STREAMING",
            BidiStreaming => "BIDI_STREAMING",
            Unknown => "UNKNOWN",
        }
    }
}

/// @Generated("message") annotation
struct Generated<'a, 'el>(&'a Module, Cons<'el>);

impl<'a, 'el> IntoTokens<'el, Java<'el>> for Generated<'a, 'el> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        toks!["@", self.0.generated.clone(), "(", self.1.quoted(), ")"]
    }
}

/// @Override annotation
struct Override<'a>(&'a Module);

impl<'a, 'el> IntoTokens<'el, Java<'el>> for Override<'a> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        toks!["@", self.0.override_.clone()]
    }
}

/// Embedded marshaller for Void.
struct VoidMarshaller<'a>(&'a Module, &'a JavaBackend);

impl<'a, 'el> IntoTokens<'el, Java<'el>> for VoidMarshaller<'a> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        use self::Modifier::*;

        let void = &self.1.void;

        let mut class = Class::new("VoidMarshaller");
        class.implements = vec![self.0.marshaller.with_arguments(vec![void.clone()])];
        class.modifiers.push(Static);

        // parse
        class.methods.push({
            let mut m = Method::new("parse");
            m.annotation(Override(self.0));
            m.returns = void.clone();
            m.arguments.push(Argument::new(
                self.0.input_stream.clone(),
                "stream",
            ));

            m.body.push("return null;");
            m
        });

        // stream
        class.methods.push({
            let mut m = Method::new("stream");
            m.annotation(Override(self.0));
            m.returns = self.0.input_stream.clone();
            m.arguments.push(Argument::new(void.clone(), "value"));

            m.body.push(toks![
                "return new ",
                self.0.bais.clone(),
                "(new byte[0]);",
            ]);
            m
        });

        class.into_tokens()
    }
}

/// Embedded marshaller for Json.
struct JsonMarshaller<'a>(&'a Module);

impl<'a, 'el> IntoTokens<'el, Java<'el>> for JsonMarshaller<'a> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        use self::Modifier::*;

        let tpl = local("T");

        let mapper = Field::new(self.0.object_mapper.clone(), "mapper");
        let ty = Field::new(
            self.0.type_reference.with_arguments(vec![tpl.clone()]),
            "type",
        );

        let mut class = Class::new("JsonMarshaller");
        class.parameters.append(tpl.clone());
        class.implements = vec![self.0.marshaller.with_arguments(vec![tpl.clone()])];
        class.modifiers.push(Static);

        class.fields.push(mapper);
        class.fields.push(ty.clone());

        class.constructors.push({
            let mut c = Constructor::new();

            c.arguments.push(Argument::new(ty.ty(), ty.var()));

            c.body.push(toks![
                "this.mapper = ",
                self.0.mapper_provider.clone(),
                ".get();",
            ]);
            c.body.push(toks!["this.", ty.var(), " = ", ty.var(), ";"]);
            c
        });

        // parse
        class.methods.push({
            let mut m = Method::new("parse");
            m.annotation(Override(self.0));
            m.returns = tpl.clone();
            m.arguments.push(Argument::new(
                self.0.input_stream.clone(),
                "stream",
            ));

            m.body.push({
                let mut t = Tokens::new();
                t.push("try {");
                t.nested(toks![
                    "return this.mapper.readValue(stream, this.",
                    ty.var(),
                    ");",
                ]);
                t.push("} catch (final Exception e) {");
                t.nested("throw new RuntimeException(e);");
                t.push("}");
                t
            });

            m
        });

        // stream
        class.methods.push({
            let mut m = Method::new("stream");
            m.annotation(Override(self.0));
            m.returns = self.0.input_stream.clone();
            m.arguments.push(Argument::new(tpl.clone(), "value"));

            m.body.push({
                let mut t = Tokens::new();
                t.push("final byte[] bytes;");
                t.push("try {");
                t.nested("bytes = this.mapper.writeValueAsBytes(value);");
                t.push("} catch (final Exception e) {");
                t.nested("throw new RuntimeException(e);");
                t.push("}");
                t.push(toks!["return new ", self.0.bais.clone(), "(bytes);"]);
                t
            });

            m
        });

        class.into_tokens()
    }
}

pub struct Module {
    snake_to_upper: Box<Naming>,
    mapper_provider: Java<'static>,
    override_: Java<'static>,
    bais: Java<'static>,
    generated: Java<'static>,
    abstract_stub: Java<'static>,
    channel: Java<'static>,
    call_options: Java<'static>,
    bindable_service: Java<'static>,
    server_service_definition: Java<'static>,
    client_calls: Java<'static>,
    server_calls: Java<'static>,
    stream_observer: Java<'static>,
    method_descriptor: Java<'static>,
    marshaller: Java<'static>,
    input_stream: Java<'static>,
    object_mapper: Java<'static>,
    type_reference: Java<'static>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            snake_to_upper: SnakeCase::new().to_upper_snake(),
            mapper_provider: imported("io.reproto", "MapperProvider"),
            override_: imported("java.lang", "Override"),
            bais: imported("java.io", "ByteArrayInputStream"),
            generated: imported("javax.annotation", "Generated"),
            abstract_stub: imported("io.grpc.stub", "AbstractStub"),
            channel: imported("io.grpc", "Channel"),
            call_options: imported("io.grpc", "CallOptions"),
            bindable_service: imported("io.grpc", "BindableService"),
            server_service_definition: imported("io.grpc", "ServerServiceDefinition"),
            client_calls: imported("io.grpc.stub", "ClientCalls"),
            server_calls: imported("io.grpc.stub", "ServerCalls"),
            stream_observer: imported("io.grpc.stub", "StreamObserver"),
            method_descriptor: imported("io.grpc", "MethodDescriptor"),
            marshaller: imported("io.grpc", "MethodDescriptor.Marshaller"),
            input_stream: imported("java.io", "InputStream"),
            object_mapper: imported("com.fasterxml.jackson.databind", "ObjectMapper"),
            type_reference: imported("com.fasterxml.jackson.core.type", "TypeReference"),
        }
    }

    /// Get the MethodType variant for the given endpoint.
    fn method_type(&self, backend: &JavaBackend, endpoint: &RpEndpoint) -> Result<MethodType> {
        use self::RpChannel::*;

        let request = backend.endpoint_request(endpoint)?.map(|v| v.1);

        let out = match (request, endpoint.response.as_ref().map(Loc::value)) {
            (Some(&Unary { .. }), Some(&Unary { .. })) => MethodType::Unary,
            (Some(&Streaming { .. }), Some(&Unary { .. })) => MethodType::ClientStreaming,
            (Some(&Unary { .. }), Some(&Streaming { .. })) => MethodType::ServerStreaming,
            (Some(&Streaming { .. }), Some(&Streaming { .. })) => MethodType::BidiStreaming,
            _ => MethodType::Unknown,
        };

        Ok(out)
    }

    /// Build the method descriptor field.
    fn method_field<'el>(
        &self,
        service_name: Rc<String>,
        backend: &JavaBackend,
        request_ty: &Java<'static>,
        response_ty: &Java<'static>,
        method_type: &MethodType,
        endpoint: &'el RpEndpoint,
    ) -> Field<'el> {
        use self::Modifier::*;

        let method_name = Rc::new(format!(
            "METHOD_{}",
            self.snake_to_upper.convert(endpoint.id.as_str())
        ));

        let descriptor_ty = self.method_descriptor.with_arguments(vec![
            request_ty.clone(),
            response_ty.clone(),
        ]);

        let mut field = Field::new(descriptor_ty, method_name.clone());
        field.modifiers = vec![Public, Static, Final];

        field.initializer({
            let mut init = Tokens::new();

            init.push(toks![
                self.method_descriptor.clone(),
                ".<",
                request_ty.clone(),
                ", ",
                response_ty.clone(),
                ">newBuilder()",
            ]);

            init.nested({
                let mut t = Tokens::new();
                t.push(toks![
                    ".setType(",
                    self.method_descriptor.clone(),
                    ".MethodType.", method_type.variant_name(), ")",
                ]);

                t.push(toks![
                    ".setFullMethodName(",
                    self.method_descriptor.clone(), ".generateFullMethodName(",
                    service_name.quoted(), ", ", endpoint.name.as_str().quoted(),
                    "))",
                ]);

                if request_ty != &backend.void {
                    t.push(toks![
                        ".setRequestMarshaller(new JsonMarshaller(",
                        "new ", self.type_reference.with_arguments(vec![request_ty.clone()]), "(){}",
                        "))",
                    ]);
                } else {
                    t.push(".setRequestMarshaller(new VoidMarshaller())");
                }

                if response_ty != &backend.void {
                    t.push(toks![
                        ".setResponseMarshaller(new JsonMarshaller(",
                        "new ", self.type_reference.with_arguments(vec![response_ty.clone()]), "(){}",
                        "))",
                    ]);
                } else {
                    t.push(".setResponseMarshaller(new VoidMarshaller())");
                }

                t.push(".build();");
                t
            });

            let mut t = Tokens::new();
            t.nested(init);
            t
        });

        field
    }

    fn is_not_whitespace(c: char) -> bool {
        !c.is_whitespace()
    }

    fn javadoc_comments<'el, S: Borrow<str>>(out: &mut Vec<Cons<'el>>, input: &'el [S]) {
        if input.is_empty() {
            return;
        }

        let offset = input
            .iter()
            .flat_map(|line| {
                line.borrow().find(Self::is_not_whitespace).into_iter()
            })
            .min();
        let offset = offset.unwrap_or(0usize);

        out.push("<pre>".into());

        for comment in input {
            let comment = comment.borrow();
            out.push(comment[usize::min(offset, comment.len())..].into());
        }

        out.push("</pre>".into());
    }

    /// Build the client method helper for making calls.
    ///
    /// This is built differently depending on which MethodType has been used.
    fn client_method<'el>(
        &self,
        name: Cons<'el>,
        field: &Field<'el>,
        request_ty: &Java<'static>,
        response_ty: &Java<'static>,
        method_type: &MethodType,
        endpoint: &'el RpEndpoint,
    ) -> Method<'el> {
        use self::MethodType::*;
        use self::Modifier::*;

        let mut method = Method::new(name);
        method.modifiers = vec![Public];

        Self::javadoc_comments(&mut method.comments, &endpoint.comment);

        let request_observer_ty = self.stream_observer.with_arguments(
            vec![request_ty.clone()],
        );

        let observer_ty = self.stream_observer.with_arguments(
            vec![response_ty.clone()],
        );

        let request_arg = Argument::new(request_ty.clone(), "request");
        let observer_arg = Argument::new(observer_ty, "observer");

        let new_call = toks!["getChannel().newCall(", field.var(), ", getCallOptions())"];

        let mut args = Tokens::new();
        args.append(new_call);

        match *method_type {
            Unary => {
                args.append(request_arg.var());
                args.append(observer_arg.var());

                method.arguments.push(request_arg);
                method.arguments.push(observer_arg);

                method.body.push(toks![
                    self.client_calls.clone(), ".asyncUnaryCall(", args.join(", "), ");",
                ]);
            }
            ClientStreaming => {
                args.append(observer_arg.var());

                method.arguments.push(observer_arg);
                method.returns = request_observer_ty;

                method.body.push(toks![
                    "return ",
                    self.client_calls.clone(),
                    ".asyncClientStreamingCall(", args.join(", "), ");",
                ]);
            }
            ServerStreaming => {
                args.append(request_arg.var());
                args.append(observer_arg.var());

                method.arguments.push(request_arg);
                method.arguments.push(observer_arg);

                method.body.push(toks![
                    self.client_calls.clone(),
                    ".asyncServerStreamingCall(", args.join(", "), ");",
                ]);
            }
            Unknown | BidiStreaming => {
                args.append(observer_arg.var());

                method.arguments.push(observer_arg);
                method.returns = request_observer_ty;

                method.body.push(toks![
                    "return ",
                    self.client_calls.clone(),
                    ".asyncBidiStreamingCall(", args.join(", "), ");",
                ]);
            }
        }

        method
    }

    /// Build the server method that will handle the request.
    fn server_method<'el>(
        &self,
        name: Cons<'el>,
        field: &Field<'el>,
        request_ty: &Java<'static>,
        response_ty: &Java<'static>,
        method_type: &MethodType,
        endpoint: &'el RpEndpoint,
    ) -> Method<'el> {
        use self::MethodType::*;
        use self::Modifier::*;

        let mut method = Method::new(name);
        method.modifiers = vec![Public];

        Self::javadoc_comments(&mut method.comments, &endpoint.comment);

        let request_observer_ty = self.stream_observer.with_arguments(
            vec![request_ty.clone()],
        );

        let observer_ty = self.stream_observer.with_arguments(
            vec![response_ty.clone()],
        );

        let request_arg = Argument::new(request_ty.clone(), "request");
        let observer_arg = Argument::new(observer_ty, "observer");

        let mut args = toks![field.var()];

        match *method_type {
            // Takes an argument which is the immediate request.
            Unary | ServerStreaming => {
                args.append(observer_arg.var());

                method.arguments.push(request_arg);
                method.arguments.push(observer_arg);

                method.body.push(toks![
                    self.server_calls.clone(),
                    ".asyncUnimplementedUnaryCall(", args.join(", "), ");",
                ]);
            }
            // All streaming identical streaming implementations.
            ClientStreaming | BidiStreaming | Unknown => {
                args.append(observer_arg.var());

                method.arguments.push(observer_arg);
                method.returns = request_observer_ty;

                method.body.push(toks![
                    "return ",
                    self.server_calls.clone(),
                    ".asyncUnimplementedStreamingCall(", args.join(", "), ");",
                ]);
            }
        }

        method
    }


    /// Setup the class corresponding to the client stub.
    fn client_stub<'el>(&self) -> Class<'el> {
        use self::Modifier::*;

        let name = CLIENT_STUB_NAME;

        let mut spec = Class::new(name.clone());
        spec.annotation(Generated(self, "Generated by reproto".into()));
        spec.modifiers = vec![Static];
        spec.extends = Some(self.abstract_stub.with_arguments(vec![local(name.clone())]));

        spec.constructors.push({
            let mut c = Constructor::new();

            c.arguments.push(
                Argument::new(self.channel.clone(), "channel"),
            );

            c.body.push("super(channel);");

            c
        });

        spec.constructors.push({
            let mut c = Constructor::new();

            c.arguments.push(
                Argument::new(self.channel.clone(), "channel"),
            );

            c.arguments.push(Argument::new(
                self.call_options.clone(),
                "callOptions",
            ));

            c.body.push("super(channel, callOptions);");

            c
        });

        spec.methods.push({
            let mut m = Method::new("build");
            m.modifiers = vec![Protected];
            m.annotation(Override(self));
            m.returns = local(name.clone());

            m.arguments.push(
                Argument::new(self.channel.clone(), "channel"),
            );

            m.arguments.push(Argument::new(
                self.call_options.clone(),
                "callOptions",
            ));

            m.body.push(toks![
                "return new ",
                name.clone(),
                "(channel, callOptions);",
            ]);

            m
        });

        spec
    }

    /// Setup the class corresponding to the server stub.
    fn server_stub<'el>(&self) -> Class<'el> {
        use self::Modifier::*;

        let name = SERVER_STUB_NAME;

        let mut spec = Class::new(name);
        spec.annotation(Generated(self, "Generated by reproto".into()));
        spec.modifiers = vec![Static, Abstract];
        spec.implements = vec![self.bindable_service.clone()];

        spec
    }

    /// Build the addMethod call for a given endpoint to populate the server definition.
    fn server_definition_add_method<'el>(
        &self,
        name: Cons<'el>,
        field: &Field<'el>,
        method_type: &MethodType,
    ) -> Tokens<'el, Java<'el>> {
        use self::MethodType::*;

        let mut args = toks![field.var()];

        match *method_type {
            Unary => {
                args.append(toks![
                    self.server_calls.clone(), ".asyncUnaryCall(this::", name, ")",
                ]);
            }
            ClientStreaming => {
                args.append(toks![
                    self.server_calls.clone(),
                    ".asyncClientStreamingCall(this::", name, ")",
                ]);
            }
            ServerStreaming => {
                args.append(toks![
                    self.server_calls.clone(),
                    ".asyncServerStreamingCall(this::", name, ")",
                ]);
            }
            Unknown | BidiStreaming => {
                args.append(toks![
                    self.server_calls.clone(),
                    ".asyncBidiStreamingCall(this::", name, ")",
                ]);
            }
        }

        toks![".addMethod(", args.join(", "), ")"]
    }
}

impl Listeners for Module {
    fn configure(&self, options: &mut JavaOptions) -> Result<()> {
        options.suppress_service_methods = true;
        Ok(())
    }

    fn service_added(&self, e: &mut ServiceAdded) -> Result<()> {
        let mut client_stub = self.client_stub();
        let mut server_stub = self.server_stub();

        let mut bind_service = Method::new("bindService");
        bind_service.returns = self.server_service_definition.clone();
        bind_service.annotation(Override(self));

        bind_service.body.push(toks![
            "return ", self.server_service_definition.clone(),
        ]);

        let service_name = Rc::new(format!(
            "{}.{}",
            e.backend.java_package(&e.body.name.package).parts.join("."),
            e.body.name.join(".")
        ));

        bind_service.body.nested(toks![
            ".builder(",
            service_name.clone().quoted(),
            ")",
        ]);

        for (endpoint, name) in e.body.endpoints.values().zip(
            e.endpoint_names.iter().cloned(),
        )
        {
            let request = e.backend.endpoint_request(endpoint)?.map(|v| v.1);

            let request_ty = if let Some(req) = request {
                e.backend.into_java_type(req.ty())?
            } else {
                e.backend.void.clone()
            };

            let response_ty = if let Some(ref res) = endpoint.response.as_ref() {
                e.backend.into_java_type(res.ty())?
            } else {
                e.backend.void.clone()
            };

            let method_type = self.method_type(e.backend, endpoint)?;

            let field = self.method_field(
                service_name.clone(),
                &e.backend,
                &request_ty,
                &response_ty,
                &method_type,
                endpoint,
            );

            let server_method = self.server_method(
                name.clone(),
                &field,
                &request_ty,
                &response_ty,
                &method_type,
                endpoint,
            );

            let client_method = self.client_method(
                name.clone(),
                &field,
                &request_ty,
                &response_ty,
                &method_type,
                endpoint,
            );

            bind_service.body.nested(self.server_definition_add_method(
                name.clone(),
                &field,
                &method_type,
            ));

            e.spec.body.push(field);
            server_stub.methods.push(server_method);
            client_stub.methods.push(client_method);
        }

        bind_service.body.nested(".build();");

        server_stub.body.push(bind_service);

        e.spec.body.push(client_stub);
        e.spec.body.push(server_stub);
        e.spec.body.push(JsonMarshaller(self));
        e.spec.body.push(VoidMarshaller(self, &e.backend));
        Ok(())
    }
}
