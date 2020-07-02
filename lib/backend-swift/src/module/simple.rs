//! gRPC module for Rust.

use crate::codegen;
use crate::compiler::Comments;
use crate::flavored::{Field, Name, RpEnumBody, RpInterfaceBody, RpPackage, RpSubType};
use crate::Options;
use backend::Initializer;
use core::errors::Result;
use core::{self, Spanned};
use genco::prelude::*;
use genco::tokens::ItemStr;
use std::rc::Rc;

static NUMERICS: [&str; 6] = ["Int", "UInt", "Int32", "Int64", "UInt32", "UInt64"];
static FLOATS: [&str; 2] = ["Float", "Double"];
static SIMPLE: [&str; 2] = ["String", "Bool"];

pub(crate) struct Module {}

impl Module {
    pub(crate) fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, opt: &mut Self::Options) -> Result<()> {
        let codegen = Rc::new(Codegen::new());
        opt.gen.type_added.push(codegen.clone());
        opt.gen.tuple_added.push(codegen.clone());
        opt.gen.enum_added.push(codegen.clone());
        opt.gen.interface_added.push(codegen.clone());
        opt.gen.package_added.push(codegen.clone());
        Ok(())
    }
}

struct Codegen {}

impl Codegen {
    pub(crate) fn new() -> Codegen {
        Self {}
    }

    // Setup a field initializer.
    pub(crate) fn encode_field<'f, A: 'f>(
        &'f self,
        field: &'f Field,
        append: A,
    ) -> impl FormatInto<Swift> + 'f
    where
        A: Fn(swift::Tokens) -> swift::Tokens,
    {
        let ident = field.safe_ident();
        let name = field.name();

        quote_fn! {
            #(if field.is_optional() {
                if let value = self.#ident {
                    #(append(field.ty.encode_value(name, quote!(value))))
                }
            } else {
                #(append(field.ty.encode_value(name, quote!(self.#ident))))
            })
        }
    }

    fn decode_field<'f, I: 'f>(&self, field: &'f Field, index: I) -> impl FormatInto<Swift> + 'f
    where
        I: Fn(&'f Field, ItemStr) -> (ItemStr, swift::Tokens),
    {
        let ident = field.safe_ident();
        let f_ident = Rc::new(format!("f_{}", &field.ident));

        let (name, index) = index(field, ItemStr::from("json"));

        quote_fn! {
            #(if field.is_optional() {
                var #ident: #(field.field_type()) = Optional.none

                if let value = #index {
                    #ident = Optional.some(#(field.ty.decode_value(name, quote!(value))))
                }
            } else {
                guard let #(&*f_ident) = #index else {
                    throw SerializationError.missing(#(quoted(name.clone())))
                }

                let #ident = #(field.ty.decode_value(name, quote!(#(&*f_ident))))
            })
        }
    }

    fn type_index(field: &Field, var: ItemStr) -> (ItemStr, swift::Tokens) {
        (
            ItemStr::from(field.name()),
            quote!(#var[#(quoted(field.name()))]),
        )
    }

    fn utils_package(&self) -> RpPackage {
        RpPackage::parse("reproto_simple")
    }

    fn utils(&self) -> swift::Tokens {
        return quote! {
            enum SerializationError: Error {
                case missing(String)
                case invalid(String)
                case bad_value
            }

            #(decode_name_func())

            #(decode_value_func())

            #(for ty in NUMERICS.iter().chain(FLOATS.iter()).copied() join (#<line>) {
                #(unbox_number_func(ty, &NUMERICS, &FLOATS))
            })

            #(for ty in SIMPLE.iter().copied() join (#<line>) {
                #(unbox_simple_func(ty))
            })

            #(decode_array_func())

            #(encode_array_func())

            #(decode_map_func())

            #(encode_map_func())
        };

        /// Build a simple unboxing functions.
        fn unbox_simple_func(ty: &str) -> impl FormatInto<Swift> + '_ {
            quote_fn! {
                func unbox(_ value: Any, as type: #ty.Type) -> #ty? {
                    return value as? #ty
                }
            }
        }

        /// Build an integer unboxing function.
        ///
        /// This is more complicated since it needs to handle numeric conversions.
        fn unbox_number_func<'f>(
            ty: &'f str,
            numerics: &'f [&str],
            floats: &'f [&str],
        ) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                func unbox(_ value: Any, as type: #ty.Type) -> #ty? {
                    switch value {
                    #(ref o => for ck in numerics.iter().copied() {
                        if ty == ck {
                            continue;
                        }

                        quote_in! { *o =>
                            case let n as #ck:
                                return #ty(exactly: n)
                        }
                    })
                    #(ref o => for ck in floats.iter().copied() {
                        if ty == ck {
                            continue;
                        }

                        quote_in! { *o =>
                            case let n as #ck:
                                return #ty(n)
                        }
                    })
                    default:
                        return value as? #ty
                    }
                }
            }
        }

        /// Build an array decoding function.
        fn decode_array_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func decode_array<T>(_ value: Any, name: String, inner: (Any) throws -> T) throws -> [T] {
                    let array = try decode_name(value as? [Any], name: name)
                    var out = [T]()

                    for item in array {
                        out.append(try inner(item))
                    }

                    return out
                }
            }
        }

        /// Build an array encoding function.
        fn encode_array_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func encode_array<T>(_ array: [T], name: String, inner: (T) throws -> Any) throws -> [Any] {
                    var out = [Any]()

                    for item in array {
                        out.append(try inner(item))
                    }

                    return out
                }
            }
        }

        /// Build an array decoding function.
        fn decode_map_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func decode_map<T>(_ map: Any, name: String, value: (Any) throws -> T) throws ->  [String: T] {
                    let map = try decode_name(map as? [String: Any], name: name)
                    var out = [String: T]()

                    for (k, v) in map {
                        out[k] = try value(v)
                    }

                    return out
                }
            }
        }

        /// Build an array encoding function.
        fn encode_map_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func encode_map<T>(_ map: [String: T], name: String, value: (T) throws -> Any) throws -> [String: Any] {
                    var out = [String: Any]()

                    for (k, v) in map {
                        out[k] = try value(v)
                    }

                    return out
                }
            }
        }

        /// Build a generic decoding function with named errors.
        fn decode_name_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func decode_name<T>(_ unbox: T?, name string: String) throws -> T {
                    guard let value = unbox else {
                        throw SerializationError.invalid(string)
                    }

                    return value
                }
            }
        }

        /// Build a generic decoding function.
        fn decode_value_func() -> impl FormatInto<Swift> {
            quote_fn! {
                func decode_value<T>(_ value: T?) throws -> T {
                    guard let value = value else {
                        throw SerializationError.bad_value
                    }

                    return value
                }
            }
        }
    }
}

impl codegen::type_added::Codegen for Codegen {
    fn generate(&self, e: codegen::type_added::Args<'_>) {
        let codegen::type_added::Args {
            container,
            name,
            fields,
        } = e;

        container.push(quote! {
            public extension #name {
                #(decode(self, name, fields))

                #(encode(self, fields))
            }
        });

        fn decode<'f>(
            codegen: &'f Codegen,
            name: &'f Name,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            let mut args = Vec::new();

            quote_fn! {
                static func decode(json: Any) throws -> #name {
                    #(if !fields.is_empty() {
                        let json = try decode_value(json as? [String: Any])

                        #(for field in fields join (#<line>) {
                            #(codegen.decode_field(field, Codegen::type_index))
                            #(ref _ {
                                let ident = field.safe_ident();
                                args.push(quote!(#ident: #ident));
                            })
                        })
                    } else {
                        let _ = try decode_value(json as? [String: Any])
                        #<line>
                    })
                    return #name(#(for a in args join (, ) => #a))
                }
            }
        }

        fn encode<'f>(
            codegen: &'f Codegen,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                func encode() throws -> [String: Any] {
                    #(if !fields.is_empty() {
                        var json = [String: Any]()

                        #(for field in fields join (#<line>) {
                            #(codegen.encode_field(field, |value| {
                                quote!(json[#(quoted(field.name()))] = #value)
                            }))
                        })

                        return json
                    } else {
                        return [String: Any]()
                    })
                }
            }
        }
    }
}

impl codegen::tuple_added::Codegen for Codegen {
    fn generate(&self, e: codegen::tuple_added::Args<'_>) {
        let codegen::tuple_added::Args {
            container,
            name,
            fields,
        } = e;

        container.push(quote! {
            public extension #name {
                #(decode(self, name, fields))

                #(encode(self, fields))
            }
        });

        fn decode<'f>(
            codegen: &'f Codegen,
            name: &'f Name,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            let mut args = Vec::new();

            quote_fn! {
                static func decode(json: Any) throws -> #name {
                    let json = try decode_value(json as? [Any])

                    #(for (i, field) in fields.iter().enumerate() join (#<line>) {
                        #(codegen.decode_field(field, |_, var| {
                            (
                                ItemStr::from(format!("[{}]", i)),
                                quote!(Optional.some(#var[#i]))
                            )
                        }))
                        #(ref _ {
                            let ident = field.safe_ident();
                            args.push(quote!(#ident: #ident));
                        })
                    })

                    return #name(#(for a in args join (, ) => #a))
                }
            }
        }

        fn encode<'f>(
            codegen: &'f Codegen,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                func encode() throws -> [Any] {
                    var json = [Any]()

                    #(for field in fields join (#<push>) {
                        #(codegen
                            .encode_field(field, |value| quote!(json.append(#value))))
                    })

                    return json
                }
            }
        }
    }
}

impl codegen::enum_added::Codegen for Codegen {
    fn generate(&self, e: codegen::enum_added::Args<'_>) {
        let codegen::enum_added::Args {
            container,
            name,
            body,
            ..
        } = e;

        container.push(quote! {
            public extension #name {
                #(decode(body, name))

                #(encode(body))
            }
        });

        fn decode<'f>(body: &'f RpEnumBody, name: &'f Name) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                static func decode(json: Any) throws -> #name {
                    let json = try decode_value(json)
                    let value = try decode_value(unbox(json, as: #(&body.enum_type).self))

                    switch value {
                    #(match &body.variants {
                        core::RpVariants::String { variants } => {
                            #(for v in variants {
                                case #(quoted(v.value.to_string())):
                                    return #name.#(v.ident())
                            })
                        }
                        core::RpVariants::Number { variants } => {
                            #(for v in variants {
                                case #(v.value.to_string()):
                                    return #name.#(v.ident())
                            })
                        }
                    })
                    default:
                        throw SerializationError.bad_value
                    }
                }
            }
        }

        fn encode(body: &RpEnumBody) -> impl FormatInto<Swift> + '_ {
            quote_fn! {
                func encode() throws -> #(&body.enum_type) {
                    switch self {
                    #(match &body.variants {
                        core::RpVariants::String { variants } => {
                            #(for v in variants join (#<push>) {
                                case .#(v.ident()):
                                    return #(quoted(v.value.to_string()))
                            })
                        }
                        core::RpVariants::Number { variants } => {
                            #(for v in variants join (#<push>) {
                                case .#(v.ident()):
                                    return #(v.value.to_string())
                            })
                        }
                    })
                    }
                }
            }
        }
    }
}

impl codegen::interface_added::Codegen for Codegen {
    fn generate(&self, e: codegen::interface_added::Args<'_>) {
        let codegen::interface_added::Args {
            container,
            name,
            body,
            ..
        } = e;

        container.push(quote! {
            #(Comments(&body.comment))
            public extension #name {
                #(match &body.sub_type_strategy {
                    core::RpSubTypeStrategy::Tagged { tag, .. } => {
                        #(decode_tag(name, tag.as_str(), &body.sub_types))
                        #(encode_tag(tag.as_str(), &body.sub_types))
                    }
                    core::RpSubTypeStrategy::Untagged => {
                        #(decode_untagged(name, body))
                        #(encode_untagged(&body.sub_types))
                    }
                })
            }
        });

        /// Build a method to decode a tagged interface.
        fn decode_tag<'f, 'el: 'f, S: 'f>(
            name: &'f Name,
            tag: &'f str,
            sub_types: S,
        ) -> impl FormatInto<Swift> + 'f
        where
            S: IntoIterator<Item = &'el Spanned<RpSubType>>,
        {
            quote_fn! {
                static func decode(json: Any) throws -> #name {
                    let json = try decode_value(json as? [String: Any])
                    let type = try decode_name(json[#(quoted(tag))] as? String, name: #(quoted(tag)))

                    switch type {
                    #(for sub_type in sub_types.into_iter() {
                        case #(quoted(sub_type.name())):
                            let v = try #(&sub_type.name).decode(json: json)
                            return #name.#(&sub_type.ident)(v)
                    })
                    default:
                        throw SerializationError.invalid(type)
                    }
                }
            }
        }

        /// Build a method to decode a tagged interface.
        fn encode_tag<'f, 'el: 'f, S: 'f>(tag: &'f str, sub_types: S) -> impl FormatInto<Swift> + 'f
        where
            S: IntoIterator<Item = &'el Spanned<RpSubType>>,
        {
            quote_fn! {
                func encode() throws -> [String: Any] {
                    switch self {
                    #(for sub_type in sub_types.into_iter() {
                        case .#(&sub_type.ident)(let s):
                            var json = try s.encode()
                            json[#(quoted(tag))] = #(quoted(sub_type.name()))
                            return json
                    })
                    }
                }
            }
        }

        /// Build a method to decode a tagged interface.
        fn decode_untagged<'f>(
            name: &'f Name,
            body: &'f RpInterfaceBody,
        ) -> impl FormatInto<Swift> + 'f {
            let optional = quoted_tags(body.fields.iter().filter(|f| f.is_optional()));

            return quote_fn! {
                static func decode(json: Any) throws -> #name {
                    let json = try decode_value(json as? [String: Any])

                    let keys = Set(json.keys).subtracting(#optional)
                    #(ref o => for sub_type in &body.sub_types {
                        let tags = quoted_tags(sub_type.fields.iter().filter(|f| f.is_optional()));
                        let req = quoted_tags(
                            body.fields
                                .iter()
                                .chain(sub_type.fields.iter())
                                .filter(|f| f.is_required()),
                        );

                        quote_in! { *o =>
                            #<line>
                            if keys.subtracting(#tags) == #req {
                                return #name.#(&sub_type.ident)(try #(&sub_type.name).decode(json: json))
                            }
                        }
                    })

                    throw SerializationError.invalid("no legal field combinations")
                }
            };

            fn quoted_tags<'a, I>(fields: I) -> swift::Tokens
            where
                I: IntoIterator<Item = &'a Spanned<Field>>,
            {
                let mut it = fields.into_iter().peekable();

                let mut tags = Tokens::new();

                tags.append("[");

                while let Some(field) = it.next() {
                    tags.append(quoted(field.name()));

                    if it.peek().is_some() {
                        tags.append(", ");
                    }
                }

                tags.append("]");
                tags
            }
        }

        /// Build a method to decode a tagged interface.
        fn encode_untagged<'f, 'el: 'f, S: 'f>(sub_types: S) -> impl FormatInto<Swift> + 'f
        where
            S: IntoIterator<Item = &'el Spanned<RpSubType>>,
        {
            quote_fn! {
                func encode() throws -> [String: Any] {
                    switch self {
                    #(for sub_type in sub_types join (#<push>) {
                        case .#(&sub_type.ident)(let s):
                            return try s.encode()
                    })
                    }
                }
            }
        }
    }
}

impl codegen::package_added::Codegen for Codegen {
    fn generate(&self, e: codegen::package_added::Args<'_>) {
        e.files.push((self.utils_package(), self.utils()));
    }
}
