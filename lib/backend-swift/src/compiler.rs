//! Backend for Swift

use {FileSpec, Options, EXT};
use backend::{PackageProcessor, PackageUtils};
use core::{Handle, Loc, RpEnumBody, RpField, RpInterfaceBody, RpName, RpPackage, RpSubType,
           RpSubTypeStrategy, RpTupleBody, RpType, RpTypeBody, RpVersionedPackage};
use core::errors::*;
use genco::{Cons, IntoTokens, Quoted, Tokens};
use std::rc::Rc;
use swift::{imported, Swift};
use trans::{self, Environment};

/// Documentation comments.
pub struct Comments<'el, S: 'el>(&'el [S]);

impl<'el, S: 'el + AsRef<str>> IntoTokens<'el, Swift<'el>> for Comments<'el, S> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut t = Tokens::new();

        for c in self.0.iter() {
            t.push(toks!["// ", c.as_ref()]);
        }

        t
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

const TYPE_SEP: &'static str = "_";

pub struct Compiler<'el> {
    pub env: &'el Environment,
    options: Options,
    handle: &'el Handle,
    data: Swift<'static>,
    date: Swift<'static>,
    formatter: Swift<'static>,
}

impl<'el> Compiler<'el> {
    pub fn new(env: &'el Environment, options: Options, handle: &'el Handle) -> Compiler<'el> {
        Compiler {
            env: env,
            options: options,
            handle: handle,
            data: imported("Foundation", "Data"),
            date: imported("Foundation", "Date"),
            formatter: imported("Foundation", "ISO8601DateFormatter"),
        }
    }

    /// Convert the type name
    ///
    /// Optionally also emit the necessary attributes to suppress warnings for bad naming
    /// conventions.
    fn convert_name<'a>(&self, name: &'a RpName) -> Result<Tokens<'a, Swift<'a>>> {
        let registered = self.env.lookup(name)?;
        let local_name = registered.local_name(&name, |p| p.join(TYPE_SEP), |c| c.join(TYPE_SEP));
        let package_name = self.package(&name.package).parts.join("_");
        return Ok(toks![package_name, "_", local_name]);
    }

    /// Convert to the type declaration of a field.
    pub fn field_type<'a>(&self, ty: &'a RpType) -> Result<Tokens<'a, Swift<'a>>> {
        use self::RpType::*;

        let ty = match *ty {
            String => toks!["String"],
            DateTime => toks![self.date.clone()],
            Bytes => toks![self.data.clone()],
            Signed { size: 32 } => toks!["Int32"],
            Signed { size: 64 } => toks!["Int64"],
            Unsigned { size: 32 } => toks!["UInt32"],
            Unsigned { size: 64 } => toks!["UInt64"],
            Float => toks!["Float"],
            Double => toks!["Double"],
            Boolean => toks!["Bool"],
            Array { ref inner } => {
                let argument = self.field_type(inner)?;
                toks!["[", argument, "]"]
            }
            Name { ref name } => toks![self.convert_name(name)?],
            Map { ref key, ref value } => {
                let key = self.field_type(key)?;
                let value = self.field_type(value)?;
                toks!["[", key, ": ", value, "]"]
            }
            Any => toks!["Any"],
            _ => return Err(format!("unsupported type: {}", ty).into()),
        };

        Ok(ty)
    }

    fn into_field<'a>(&self, field: &'a RpField) -> Result<Tokens<'a, Swift<'a>>> {
        let stmt = self.field_type(&field.ty)?;

        if field.is_optional() {
            return Ok(toks![stmt, "?"]);
        }

        Ok(stmt)
    }

    /// Decode the given value.
    fn decode_value<'a>(
        &self,
        ty: &'a RpType,
        name: Cons<'a>,
        var: Tokens<'a, Swift<'a>>,
    ) -> Result<Tokens<'a, Swift<'a>>> {
        use self::RpType::*;

        let unbox = match *ty {
            String => unbox(var, "String"),
            DateTime => {
                let string = toks!["try decode_value(", var, " as? String)"];
                let date = toks![self.formatter.clone(), "().date(from: ", string, ")"];
                toks!["try decode_value(", date, ")"]
            }
            Bytes => toks![
                self.data.clone(),
                "(base64Encoded: try decode_value(",
                var,
                " as? String))"
            ],
            Signed { size: 32 } => unbox(var, "Int32"),
            Signed { size: 64 } => unbox(var, "Int64"),
            Unsigned { size: 32 } => unbox(var, "UInt32"),
            Unsigned { size: 64 } => unbox(var, "UInt64"),
            Float => unbox(var, "Float"),
            Double => unbox(var, "Double"),
            Boolean => unbox(var, "Bool"),
            Array { ref inner } => {
                let inner = self.decode_value(inner, name.clone(), "inner".into())?;
                return Ok(toks![
                    "try decode_array(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", inner: { inner in ",
                    inner,
                    " })"
                ]);
            }
            Map { ref value, .. } => {
                let value = self.decode_value(value, name.clone(), "value".into())?;
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
            Name {
                name: ref inner_name,
            } => {
                let inner_name = self.convert_name(inner_name)?;
                return Ok(toks!["try ", inner_name, ".decode(json: ", var, ")"]);
            }
            Any => toks![var],
            _ => return Err(format!("unsupported type: {}", ty).into()),
        };

        return Ok(toks![
            "try decode_name(",
            unbox,
            ", name: ",
            name.quoted(),
            ")"
        ]);

        /// Call the unbox function for the given type.
        fn unbox<'el>(var: Tokens<'el, Swift<'el>>, ty: &'el str) -> Tokens<'el, Swift<'el>> {
            toks!["unbox(", var, ", as: ", ty, ".self)"]
        }
    }

    fn decode_field<'a, I>(&self, field: &'a RpField, index: I) -> Result<Tokens<'a, Swift<'a>>>
    where
        I: for<'b> Fn(&'b RpField, Cons<'b>) -> (Cons<'b>, Tokens<'b, Swift<'b>>),
    {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let f_ident = Rc::new(format!("f_{}", field.ident()));

        let (name, index) = index(field, Cons::from("json"));

        if field.is_optional() {
            t.push({
                let ty = self.into_field(field)?;

                let mut t = Tokens::new();

                t.push(toks!["var ", ident, ": ", ty, " = Optional.none"]);

                t.push({
                    let mut t = Tokens::new();

                    t.push(toks!["if let value = ", index, " {",]);

                    t.nested({
                        let mut t = Tokens::new();

                        let value = self.decode_value(&field.ty, name, "value".into())?;

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

            let value = self.decode_value(&field.ty, name, toks![f_ident])?;
            t.push(toks!["let ", ident, " = ", value]);
        }

        Ok(t.join_line_spacing())
    }

    /// Decode the given value.
    fn encode_value<'a>(
        &self,
        ty: &'a RpType,
        name: &'a str,
        var: Tokens<'a, Swift<'a>>,
    ) -> Result<Tokens<'a, Swift<'a>>> {
        use self::RpType::*;

        let encode = match *ty {
            DateTime => toks![self.formatter.clone(), "().string(from: ", var, ")"],
            Bytes => toks![var, ".base64EncodedString()"],
            Array { ref inner } => {
                let inner = self.encode_value(inner, name, "inner".into())?;
                toks![
                    "try encode_array(",
                    var,
                    ", name: ",
                    name.quoted(),
                    ", inner: { inner in ",
                    inner,
                    " })"
                ]
            }
            Map { ref value, .. } => {
                let value = self.encode_value(value, name, "value".into())?;

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

    // Setup a field initializer.
    fn encode_field<'a, A>(&self, field: &'a RpField, append: A) -> Result<Tokens<'a, Swift<'a>>>
    where
        A: Fn(Tokens<'a, Swift<'a>>) -> Tokens<'a, Swift<'a>>,
    {
        let mut t = Tokens::new();

        let ident = field.safe_ident();
        let name = field.name();

        if field.is_optional() {
            t.push({
                let mut t = Tokens::new();

                let value = self.encode_value(&field.ty, name, "value".into())?;

                t.push(toks!["if let value = self.", ident, " {",]);
                t.nested(append(value));
                t.push("}");

                t
            });
        } else {
            let value = self.encode_value(&field.ty, name, toks!["self.", ident])?;
            t.push(append(value));
        }

        Ok(t.join_line_spacing())
    }

    pub fn utils_package(&self) -> RpVersionedPackage {
        let package = RpPackage::new(vec!["ReprotoUtils".to_string()]);
        RpVersionedPackage::new(package, None)
    }

    /// Build a simple unboxing functions.
    fn unbox_simple_func(&self, ty: &'el str) -> Tokens<'el, Swift<'el>> {
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
    fn unbox_number_func(
        &self,
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
    fn decode_array_func(&self) -> Tokens<'el, Swift<'el>> {
        let mut t = Tokens::new();

        t.push(toks![
            "func decode_array<T>(_ value: Any, name: String, inner: (Any) throws -> T) throws -> \
             [T] {"
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
    fn encode_array_func(&self) -> Tokens<'el, Swift<'el>> {
        let mut t = Tokens::new();

        t.push(toks![
            "func encode_array<T>(_ array: [T], name: String, inner: (T) throws -> Any) throws -> \
             [Any] {"
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
    fn decode_map_func(&self) -> Tokens<'el, Swift<'el>> {
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
    fn encode_map_func(&self) -> Tokens<'el, Swift<'el>> {
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
    fn decode_name_func(&self) -> Tokens<'el, Swift<'el>> {
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
    fn decode_value_func(&self) -> Tokens<'el, Swift<'el>> {
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

    /// Build a method to decode a tagged interface.
    fn decode_tag<S>(
        &self,
        name: Tokens<'el, Swift<'el>>,
        tag: &'el str,
        sub_types: S,
    ) -> Result<Tokens<'el, Swift<'el>>>
    where
        S: IntoIterator<Item = (&'el String, &'el Rc<Loc<RpSubType>>)>,
    {
        let mut t = Tokens::new();

        t.push(toks![
            "static func decode(json: Any) throws -> ",
            name.clone(),
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

                for (_, sub_type) in sub_types.into_iter() {
                    let n = self.convert_name(&sub_type.name)?;

                    let local_name = sub_type.local_name.as_str();

                    t.nested({
                        let mut t = Tokens::new();
                        t.push(toks!["case ", sub_type.name().quoted(), ":"]);
                        t.nested(toks!["let v = try ", n.clone(), ".decode(json: json)"]);
                        t.nested(toks!["return ", name.clone(), ".", local_name, "(v)"]);
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
    fn encode_tag<S>(&self, tag: &'el str, sub_types: S) -> Result<Tokens<'el, Swift<'el>>>
    where
        S: IntoIterator<Item = (&'el String, &'el Rc<Loc<RpSubType>>)>,
    {
        let mut t = Tokens::new();

        t.push(toks!["func encode() throws -> [String: Any] {"]);
        t.nested({
            let mut t = Tokens::new();
            t.push("switch self {");

            for (_, sub_type) in sub_types.into_iter() {
                let name = sub_type.name();
                let local_name = sub_type.local_name.as_str();

                t.nested({
                    let mut t = Tokens::new();
                    t.push(toks!["case .", local_name, "(let s):"]);
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

    /// Set up a model structure for the given fields.
    fn model_struct<'a, F>(
        &self,
        name: Tokens<'a, Swift<'a>>,
        comment: &'a [String],
        fields: F,
        codable: bool,
    ) -> Result<Tokens<'a, Swift<'a>>>
    where
        F: IntoIterator<Item = &'a Loc<RpField>>,
    {
        let fields = fields.into_iter().collect::<Vec<_>>();

        let mut t = Tokens::new();

        t.push_unless_empty(Comments(comment));

        if codable {
            t.push(toks!["public struct ", name.clone(), ": Codable {"]);
        } else {
            t.push(toks!["public struct ", name.clone(), " {"]);
        }

        // fields
        t.nested({
            let mut t = Tokens::new();

            t.push({
                let mut t = Tokens::new();

                for field in fields.iter() {
                    t.push_unless_empty(Comments(&field.comment));
                    let ty = self.into_field(field)?;
                    t.push(toks!["let ", field.safe_ident(), ": ", ty]);
                }

                t
            });

            if codable {
                if !fields.is_empty() {
                    t.push({
                        let mut t = Tokens::new();

                        t.push("enum CodingKeys: String, CodingKey {");

                        for field in fields.iter() {
                            t.nested(toks![
                                "case ",
                                field.safe_ident(),
                                " = ",
                                field.name().quoted()
                            ]);
                        }

                        t.push("}");

                        t
                    });
                }
            }

            t.join_line_spacing()
        });

        t.push("}");
        Ok(t)
    }

    /// Build a model struct for the given set of fields.
    fn model_tuple<'a, F>(
        &self,
        name: Tokens<'a, Swift<'a>>,
        comment: &'a [String],
        fields: F,
    ) -> Result<Tokens<'a, Swift<'a>>>
    where
        F: IntoIterator<Item = &'a Loc<RpField>>,
    {
        let fields = fields.into_iter().collect::<Vec<_>>();

        let mut tokens = Tokens::new();

        tokens.push(self.model_struct(name.clone(), comment, fields.iter().cloned(), false)?);

        if self.options.simple {
            tokens.push({
                let mut t = Tokens::new();

                t.push(toks!["public extension ", name.clone(), " {"]);

                t.push({
                    let mut t = Tokens::new();

                    t.nested(decode(self, name.clone(), &fields)?);
                    t.nested(encode(self, &fields)?);

                    t.join_line_spacing()
                });

                t.push("}");
                t
            });
        }

        if self.options.codable {
            tokens.push(decodable(self, name.clone(), &fields)?);
            tokens.push(encodable(name.clone(), &fields)?);
        }

        return Ok(tokens);

        fn decode<'a>(
            compiler: &Compiler,
            name: Tokens<'a, Swift<'a>>,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            let mut args = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name.clone(),
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["let json = try decode_value(json as? [Any])"]);

                for (i, field) in fields.iter().cloned().enumerate() {
                    let ident = field.safe_ident();
                    t.push(compiler.decode_field(field, |_, var| {
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

            t.nested(toks!["return ", name.clone(), "(", args.join(", "), ")"]);
            t.push("}");

            Ok(t)
        }

        fn encode<'el, 'a>(
            compiler: &Compiler<'el>,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push("func encode() throws -> [Any] {");
            t.nested({
                let mut t = Tokens::new();

                t.push("var json = [Any]()");
                t.push_unless_empty({
                    let mut t = Tokens::new();

                    for field in fields.iter().cloned() {
                        t.push(compiler
                            .encode_field(field, |value| toks!["json.append(", value, ")"])?);
                    }

                    t
                });
                t.push("return json");

                t.join_line_spacing()
            });
            t.push("}");

            Ok(t)
        }

        fn decodable<'a>(
            compiler: &Compiler,
            name: Tokens<'a, Swift<'a>>,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Decodable {"]);
            t.nested(init(compiler, fields)?);
            t.push("}");

            return Ok(t);

            fn init<'a>(
                compiler: &Compiler,
                fields: &[&'a Loc<RpField>],
            ) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public init(from decoder: Decoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("var values = try decoder.unkeyedContainer()");

                    t.push({
                        let mut t = Tokens::new();

                        for field in fields {
                            let s = toks!["self.", field.safe_ident()];
                            let ty = compiler.field_type(&field.ty)?;

                            if field.is_optional() {
                                t.push(toks![s, " = try values.decodeIfPresent(", ty, ".self)"]);
                            } else {
                                t.push(toks![s, " = try values.decode(", ty, ".self)"]);
                            }
                        }

                        t
                    });

                    t.join_line_spacing()
                });

                t.push("}");

                Ok(t)
            }
        }

        fn encodable<'a>(
            name: Tokens<'a, Swift<'a>>,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Encodable {"]);

            t.push({
                let mut t = Tokens::new();
                t.nested(encode(fields)?);
                t.join_line_spacing()
            });

            t.push("}");

            return Ok(t);

            fn encode<'a>(fields: &[&'a Loc<RpField>]) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public func encode(to encoder: Encoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("var values = encoder.unkeyedContainer()");

                    t.push({
                        let mut t = Tokens::new();

                        for field in fields {
                            let s = toks!["self.", field.safe_ident()];

                            if field.is_optional() {
                                let var = field.safe_ident();

                                t.push({
                                    let mut t = Tokens::new();

                                    t.push(toks!["if let ", var.clone(), " = ", s, "{"]);
                                    t.nested(toks!["try values.encode(", var, ")"]);
                                    t.push("}");

                                    t
                                });
                            } else {
                                t.push(toks!["try values.encode(", s, ")"]);
                            }
                        }

                        t
                    });

                    t.join_line_spacing()
                });

                t.push("}");

                Ok(t)
            }
        }
    }

    fn type_index<'a>(field: &'a RpField, var: Cons<'a>) -> (Cons<'a>, Tokens<'a, Swift<'a>>) {
        let name = field.name();
        (
            Cons::from(name),
            toks![var, "[", Cons::from(name).quoted(), "]"],
        )
    }

    /// Build a model struct for the given set of fields.
    fn model_type<'a, F>(
        &self,
        name: Tokens<'a, Swift<'a>>,
        comment: &'a [String],
        fields: F,
    ) -> Result<Tokens<'a, Swift<'a>>>
    where
        F: IntoIterator<Item = &'a Loc<RpField>>,
    {
        let fields = fields.into_iter().collect::<Vec<_>>();

        let mut tokens = Tokens::new();

        tokens.push(self.model_struct(
            name.clone(),
            comment,
            fields.iter().cloned(),
            self.options.codable,
        )?);

        if self.options.simple {
            tokens.push({
                let mut t = Tokens::new();

                t.push(toks!["public extension ", name.clone(), " {"]);

                t.push({
                    let mut t = Tokens::new();

                    // decode function
                    t.nested(decode(self, name.clone(), &fields)?);
                    t.nested(encode(self, &fields)?);

                    t.join_line_spacing()
                });

                t.push("}");
                t
            });
        }

        return Ok(tokens);

        fn decode<'a>(
            compiler: &Compiler,
            name: Tokens<'a, Swift<'a>>,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name.clone(),
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();
                let mut args = Tokens::new();

                if !fields.is_empty() {
                    t.push(toks!["let json = try decode_value(json as? [String: Any])"]);

                    for field in fields.iter().cloned() {
                        let ident = field.safe_ident();
                        t.push(compiler.decode_field(field, Compiler::type_index)?);
                        args.append(toks![ident.clone(), ": ", ident.clone()]);
                    }
                } else {
                    t.push(toks!["let _ = try decode_value(json as? [String: Any])"]);
                }

                t.push(toks!["return ", name.clone(), "(", args.join(", "), ")"]);
                t.join_line_spacing()
            });

            t.push("}");

            Ok(t)
        }

        fn encode<'a>(
            compiler: &Compiler,
            fields: &[&'a Loc<RpField>],
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push("func encode() throws -> [String: Any] {");
            t.nested({
                let mut t = Tokens::new();

                if !fields.is_empty() {
                    t.push("var json = [String: Any]()");
                    t.push({
                        let mut t = Tokens::new();

                        for field in fields.iter().cloned() {
                            t.push(compiler.encode_field(field, |value| {
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

    fn utils(&self) -> Result<FileSpec> {
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

        out.0.push(self.decode_name_func());
        out.0.push(self.decode_value_func());

        for ty in numerics.iter().chain(floats.iter()).cloned() {
            out.0.push(self.unbox_number_func(ty, &numerics, &floats));
        }

        for ty in simple.iter().cloned() {
            out.0.push(self.unbox_simple_func(ty));
        }

        out.0.push(self.decode_array_func());
        out.0.push(self.encode_array_func());
        out.0.push(self.decode_map_func());
        out.0.push(self.encode_map_func());

        Ok(out)
    }

    pub fn compile(&self) -> Result<()> {
        let mut files = self.populate_files()?;

        if self.options.simple {
            files.insert(self.utils_package(), self.utils()?);
        }

        self.write_files(files)
    }
}

impl<'el> PackageUtils for Compiler<'el> {}

impl<'el> PackageProcessor<'el> for Compiler<'el> {
    type Out = FileSpec<'el>;
    type DeclIter = trans::environment::DeclIter<'el>;

    fn ext(&self) -> &str {
        EXT
    }

    fn decl_iter(&self) -> Self::DeclIter {
        self.env.decl_iter()
    }

    fn handle(&self) -> &'el Handle {
        self.handle
    }

    fn processed_package(&self, package: &RpVersionedPackage) -> RpPackage {
        self.package(package)
    }

    fn default_process(&self, _out: &mut Self::Out, _: &RpName) -> Result<()> {
        Ok(())
    }

    fn process_type(&self, out: &mut Self::Out, body: &'el RpTypeBody) -> Result<()> {
        let name = self.convert_name(&body.name)?;

        out.0
            .extend(self.model_type(name, &body.comment, &body.fields)?);

        Ok(())
    }

    fn process_tuple(&self, out: &mut Self::Out, body: &'el RpTupleBody) -> Result<()> {
        let name = self.convert_name(&body.name)?;

        out.0
            .extend(self.model_tuple(name, &body.comment, &body.fields)?);

        Ok(())
    }

    fn process_enum(&self, out: &mut Self::Out, body: &'el RpEnumBody) -> Result<()> {
        let name = self.convert_name(&body.name)?;

        out.0.push({
            let mut t = Tokens::new();

            t.push(toks!["public enum ", name.clone(), " {"]);

            for variant in &body.variants {
                t.nested(toks!["case ", variant.local_name.as_str()]);
            }

            t.push("}");

            t
        });

        if self.options.simple {
            out.0.push({
                let mut t = Tokens::new();

                t.push(toks!["public extension ", name.clone(), " {"]);

                t.push({
                    let mut t = Tokens::new();

                    // decode function
                    t.nested(decode(body, name.clone())?);
                    t.nested(encode(body)?);

                    t.join_line_spacing()
                });

                t.push("}");
                t
            });
        }

        if self.options.codable {
            out.0.push(decodable(name.clone(), body)?);
            out.0.push(encodable(name.clone(), body)?);
        }

        return Ok(());

        fn decode<'a>(
            body: &'a RpEnumBody,
            name: Tokens<'a, Swift<'a>>,
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();

            t.push(toks![
                "static func decode(json: Any) throws -> ",
                name.clone(),
                " {"
            ]);
            t.nested({
                let mut t = Tokens::new();

                t.push(toks!["let json = try decode_value(json as? String)"]);

                t.push({
                    let mut t = Tokens::new();

                    t.push("switch json {");

                    for variant in &body.variants {
                        t.nested({
                            let mut t = Tokens::new();
                            t.push(toks!["case ", variant.ordinal().quoted(), ":"]);
                            t.nested(toks![
                                "return ",
                                name.clone(),
                                ".",
                                variant.local_name.as_str(),
                            ]);
                            t
                        });
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

            t.push("func encode() throws -> String {");
            t.nested({
                let mut t = Tokens::new();

                t.push("switch self {");

                for variant in &body.variants {
                    t.nested({
                        let mut t = Tokens::new();
                        t.push(toks!["case .", variant.local_name.as_str(), ":"]);
                        t.nested(toks!["return ", variant.ordinal().quoted()]);
                        t
                    });
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
            t.push("}");

            Ok(t)
        }

        fn decodable<'a>(
            name: Tokens<'a, Swift<'a>>,
            body: &'a RpEnumBody,
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Decodable {"]);
            t.nested(init(body)?);
            t.push("}");

            return Ok(t);

            fn init<'a>(body: &'a RpEnumBody) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public init(from decoder: Decoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("let value = try decoder.singleValueContainer()");

                    t.push({
                        let mut t = Tokens::new();

                        t.push("switch try value.decode(String.self) {");

                        for variant in &body.variants {
                            t.push({
                                let mut t = Tokens::new();
                                t.push(toks!["case ", variant.ordinal().quoted(), ":"]);
                                t.nested(toks!["self = .", variant.local_name.as_str()]);
                                t
                            });
                        }

                        t.push({
                            let mut t = Tokens::new();

                            t.push("default:");
                            t.nested(toks![
                                "let context = DecodingError.Context(codingPath: [], \
                                 debugDescription: ",
                                "enum variant".quoted(),
                                ")"
                            ]);
                            t.nested("throw DecodingError.dataCorrupted(context)");

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
        }

        fn encodable<'a>(
            name: Tokens<'a, Swift<'a>>,
            body: &'a RpEnumBody,
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Encodable {"]);

            t.push({
                let mut t = Tokens::new();
                t.nested(encode(body)?);
                t.join_line_spacing()
            });

            t.push("}");

            return Ok(t);

            fn encode<'a>(body: &'a RpEnumBody) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public func encode(to encoder: Encoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("var value = encoder.singleValueContainer()");

                    t.push({
                        let mut t = Tokens::new();

                        t.push("switch self {");

                        for variant in &body.variants {
                            t.push({
                                let mut t = Tokens::new();
                                t.push(toks!["case .", variant.local_name.as_str(), ":"]);
                                t.nested(toks![
                                    "try value.encode(",
                                    variant.ordinal().quoted(),
                                    ")"
                                ]);
                                t
                            });
                        }

                        t.push("}");

                        t
                    });

                    t.join_line_spacing()
                });

                t.push("}");

                Ok(t)
            }
        }
    }

    fn process_interface(&self, out: &mut Self::Out, body: &'el RpInterfaceBody) -> Result<()> {
        let name = self.convert_name(&body.name)?;

        out.0.push({
            let mut t = Tokens::new();

            t.push_unless_empty(Comments(&body.comment));
            t.push(toks!["public enum ", name.clone(), " {"]);

            for sub_type in body.sub_types.values() {
                let name = self.convert_name(&sub_type.name)?;
                let local_name = sub_type.local_name.as_str();
                t.nested(toks!["case ", local_name, "(", name.clone(), ")"]);
            }

            if self.options.codable {
                match body.sub_type_strategy {
                    RpSubTypeStrategy::Tagged { ref tag, .. } => {
                        t.nested({
                            let mut t = Tokens::new();

                            t.push({
                                let mut t = Tokens::new();

                                t.push("enum CodingKeys: String, CodingKey {");
                                t.nested(toks!["case tag = ", tag.as_str().quoted()]);
                                t.push("}");

                                t
                            });

                            t
                        });
                    }
                }
            }

            t.push("}");
            t
        });

        if self.options.codable {
            out.0.push(decodable(self, name.clone(), body)?);
            out.0.push(encodable(name.clone(), body)?);
        }

        if self.options.simple {
            out.0.push({
                let mut t = Tokens::new();

                t.push_unless_empty(Comments(&body.comment));
                t.push(toks!["public extension ", name.clone(), " {"]);

                t.push({
                    let mut t = Tokens::new();

                    match body.sub_type_strategy {
                        RpSubTypeStrategy::Tagged { ref tag, .. } => {
                            // decode function
                            t.nested(self.decode_tag(name.clone(), tag.as_str(), &body.sub_types)?);
                            t.nested(self.encode_tag(tag.as_str(), &body.sub_types)?);
                        }
                    }

                    t.join_line_spacing()
                });

                t.push("}");
                t
            });
        }

        for sub_type in body.sub_types.values() {
            let sub_type_name = self.convert_name(&sub_type.name)?;

            let fields = body.fields.iter().chain(sub_type.fields.iter());

            out.0
                .push(self.model_type(sub_type_name, &sub_type.comment, fields)?);
        }

        return Ok(());

        fn decodable<'a>(
            compiler: &Compiler,
            name: Tokens<'a, Swift<'a>>,
            body: &'a RpInterfaceBody,
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Decodable {"]);

            t.push({
                let mut t = Tokens::new();

                match body.sub_type_strategy {
                    RpSubTypeStrategy::Tagged { ref tag, .. } => {
                        t.nested(init(compiler, body, tag)?);
                    }
                }

                t.join_line_spacing()
            });

            t.push("}");

            return Ok(t);

            fn init<'a>(
                compiler: &Compiler,
                body: &'a RpInterfaceBody,
                tag: &'a str,
            ) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public init(from decoder: Decoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("let values = try decoder.container(keyedBy: CodingKeys.self)");

                    t.push({
                        let mut t = Tokens::new();

                        t.push("switch try values.decode(String.self, forKey: .tag) {");

                        for sub_type in body.sub_types.values() {
                            t.push({
                                let mut t = Tokens::new();

                                let name = compiler.convert_name(&sub_type.name)?;
                                let n = sub_type.local_name.as_str();

                                let d = toks![name, "(from: decoder)"];
                                let d = toks![".", n, "(", d, ")"];

                                t.push(toks!["case ", sub_type.name().quoted(), ":"]);
                                t.nested(toks!["self = try ", d]);

                                t
                            });
                        }

                        t.push({
                            let mut t = Tokens::new();

                            t.push("default:");
                            t.nested(toks![
                                "let context = DecodingError.Context(codingPath: [], \
                                 debugDescription: ",
                                tag.quoted(),
                                ")"
                            ]);
                            t.nested("throw DecodingError.dataCorrupted(context)");

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
        }

        fn encodable<'a>(
            name: Tokens<'a, Swift<'a>>,
            body: &'a RpInterfaceBody,
        ) -> Result<Tokens<'a, Swift<'a>>> {
            let mut t = Tokens::new();
            t.push(toks!["extension ", name, ": Encodable {"]);

            t.push({
                let mut t = Tokens::new();
                t.nested(encode(body)?);
                t.join_line_spacing()
            });

            t.push("}");

            return Ok(t);

            fn encode<'a>(body: &'a RpInterfaceBody) -> Result<Tokens<'a, Swift<'a>>> {
                let mut t = Tokens::new();

                t.push("public func encode(to encoder: Encoder) throws {");

                t.nested({
                    let mut t = Tokens::new();

                    t.push("var values = encoder.container(keyedBy: CodingKeys.self)");

                    t.push({
                        let mut t = Tokens::new();

                        t.push("switch self {");

                        for sub_type in body.sub_types.values() {
                            let n = sub_type.local_name.as_str();
                            let name = sub_type.name();
                            let ty = toks!["try values.encode(", name.quoted(), ", forKey: .tag)"];

                            t.push({
                                let mut t = Tokens::new();
                                t.push(toks!["case .", n, "(let d):"]);
                                t.nested(ty);
                                t.nested(toks!["try d.encode(to: encoder)"]);
                                t
                            });
                        }

                        t.push("}");

                        t
                    });

                    t.join_line_spacing()
                });

                t.push("}");

                Ok(t)
            }
        }
    }
}
