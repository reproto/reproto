//! gRPC module for Rust.

use crate::codegen;
use crate::flavored::{Field, Name, RpEnumBody, RpInterfaceBody, RpPackage, Type};
use crate::Options;
use backend::Initializer;
use core::errors::Result;
use core::Spanned;
use genco::prelude::*;
use std::collections::BTreeSet;
use std::rc::Rc;

static PRIMITIVES: [&str; 10] = [
    "Bool", "Int", "UInt", "Int32", "Int64", "UInt32", "UInt64", "Float", "Double", "String",
];

pub(crate) struct Module {}

impl Module {
    pub(crate) fn new() -> Module {
        Module {}
    }
}

impl Initializer for Module {
    type Options = Options;

    fn initialize(&self, opt: &mut Self::Options) -> Result<()> {
        let codegen = Rc::new(Codegen);
        opt.struct_model_extends.push(quote!(Codable));
        opt.gen.tuple_added.push(codegen.clone());
        opt.gen.struct_model_added.push(codegen.clone());
        opt.gen.enum_added.push(codegen.clone());
        opt.gen.interface_added.push(codegen.clone());
        opt.gen.interface_model_added.push(codegen.clone());
        opt.gen.package_added.push(codegen.clone());
        opt.any_type.push(("codable", Type::local("AnyCodable")));
        Ok(())
    }
}

struct Codegen;

impl Codegen {
    fn utils_package(&self) -> RpPackage {
        RpPackage::parse("reproto_codable")
    }

    fn utils(&self) -> swift::Tokens {
        let mut out = swift::Tokens::default();
        any_codable(&mut out);
        return out;

        fn any_codable(t: &mut swift::Tokens) {
            quote_in! { *t =>
                class AnyCodable: Codable {
                    public let value: Any

                    #(init())

                    #(encode())

                    #(decoding_error())

                    #(encoding_error())

                    #(decode_single(&PRIMITIVES))

                    #(decode_unkeyed(&PRIMITIVES))

                    #(decode_keyed(&PRIMITIVES))

                    #(decode_array())

                    #(decode_dictionary())

                    #(encode_single(&PRIMITIVES))

                    #(encode_unkeyed(&PRIMITIVES))

                    #(encode_keyed(&PRIMITIVES))
                }

                #(any_coding_key())

                #(any_null())
            };

            return;

            fn init() -> impl FormatInto<Swift> {
                quote_fn! {
                    public required init(from decoder: Decoder) throws {
                        if var array = try? decoder.unkeyedContainer() {
                            self.value = try AnyCodable.decodeArray(from: &array)
                            return
                        }

                        if var c = try? decoder.container(keyedBy: AnyCodingKey.self) {
                            self.value = try AnyCodable.decodeDictionary(from: &c)
                            return
                        }

                        let c = try decoder.singleValueContainer()
                        self.value = try AnyCodable.decode(from: c)
                    }
                }
            }

            fn encode() -> impl FormatInto<Swift> {
                quote_fn! {
                    public func encode(to encoder: Encoder) throws {
                        if let arr = self.value as? [Any] {
                            var c = encoder.unkeyedContainer()
                            try AnyCodable.encode(to: &c, array: arr)
                            return
                        }

                        if let dict = self.value as? [String: Any] {
                            var c = encoder.container(keyedBy: AnyCodingKey.self)
                            try AnyCodable.encode(to: &c, dictionary: dict)
                            return
                        }

                        var c = encoder.singleValueContainer()
                        try AnyCodable.encode(to: &c, value: self.value)
                    }
                }
            }

            fn decoding_error() -> impl FormatInto<Swift> {
                quote_fn! {
                    static func decodingError(forCodingPath codingPath: [CodingKey]) -> DecodingError {
                        let context = DecodingError.Context(
                            codingPath: codingPath,
                            debugDescription: "Cannot decode AnyCodable"
                        )

                        return DecodingError.typeMismatch(AnyCodable.self, context)
                    }
                }
            }

            fn encoding_error() -> impl FormatInto<Swift> {
                quote_fn! {
                    static func encodingError(forValue value: Any, codingPath: [CodingKey]) -> EncodingError {
                        let context = EncodingError.Context(
                            codingPath: codingPath,
                            debugDescription: "Cannot encode AnyCodable"
                        )

                        return EncodingError.invalidValue(value, context)
                    }
                }
            }

            fn decode_single<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func decode(from c: SingleValueDecodingContainer) throws -> Any {
                        #(for p in primitives.iter().copied() join (#<line>) {
                            if let value = try? c.decode(#p.self) {
                                return value
                            }
                        })

                        if c.decodeNil() {
                            return AnyNull()
                        }

                        throw decodingError(forCodingPath: c.codingPath)
                    }
                }
            }

            fn decode_unkeyed<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func decode(from c: inout UnkeyedDecodingContainer) throws -> Any {
                        #(for p in primitives.iter().copied() join (#<line>) {
                            if let value = try? c.decode(#p.self) {
                                return value
                            }
                        })

                        if let value = try? c.decodeNil() {
                            if value {
                                return AnyNull()
                            }
                        }

                        if var c = try? c.nestedUnkeyedContainer() {
                            return try decodeArray(from: &c)
                        }

                        if var c = try? c.nestedContainer(keyedBy: AnyCodingKey.self) {
                            return try decodeDictionary(from: &c)
                        }

                        throw decodingError(forCodingPath: c.codingPath)
                    }
                }
            }

            fn decode_keyed<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func decode(from c: inout KeyedDecodingContainer<AnyCodingKey>, forKey key: AnyCodingKey) throws -> Any {
                        #(for p in primitives.iter().copied() join (#<line>) {
                            if let value = try? c.decode(#p.self, forKey: key) {
                                return value
                            }
                        })

                        if let value = try? c.decodeNil(forKey: key) {
                            if value {
                                return AnyNull()
                            }
                        }

                        if var c = try? c.nestedUnkeyedContainer(forKey: key) {
                            return try decodeArray(from: &c)
                        }

                        if var c = try? c.nestedContainer(keyedBy: AnyCodingKey.self, forKey: key) {
                            return try decodeDictionary(from: &c)
                        }

                        throw decodingError(forCodingPath: c.codingPath)
                    }
                }
            }

            fn decode_array() -> impl FormatInto<Swift> {
                quote_fn! {
                    static func decodeArray(from c: inout UnkeyedDecodingContainer) throws ->  [Any] {
                        var array: [Any] = []

                        while !c.isAtEnd {
                            array.append(try decode(from: &c))
                        }

                        return array
                    }
                }
            }

            fn decode_dictionary() -> impl FormatInto<Swift> {
                quote_fn! {
                    static func decodeDictionary(from c: inout KeyedDecodingContainer<AnyCodingKey>) throws -> [String: Any] {
                        var dict = [String: Any]()

                        for key in c.allKeys {
                            dict[key.stringValue] = try decode(from: &c, forKey: key)
                        }

                        return dict
                    }
                }
            }

            fn encode_single<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func encode(to c: inout SingleValueEncodingContainer, value: Any) throws {
                        switch value {
                        #(for p in primitives.iter().copied() join (#<push>) {
                            case let value as #p:
                                try c.encode(value)
                        })
                        case _ as AnyNull:
                            try c.encodeNil()
                        default:
                            throw encodingError(forValue: value, codingPath: c.codingPath)
                        }
                    }
                }
            }

            fn encode_unkeyed<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func encode(to c: inout UnkeyedEncodingContainer, array: [Any]) throws {
                        for value in array {
                            switch value {
                            #(for p in primitives.iter().copied() join (#<push>) {
                                case let value as #p:
                                    try c.encode(value)
                            })
                            case let value as [Any]:
                                var c = c.nestedUnkeyedContainer()
                                try encode(to: &c, array: value)
                            case let value as [String: Any]:
                                var c = c.nestedContainer(keyedBy: AnyCodingKey.self)
                                try encode(to: &c, dictionary: value)
                            case _ as AnyNull:
                                try c.encodeNil()
                            default:
                                throw encodingError(forValue: value, codingPath: c.codingPath)
                            }
                        }
                    }
                }
            }

            fn encode_keyed<'f>(primitives: &'f [&str]) -> impl FormatInto<Swift> + 'f {
                quote_fn! {
                    static func encode(to c: inout KeyedEncodingContainer<AnyCodingKey>, dictionary: [String: Any]) throws {
                        for (key, value) in dictionary {
                            let key = AnyCodingKey(stringValue: key)!

                            switch value {
                            #(for p in primitives.iter().copied() join (#<push>) {
                                case let value as #p:
                                    try c.encode(value, forKey: key)
                            })
                            case let value as [Any]:
                                var c = c.nestedUnkeyedContainer(forKey: key)
                                try encode(to: &c, array: value)
                            case let value as [String: Any]:
                                var c = c.nestedContainer(keyedBy: AnyCodingKey.self, forKey: key)
                                try encode(to: &c, dictionary: value)
                            case _ as AnyNull:
                                try c.encodeNil(forKey: key)
                            default:
                                throw encodingError(forValue: value, codingPath: c.codingPath)
                            }
                        }
                    }
                }
            }

            fn any_coding_key() -> impl FormatInto<Swift> {
                quote_fn! {
                    class AnyCodingKey: CodingKey {
                        let key: String

                        required init?(intValue: Int) {
                            return nil
                        }

                        required init?(stringValue: String) {
                            key = stringValue
                        }

                        var intValue: Int? {
                            return nil
                        }

                        var stringValue: String {
                            return key
                        }
                    }
                }
            }

            fn any_null() -> impl FormatInto<Swift> {
                quote_fn! {
                    class AnyNull: Codable {
                        public init() {
                        }

                        public required init(from decoder: Decoder) throws {
                            let c = try decoder.singleValueContainer()

                            if !c.decodeNil() {
                                let context = DecodingError.Context(
                                    codingPath: decoder.codingPath,
                                    debugDescription: "Wrong type for AnyNull"
                                )
                                throw DecodingError.typeMismatch(AnyNull.self, context)
                            }
                        }

                        public func encode(to encoder: Encoder) throws {
                            var c = encoder.singleValueContainer()
                            try c.encodeNil()
                        }
                    }
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
            ..
        } = e;

        container.push(quote! {
            #(decodable(name, fields))

            #(encodable(name, fields))
        });

        fn decodable<'f>(
            name: &'f Name,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                extension #name: Decodable {
                    public init(from decoder: Decoder) throws {
                        var values = try decoder.unkeyedContainer()

                        #(for field in fields join (#<push>) {
                            #(if field.is_optional() {
                                self.#(field.safe_ident()) = try values.decodeIfPresent(#(&field.ty).self)
                            } else {
                                self.#(field.safe_ident()) = try values.decode(#(&field.ty).self)
                            })
                        })
                    }
                }
            }
        }

        fn encodable<'f>(
            name: &'f Name,
            fields: &'f [Spanned<Field>],
        ) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                extension #name: Encodable {
                    public func encode(to encoder: Encoder) throws {
                        var values = encoder.unkeyedContainer()

                        #(for field in fields join (#<push>) {
                            #(if field.is_optional() {
                                if let #(field.safe_ident()) = self.#(field.safe_ident()) {
                                    try values.encode(#(field.safe_ident()))
                                }
                            } else {
                                try values.encode(self.#(field.safe_ident()))
                            })
                        })
                    }
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
            #(decodable(name, body))

            #(encodable(name, body))
        });

        fn decodable<'f>(name: &'f Name, body: &'f RpEnumBody) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                extension #name: Decodable {
                    public init(from decoder: Decoder) throws {
                        let value = try decoder.singleValueContainer()

                        switch try value.decode(#(&body.enum_type).self) {
                        #(match &body.variants {
                            core::RpVariants::String { variants } => {
                                #(for v in variants join (#<push>) {
                                    case #(quoted(v.value.to_string())):
                                        self = .#(v.ident())
                                })
                            }
                            core::RpVariants::Number { variants } => {
                                #(for v in variants join (#<push>) {
                                    case #(v.value.to_string()):
                                        self = .#(v.ident())
                                })
                            }
                        })
                        default:
                            let context = DecodingError.Context(
                                codingPath: decoder.codingPath,
                                debugDescription: "enum variant"
                            )

                            throw DecodingError.dataCorrupted(context)
                        }
                    }
                }
            }
        }

        fn encodable<'f>(name: &'f Name, body: &'f RpEnumBody) -> impl FormatInto<Swift> + 'f {
            quote_fn! {
                extension #name: Encodable {
                    public func encode(to encoder: Encoder) throws {
                        var value = encoder.singleValueContainer()

                        switch self {
                        #(match &body.variants {
                            core::RpVariants::String { variants } => {
                                #(for v in variants join (#<push>) {
                                    case .#(v.ident()):
                                        try value.encode(#(quoted(v.value.to_string())))
                                })
                            }
                            core::RpVariants::Number { variants } => {
                                #(for v in variants {
                                    case .#(v.ident()):
                                        try value.encode(#(v.value.to_string()))
                                })
                            }
                        })
                        }
                    }
                }
            }
        }
    }
}

impl codegen::struct_model_added::Codegen for Codegen {
    fn generate(&self, e: codegen::struct_model_added::Args<'_>) {
        let codegen::struct_model_added::Args {
            container, fields, ..
        } = e;

        if !fields.is_empty() {
            container.push(quote! {
                enum CodingKeys: String, CodingKey {
                    #(for field in fields join (#<push>) {
                        case #(field.safe_ident()) = #(quoted(field.name()))
                    })
                }
            });
        }
    }
}

impl codegen::interface_added::Codegen for Codegen {
    fn generate(&self, e: codegen::interface_added::Args) {
        let codegen::interface_added::Args {
            container,
            name,
            body,
            ..
        } = e;

        container.push(quote! {
            #(decodable(name, body))

            #(encodable(name, body))
        });

        fn decodable<'f>(name: &'f Name, body: &'f RpInterfaceBody) -> impl FormatInto<Swift> + 'f {
            return quote_fn! {
                extension #name: Decodable {
                    #(match &body.sub_type_strategy {
                        core::RpSubTypeStrategy::Tagged { tag, .. } => {
                            #(ref o => tagged_init(o, body, tag))
                        }
                        core::RpSubTypeStrategy::Untagged => {
                            #(ref o => untagged_init(o, body))
                        }
                    })
                }
            };

            fn tagged_init(t: &mut swift::Tokens, body: &RpInterfaceBody, tag: &str) {
                quote_in! { *t =>
                    public init(from decoder: Decoder) throws {
                        let values = try decoder.container(keyedBy: CodingKeys.self)

                        switch try values.decode(String.self, forKey: .tag) {
                        #(for sub_type in &body.sub_types join (#<push>) {
                            case #(quoted(sub_type.name())):
                                self = try .#(sub_type.ident.as_str())(#(&sub_type.name)(from: decoder))
                        })
                        default:
                            let context = DecodingError.Context(codingPath: [], debugDescription: #(quoted(tag)))
                            throw DecodingError.dataCorrupted(context)
                        }
                    }
                }
            }

            fn untagged_init(t: &mut swift::Tokens, body: &RpInterfaceBody) {
                quote_in! { *t =>
                    public init(from decoder: Decoder) throws {
                        #(ref o => for sub_type in &body.sub_types {
                            let keys = quote!(Set(try decoder.container(keyedBy: #(&sub_type.ident)Keys.self).allKeys));

                            let mut expected = Vec::new();

                            for f in sub_type.discriminating_fields() {
                                expected.push(quote!(#(&sub_type.ident)Keys.#(f.safe_ident())));
                            }

                            quote_in! { *o =>
                                if #keys == Set([#(for e in expected join (, ) => #e)]) {
                                    self = try .#(&sub_type.ident)(#(&sub_type.name)(from: decoder))
                                    return
                                }
                                #<line>
                            }
                        })
                        let context = DecodingError.Context(
                            codingPath: decoder.codingPath,
                            debugDescription: "no legal field combination"
                        )

                        throw DecodingError.dataCorrupted(context)
                    }
                }
            }
        }

        fn encodable<'f>(name: &'f Name, body: &'f RpInterfaceBody) -> impl FormatInto<Swift> + 'f {
            return quote_fn! {
                extension #name: Encodable {
                    #(ref o => match body.sub_type_strategy {
                        core::RpSubTypeStrategy::Tagged { .. } => {
                            encode_tagged(o, body);
                        }
                        core::RpSubTypeStrategy::Untagged => {
                            encode_untagged(o, body);
                        }
                    })
                }
            };

            fn encode_tagged(t: &mut swift::Tokens, body: &RpInterfaceBody) {
                quote_in! { *t =>
                    public func encode(to encoder: Encoder) throws {
                        var values = encoder.container(keyedBy: CodingKeys.self)

                        switch self {
                        #(for sub_type in &body.sub_types join (#<push>) {
                            case .#(&sub_type.ident)(let d):
                                try values.encode(#(quoted(sub_type.name())), forKey: .tag)
                                try d.encode(to: encoder)
                        })
                        }
                    }
                }
            }

            fn encode_untagged(o: &mut swift::Tokens, body: &RpInterfaceBody) {
                quote_in! { *o =>
                    public func encode(to encoder: Encoder) throws {
                        switch self {
                        #(for sub_type in &body.sub_types join (#<push>) {
                            case .#(sub_type.ident.as_str())(let d):
                                try d.encode(to: encoder)
                        })
                        }
                    }
                }
            }
        }
    }
}

impl codegen::interface_model_added::Codegen for Codegen {
    fn generate(&self, e: codegen::interface_model_added::Args<'_>) {
        let codegen::interface_model_added::Args {
            container, body, ..
        } = e;

        match &body.sub_type_strategy {
            core::RpSubTypeStrategy::Tagged { tag, .. } => {
                container.push(quote! {
                    enum CodingKeys: String, CodingKey {
                        case tag = #(quoted(tag.as_str()))
                    }
                });
            }
            core::RpSubTypeStrategy::Untagged => {
                let all = body
                    .sub_types
                    .iter()
                    .flat_map(|s| s.fields.iter())
                    .filter(|f| f.is_required())
                    .map(|f| f.name())
                    .collect::<BTreeSet<_>>();

                for sub_type in &body.sub_types {
                    let mut current = all.clone();

                    // rest of the fields that need to be declared to throw of the count in
                    // case of intersections.
                    container.push(quote!{
                        enum #(&sub_type.ident)Keys: String, CodingKey {
                            #(for f in sub_type.fields.iter().filter(|f| f.is_required()) join (#<push>) {
                                #(ref o => {
                                    current.remove(f.name());
                                    quote_in!(*o => case #(f.safe_ident()) = #(quoted(f.name())));
                                })
                            })

                            #(for (n, name) in current.into_iter().enumerate() join (#<push>) {
                                case _k#(n.to_string()) = #(quoted(name))
                            })
                        }
                    });
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
