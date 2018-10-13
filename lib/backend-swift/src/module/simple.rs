//! gRPC module for Rust.

use backend::Initializer;
use compiler::Comments;
use core::errors::Result;
use core::{self, Loc};
use flavored::{RpEnumBody, RpField, RpInterfaceBody, RpPackage, RpSubType, SwiftName};
use genco::swift::{imported, Swift};
use genco::{Cons, IntoTokens, Quoted, Tokens};
use std::rc::Rc;
use {
    Compiler, EnumAdded, EnumCodegen, FileSpec, InterfaceAdded, InterfaceCodegen, Options,
    PackageAdded, PackageCodegen, TupleAdded, TupleCodegen, TypeAdded, TypeCodegen,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Simple<'el> {
    DateTime,
    Bytes,
    Array {
        argument: Box<Simple<'el>>,
    },
    Map {
        key: Box<Simple<'el>>,
        value: Box<Simple<'el>>,
    },
    Name {
        name: Swift<'el>,
    },
    Any {
        ty: Swift<'el>,
    },
    Type {
        ty: Swift<'el>,
    },
}

impl<'el> Simple<'el> {
    /// Decode the given value.
    fn decode_value(
        &self,
        codegen: &Codegen,
        name: Cons<'el>,
        var: Tokens<'el, Swift<'el>>,
    ) -> Result<Tokens<'el, Swift<'el>>> {
        use self::Simple::*;

        let unbox = match *self {
            DateTime => {
                let string = toks!["try decode_value(", var, " as? String)"];
                let date = toks![codegen.formatter.clone(), "().date(from: ", string, ")"];
                toks!["try decode_value(", date, ")"]
            }
            Bytes => toks![
                codegen.data.clone(),
                "(base64Encoded: try decode_value(",
                var,
                " as? String))"
            ],
            Array { ref argument } => {
                let argument = argument.decode_value(codegen, name.clone(), "inner".into())?;

                return Ok(toks![
                    "try decode_array(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", inner: { inner in ",
                    argument,
                    " })"
                ]);
            }
            Map { ref value, .. } => {
                let value = value.decode_value(codegen, name.clone(), "value".into())?;
                return Ok(toks![
                    "try decode_map(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", value: { value in ",
                    value,
                    " })"
                ]);
            }
            Name { ref name } => {
                return Ok(toks!["try ", name.clone(), ".decode(json: ", var, ")"]);
            }
            Any { .. } => toks![var],
            Type { ref ty } => toks![unbox(var, ty.clone())],
        };

        return Ok(toks![
            "try decode_name(",
            unbox,
            ", name: ",
            name.quoted(),
            ")"
        ]);

        /// Call the unbox function for the given type.
        fn unbox<'el>(var: Tokens<'el, Swift<'el>>, ty: Swift<'el>) -> Tokens<'el, Swift<'el>> {
            toks!["unbox(", var, ", as: ", ty, ".self)"]
        }
    }

    /// Decode the given value.
    fn encode_value(
        &self,
        codegen: &Codegen,
        name: &'el str,
        var: Tokens<'el, Swift<'el>>,
    ) -> Result<Tokens<'el, Swift<'el>>> {
        use self::Simple::*;

        let encode = match *self {
            DateTime => toks![codegen.formatter.clone(), "().string(from: ", var, ")"],
            Bytes => toks![var, ".base64EncodedString()"],
            Array { ref argument } => {
                let argument = argument.encode_value(codegen, name, "inner".into())?;

                toks![
                    "try encode_array(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", inner: { inner in ",
                    argument,
                    " })"
                ]
            }
            Map { ref value, .. } => {
                let value = value.encode_value(codegen, name, "value".into())?;

                toks![
                    "try encode_map(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", value: { value in ",
                    value,
                    " })"
                ]
            }
            Name { .. } => toks!["try ", var, ".encode()"],
            _ => var,
        };

        return Ok(encode.into());
    }
}

pub struct GuardMissing<'el>(Cons<'el>, Tokens<'el, Swift<'el>>, Cons<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for GuardMissing<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let GuardMissing(dest, source, name) = self;

        let mut t = Tokens::new();

        t.push(toks!["guard let ", dest, " = ", source, " else {"]);

        t.nested(toks![
            "throw SerializationError.missing(",
            name.quoted(),
            ")"
        ]);

        t.push("}");

        t
    }
}

pub struct Module {}

impl Module {
    pub fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, options: &mut Self::Options) -> Result<()> {
        let codegen = Rc::new(Codegen::new());
        options.type_gens.push(Box::new(codegen.clone()));
        options.tuple_gens.push(Box::new(codegen.clone()));
        options.enum_gens.push(Box::new(codegen.clone()));
        options.interface_gens.push(Box::new(codegen.clone()));
        options.package_gens.push(Box::new(codegen.clone()));
        Ok(())
    }
}

struct Codegen {
    data: Swift<'static>,
    formatter: Swift<'static>,
}

impl Codegen {
    pub fn new() -> Codegen {
        Self {
            data: imported("Foundation", "Data"),
            formatter: imported("Foundation", "ISO8601DateFormatter"),
        }
    }

    // Setup a field initializer.
    pub fn encode_field<'a, A>(
        &self,
        field: &'a RpField,
        append: A,
    ) -> Result<Tokens<'a, Swift<'a>>>
    where
        A: Fn(Tokens<'a, Swift<'a>>) -> Tokens<'a, Swift<'a>>,
    {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let name = field.name();

        if field.is_optional() {
            t.push({
                let mut t = Tokens::new();

                let value = field.ty.simple().encode_value(self, name, "value".into())?;

                t.push(toks!["if let value = self.", ident, " {"]);
                t.nested(append(value));
                t.push("}");

                t
            });
        } else {
            let value = field
                .ty
                .simple()
                .encode_value(self, name, toks!["self.", ident])?;
            t.push(append(value));
        }

        Ok(t.join_line_spacing())
    }

    fn decode_field<'a, I>(
        &self,
        compiler: &Compiler,
        field: &'a RpField,
        index: I,
    ) -> Result<Tokens<'a, Swift<'a>>>
    where
        I: for<'b> Fn(&'b RpField, Cons<'b>) -> (Cons<'b>, Tokens<'b, Swift<'b>>),
    {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let f_ident = Rc::new(format!("f_{}", field.ident()));

        let (name, index) = index(field, Cons::from("json"));

        if field.is_optional() {
            t.push({
                let ty = compiler.into_field(field)?;

                let mut t = Tokens::new();

                t.push(toks!["var ", ident, ": ", ty, " = Optional.none"]);

                t.push({
                    let mut t = Tokens::new();

                    t.push(toks!["if let value = ", index, " {",]);

                    t.nested({
                        let mut t = Tokens::new();

                        let value = field.ty.simple().decode_value(self, name, "value".into())?;

                        t.push(toks![ident, " = Optional.some(", value, ")"]);

                        t.join_line_spacing()
                    });

                    t.push("}");

                    t
                });

                t.join_line_spacing()
            });
        } else {
            t.push(GuardMissing(
                f_ident.clone().into(),
                toks![index],
                name.clone(),
            ));

            let value = field.ty.simple().decode_value(self, name, toks![f_ident])?;
            t.push(toks!["let ", ident, " = ", value]);
        }

        Ok(t.join_line_spacing())
    }

    fn type_index<'a>(field: &'a RpField, var: Cons<'a>) -> (Cons<'a>, Tokens<'a, Swift<'a>>) {
        let name = field.name();
        (
            Cons::from(name),
            toks![var, "[", Cons::from(name).quoted(), "]"],
        )
    }

    fn utils_package(&self) -> RpPackage {
        RpPackage::parse("reproto_simple")
    }

    fn utils<'el>(&self) -> Result<FileSpec<'el>> {
        let mut out = FileSpec::default();

        let numerics = vec!["Int", "UInt", "Int32", "Int64", "UInt32", "UInt64"];
        let floats = vec!["Float", "Double"];
        let simple = vec!["String", "Bool"];

        out.0.push({
            let mut t = Tokens::new();

            t.push("enum SerializationError: Error {");
            t.nested("case missing(String)");
            t.nested("case invalid(String)");
            t.nested("case bad_value()");
            t.push("}");

            t
        });

        out.0.push(decode_name_func());
        out.0.push(decode_value_func());

        for ty in numerics.iter().chain(floats.iter()).cloned() {
            out.0.push(unbox_number_func(ty, &numerics, &floats));
        }

        for ty in simple.iter().cloned() {
            out.0.push(unbox_simple_func(ty));
        }

        out.0.push(decode_array_func());
        out.0.push(encode_array_func());
        out.0.push(decode_map_func());
        out.0.push(encode_map_func());

        return Ok(out);

        /// Build a simple unboxing functions.
        fn unbox_simple_func<'el>(ty: &'el str) -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func unbox(_ value: Any, as type: ",
                ty,
                ".Type) -> ",
                ty,
                "? {"
            ]);
            t.nested(toks!["return value as? ", ty]);
            t.push("}");

            t
        }

        /// Build an integer unboxing function.
        ///
        /// This is more complicated since it needs to handle numeric conversions.
        fn unbox_number_func<'el>(
            ty: &'el str,
            numerics: &[&'el str],
            floats: &[&'el str],
        ) -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func unbox(_ value: Any, as type: ",
                ty,
                ".Type) -> ",
                ty,
                "? {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push("switch value {");

                t.nested({
                    let mut t = Tokens::new();

                    for ck in numerics.iter().cloned() {
                        if ty == ck {
                            continue;
                        }

                        t.push(toks!["case let n as ", ck, ":"]);
                        t.nested(toks!["return ", ty, "(exactly: n)"]);
                    }

                    for ck in floats.iter().cloned() {
                        if ty == ck {
                            continue;
                        }

                        t.push(toks!["case let n as ", ck, ":"]);
                        t.nested(toks!["return ", ty, "(n)"]);
                    }

                    t.push("default:");
                    t.nested(toks!["return value as? ", ty]);

                    t
                });

                t.push("}");

                t
            });

            t.push("}");
            t
        }

        /// Build an array decoding function.
        fn decode_array_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func decode_array<T>(_ value: Any, name: String, inner: (Any) throws -> T) \
                 throws -> [T] {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push("let array = try decode_name(value as? [Any], name: name)");
                t.push("var out = [T]()");
                t.push("for item in array {");
                t.nested("out.append(try inner(item))");
                t.push("}");
                t.push("return out");

                t
            });
            t.push("}");

            t
        }

        /// Build an array encoding function.
        fn encode_array_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func encode_array<T>(_ array: [T], name: String, inner: (T) throws -> Any) \
                 throws -> [Any] {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push("var out = [Any]()");
                t.push("for item in array {");
                t.nested("out.append(try inner(item))");
                t.push("}");
                t.push("return out");

                t
            });
            t.push("}");

            t
        }

        /// Build an array decoding function.
        fn decode_map_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func decode_map<T>(_ map: Any, name: String, value: (Any) throws -> T) throws -> \
                 [String: T] {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push("let map = try decode_name(map as? [String: Any], name: name)");
                t.push("var out = [String: T]()");
                t.push("for (k, v) in map {");
                t.nested("out[k] = try value(v)");
                t.push("}");
                t.push("return out");

                t
            });
            t.push("}");

            t
        }

        /// Build an array encoding function.
        fn encode_map_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func encode_map<T>(_ map: [String: T], name: String, value: (T) throws -> Any) \
                 throws -> [String: Any] {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push("var out = [String: Any]()");
                t.push("for (k, v) in map {");
                t.nested("out[k] = try value(v)");
                t.push("}");
                t.push("return out");

                t
            });
            t.push("}");

            t
        }

        /// Build a generic decoding function with named errors.
        fn decode_name_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks![
                "func decode_name<T>(_ unbox: T?, name string: String) throws -> T {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["guard let value = unbox else {"]);
                t.nested("throw SerializationError.invalid(string)");
                t.push("}");

                t.push("return value");

                t
            });

            t.push("}");
            t
        }

        /// Build a generic decoding function.
        fn decode_value_func<'el>() -> Tokens<'el, Swift<'el>> {
            let mut t = Tokens::new();

            t.push(toks!["func decode_value<T>(_ value: T?) throws -> T {"]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["guard let value = value else {"]);
                t.nested("throw SerializationError.bad_value()");
                t.push("}");

                t.push("return value");

                t
            });

            t.push("}");
            t
        }
    }
}

impl TypeCodegen for Codegen {
    fn generate(&self, e: TypeAdded) -> Result<()> {
        let TypeAdded {
            container,
            compiler,
            name,
            fields,
        } = e;

        container.push({
            let mut t = Tokens::new();

            t.push(toks!["public extension ", name, " {"]);

            t.push({
                let mut t = Tokens::new();

                // decode function
                t.nested(decode(self, compiler, name, &fields)?);
                t.nested(encode(self, &fields)?);

                t.join_line_spacing()
            });

            t.push("}");
            t
        });

        return Ok(());

        fn decode<'a>(
            codegen: &Codegen,
            compiler: &Compiler,
            name: &'a SwiftName,
            fields: &[&'a RpField],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name,
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();
                let mut args = Tokens::new();

                if !fields.is_empty() {
                    t.push(toks!["let json = try decode_value(json as? [String: Any])"]);

                    for field in fields.iter().cloned() {
                        let ident = field.safe_ident();
                        t.push(codegen.decode_field(compiler, field, Codegen::type_index)?);
                        args.append(toks![ident.clone(), ": ", ident.clone()]);
                    }
                } else {
                    t.push(toks!["let _ = try decode_value(json as? [String: Any])"]);
                }

                t.push(toks!["return ", name, "(", args.join(", "), ")"]);
                t.join_line_spacing()
            });

            t.push("}");

            Ok(t)
        }

        fn encode<'a>(codegen: &Codegen, fields: &[&'a RpField]) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push("func encode() throws -> [String: Any] {");
            t.nested({
                let mut t = Tokens::new();

                if !fields.is_empty() {
                    t.push("var json = [String: Any]()");
                    t.push({
                        let mut t = Tokens::new();

                        for field in fields.iter().cloned() {
                            t.push(codegen.encode_field(field, |value| {
                                toks!["json[", field.name().quoted(), "] = ", value]
                            })?);
                        }

                        t
                    });
                    t.push("return json");
                } else {
                    t.push("return [String: Any]()");
                }

                t.join_line_spacing()
            });
            t.push("}");

            Ok(t)
        }
    }
}

impl TupleCodegen for Codegen {
    fn generate(&self, e: TupleAdded) -> Result<()> {
        let TupleAdded {
            container,
            compiler,
            name,
            fields,
        } = e;

        container.push({
            let mut t = Tokens::new();

            t.push(toks!["public extension ", name, " {"]);

            t.push({
                let mut t = Tokens::new();

                t.nested(decode(self, compiler, name, &fields)?);
                t.nested(encode(self, &fields)?);

                t.join_line_spacing()
            });

            t.push("}");
            t
        });

        return Ok(());

        fn decode<'a>(
            codegen: &Codegen,
            compiler: &Compiler,
            name: &'a SwiftName,
            fields: &[&'a RpField],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            let mut args = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name,
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["let json = try decode_value(json as? [Any])"]);

                for (i, field) in fields.iter().cloned().enumerate() {
                    let ident = field.safe_ident();
                    t.push(codegen.decode_field(compiler, field, |_, var| {
                        let i = Cons::from(i.to_string());
                        (
                            Cons::from(format!("[{}]", i.as_ref())),
                            toks!["Optional.some(", var, "[", i, "])"],
                        )
                    })?);
                    args.append(toks![ident.clone(), ": ", ident.clone()]);
                }

                t.join_line_spacing()
            });

            t.nested(toks!["return ", name, "(", args.join(", "), ")"]);
            t.push("}");

            Ok(t)
        }

        fn encode<'el, 'a>(
            codegen: &Codegen,
            fields: &[&'a RpField],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push("func encode() throws -> [Any] {");
            t.nested({
                let mut t = Tokens::new();

                t.push("var json = [Any]()");
                t.push_unless_empty({
                    let mut t = Tokens::new();

                    for field in fields.iter().cloned() {
                        t.push(
                            codegen
                                .encode_field(field, |value| toks!["json.append(", value, ")"])?,
                        );
                    }

                    t
                });
                t.push("return json");

                t.join_line_spacing()
            });
            t.push("}");

            Ok(t)
        }
    }
}

impl EnumCodegen for Codegen {
    fn generate(&self, e: EnumAdded) -> Result<()> {
        let EnumAdded {
            container,
            name,
            body,
            ..
        } = e;

        container.push({
            let mut t = Tokens::new();

            t.push(toks!["public extension ", name, " {"]);

            t.push({
                let mut t = Tokens::new();

                // decode function
                t.nested(decode(body, name)?);
                t.nested(encode(body)?);

                t.join_line_spacing()
            });

            t.push("}");
            t
        });

        return Ok(());

        fn decode<'a>(body: &'a RpEnumBody, name: &'a SwiftName) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name,
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                push!(t, "let json = try decode_value(json)");
                let unbox = toks!["unbox(json, as: ", body.enum_type.ty(), ")"];
                push!(t, "let value = try decode_value(", unbox, ")");

                t.push({
                    let mut t = Tokens::new();

                    t.push("switch value {");

                    match body.variants {
                        core::RpVariants::String { ref variants } => for v in variants {
                            t.nested_into(|t| {
                                push!(t, "case ", v.value.to_string().quoted(), ":");
                                nested!(t, "return ", name, ".", v.ident());
                            });
                        },
                        core::RpVariants::Number { ref variants } => for v in variants {
                            t.nested_into(|t| {
                                push!(t, "case ", v.value.to_string(), ":");
                                nested!(t, "return ", name, ".", v.ident());
                            });
                        },
                    }

                    t.nested({
                        let mut t = Tokens::new();
                        t.push("default:");
                        t.nested("throw SerializationError.bad_value()");
                        t
                    });

                    t.push("}");

                    t
                });

                t.join_line_spacing()
            });

            t.push("}");

            Ok(t)
        }

        fn encode<'a>(body: &'a RpEnumBody) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            push!(t, "func encode() throws -> ", body.enum_type.ty(), " {");
            t.nested({
                let mut t = Tokens::new();

                t.push("switch self {");

                match body.variants {
                    core::RpVariants::String { ref variants } => for v in variants {
                        t.nested_into(|t| {
                            push!(t, "case .", v.ident(), ":");
                            nested!(t, "return ", v.value.to_string().quoted());
                        });
                    },
                    core::RpVariants::Number { ref variants } => for v in variants {
                        t.nested_into(|t| {
                            push!(t, "case .", v.ident(), ":");
                            nested!(t, "return ", v.value.to_string());
                        });
                    },
                }

                t.push("}");

                t
            });
            t.push("}");

            Ok(t)
        }
    }
}

impl InterfaceCodegen for Codegen {
    fn generate(&self, e: InterfaceAdded) -> Result<()> {
        let InterfaceAdded {
            container,
            name,
            body,
            ..
        } = e;

        container.push({
            let mut t = Tokens::new();

            t.push_unless_empty(Comments(&body.comment));
            t.push(toks!["public extension ", name, " {"]);

            t.push({
                let mut t = Tokens::new();

                match body.sub_type_strategy {
                    core::RpSubTypeStrategy::Tagged { ref tag, .. } => {
                        t.nested(decode_tag(name, tag.as_str(), &body.sub_types)?);
                        t.nested(encode_tag(tag.as_str(), &body.sub_types)?);
                    }
                    core::RpSubTypeStrategy::Untagged => {
                        t.nested(decode_untagged(name, body)?);
                        t.nested(encode_untagged(&body.sub_types)?);
                    }
                }

                t.join_line_spacing()
            });

            t.push("}");
            t
        });

        return Ok(());

        /// Build a method to decode a tagged interface.
        fn decode_tag<'el, S>(
            name: &'el SwiftName,
            tag: &'el str,
            sub_types: S,
        ) -> Result<Tokens<'el, Swift<'el>>>
        where
            S: IntoIterator<Item = &'el Loc<RpSubType>>,
        {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name,
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["let json = try decode_value(json as? [String: Any])"]);
                t.push(toks![
                    "let type = try decode_name(json[",
                    tag.quoted(),
                    "] as? String, name: ",
                    tag.quoted(),
                    ")"
                ]);

                t.push({
                    let mut t = Tokens::new();
                    t.push("switch type {");

                    for sub_type in sub_types.into_iter() {
                        let ident = sub_type.ident.as_str();

                        t.nested({
                            let mut t = Tokens::new();
                            t.push(toks!["case ", sub_type.name().quoted(), ":"]);
                            t.nested(toks![
                                "let v = try ",
                                Loc::borrow(&sub_type.name),
                                ".decode(json: json)"
                            ]);
                            t.nested(toks!["return ", name, ".", ident, "(v)"]);
                            t
                        });
                    }

                    t.nested({
                        let mut t = Tokens::new();

                        t.push("default:");
                        t.nested("throw SerializationError.invalid(type)");

                        t
                    });

                    t.push("}");
                    t
                });

                t.join_line_spacing()
            });
            t.push("}");

            Ok(t)
        }

        /// Build a method to decode a tagged interface.
        fn encode_tag<'el, S>(tag: &'el str, sub_types: S) -> Result<Tokens<'el, Swift<'el>>>
        where
            S: IntoIterator<Item = &'el Loc<RpSubType>>,
        {
            let mut t = Tokens::new();

            t.push(toks!["func encode() throws -> [String: Any] {"]);
            t.nested({
                let mut t = Tokens::new();
                t.push("switch self {");

                for sub_type in sub_types.into_iter() {
                    let name = sub_type.name();
                    let ident = sub_type.ident.as_str();

                    t.nested({
                        let mut t = Tokens::new();
                        t.push(toks!["case .", ident, "(let s):"]);
                        t.nested(toks!["var json = try s.encode()"]);
                        t.nested(toks!["json[", tag.quoted(), "] = ", name.quoted()]);
                        t.nested(toks!["return json"]);
                        t
                    });
                }

                t.push("}");
                t
            });
            t.push("}");

            Ok(t)
        }

        /// Build a method to decode a tagged interface.
        fn decode_untagged<'el>(
            name: &'el SwiftName,
            body: &'el RpInterfaceBody,
        ) -> Result<Tokens<'el, Swift<'el>>> {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name,
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["let json = try decode_value(json as? [String: Any])"]);

                // shared optional keys
                let optional = quoted_tags(body.fields.iter().filter(|f| f.is_optional()));

                let keys = "keys";
                // keys of incoming data
                push!(
                    t,
                    "let ",
                    keys,
                    " = Set(json.keys).subtracting(",
                    optional,
                    ")"
                );

                t.push({
                    let mut t = Tokens::new();

                    for sub_type in &body.sub_types {
                        let ident = sub_type.ident.as_str();
                        let tags = quoted_tags(sub_type.fields.iter().filter(|f| f.is_optional()));
                        let req = quoted_tags(
                            body.fields
                                .iter()
                                .chain(sub_type.fields.iter())
                                .filter(|f| f.is_required()),
                        );

                        t.push_into(|t| {
                            let d =
                                toks!["try ", Loc::borrow(&sub_type.name), ".decode(json: json)"];

                            push!(t, "if ", keys, ".subtracting(", tags, ") == ", req, " {");
                            nested!(t, "return ", name, ".", ident, "(", d, ")");
                            push!(t, "}");
                        });
                    }

                    let m = "no legal field combinations".quoted();
                    push!(t, "throw SerializationError.invalid(", m, ")");

                    t.join_line_spacing()
                });

                t.join_line_spacing()
            });
            t.push("}");

            return Ok(t);

            fn quoted_tags<'el, F>(fields: F) -> Tokens<'el, Swift<'el>>
            where
                F: IntoIterator<Item = &'el Loc<RpField>>,
            {
                let mut tags = Tokens::new();

                for field in fields {
                    tags.append(field.name().quoted());
                }

                toks!["[", tags.join(", "), "]"]
            }
        }

        /// Build a method to decode a tagged interface.
        fn encode_untagged<'el, S>(sub_types: S) -> Result<Tokens<'el, Swift<'el>>>
        where
            S: IntoIterator<Item = &'el Loc<RpSubType>>,
        {
            let mut t = Tokens::new();

            t.push(toks!["func encode() throws -> [String: Any] {"]);
            t.nested({
                let mut t = Tokens::new();
                t.push("switch self {");

                for sub_type in sub_types.into_iter() {
                    let ident = sub_type.ident.as_str();

                    t.nested({
                        let mut t = Tokens::new();
                        t.push(toks!["case .", ident, "(let s):"]);
                        t.nested(toks!["return try s.encode()"]);
                        t
                    });
                }

                t.push("}");
                t
            });
            t.push("}");

            Ok(t)
        }
    }
}

impl PackageCodegen for Codegen {
    fn generate(&self, e: PackageAdded) -> Result<()> {
        e.files.push((self.utils_package(), self.utils()?));
        Ok(())
    }
}
