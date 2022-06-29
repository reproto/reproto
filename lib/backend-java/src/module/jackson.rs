//! Module that adds fasterxml annotations to generated classes.

use crate::codegen;
use crate::flavored::*;
use crate::Options;
use genco::prelude::*;
use std::rc::Rc;
use RpSubTypeStrategy;

pub struct Module;

impl Module {
    pub fn initialize(self, options: &mut Options) {
        let generator = Rc::new(Generator::new());

        options.gen.class_constructor_arg.push(generator.clone());
        options.gen.class_constructor.push(generator.clone());
        options.gen.class_field.push(generator.clone());
        options.gen.class_getter.push(generator.clone());
        options.gen.tuple.push(generator.clone());
        options.gen.enum_ty.push(generator.clone());
        options.gen.interface.push(generator.clone());
        options.gen.interface_sub_type.push(generator.clone());
    }
}

pub struct Generator {
    io_exception: java::Import,

    json_property: java::Import,
    json_creator: java::Import,
    json_value: java::Import,
    json_format: java::Import,
    json_serialize: java::Import,
    json_deserialize: java::Import,

    type_reference: java::Import,

    json_serializer: java::Import,
    json_serializer_provider: java::Import,
    json_generator: java::Import,

    json_deserializer: java::Import,
    json_token: java::Import,
    json_parser: java::Import,
    deserialization_context: java::Import,

    json_type_info: java::Import,
    json_sub_types: java::Import,
    tree_traversing_parser: java::Import,

    object_node: java::Import,

    hash_set: java::Import,
    set: java::Import,
    iterator: java::Import,
}

impl Generator {
    fn new() -> Self {
        Self {
            io_exception: java::import("java.io", "IOException"),

            json_property: java::import("com.fasterxml.jackson.annotation", "JsonProperty"),
            json_creator: java::import("com.fasterxml.jackson.annotation", "JsonCreator"),
            json_value: java::import("com.fasterxml.jackson.annotation", "JsonValue"),
            json_format: java::import("com.fasterxml.jackson.annotation", "JsonFormat"),
            json_serialize: java::import(
                "com.fasterxml.jackson.databind.annotation",
                "JsonSerialize",
            ),
            json_deserialize: java::import(
                "com.fasterxml.jackson.databind.annotation",
                "JsonDeserialize",
            ),

            type_reference: java::import("com.fasterxml.jackson.core.type", "TypeReference"),

            json_serializer: java::import("com.fasterxml.jackson.databind", "JsonSerializer"),
            json_serializer_provider: java::import(
                "com.fasterxml.jackson.databind",
                "SerializerProvider",
            ),
            json_generator: java::import("com.fasterxml.jackson.core", "JsonGenerator"),

            json_deserializer: java::import("com.fasterxml.jackson.databind", "JsonDeserializer"),
            json_token: java::import("com.fasterxml.jackson.core", "JsonToken"),
            json_parser: java::import("com.fasterxml.jackson.core", "JsonParser"),
            deserialization_context: java::import(
                "com.fasterxml.jackson.databind",
                "DeserializationContext",
            ),

            json_type_info: java::import("com.fasterxml.jackson.annotation", "JsonTypeInfo"),
            json_sub_types: java::import("com.fasterxml.jackson.annotation", "JsonSubTypes"),
            tree_traversing_parser: java::import(
                "com.fasterxml.jackson.databind.node",
                "TreeTraversingParser",
            ),

            object_node: java::import("com.fasterxml.jackson.databind.node", "ObjectNode"),
            hash_set: java::import("java.util", "HashSet"),
            set: java::import("java.util", "Set"),
            iterator: java::import("java.util", "Iterator"),
        }
    }
}

impl codegen::class_constructor_arg::Codegen for Generator {
    fn generate(&self, args: codegen::class_constructor_arg::Args<'_>) {
        args.annotations.push(quote! {
            @$(&self.json_property)($(quoted(args.field.name())))
        })
    }
}

impl codegen::class_field::Codegen for Generator {
    fn generate(&self, args: codegen::class_field::Args<'_>) {
        args.annotations.push(quote! {
            @$(&self.json_property)($(quoted(args.field.name())))
        });

        if let Type::DateTime { .. } = args.field.ty {
            args.annotations.push(quote! {
                @$(&self.json_format)(shape = $(&self.json_format).Shape.STRING)
            });
        }
    }
}

impl codegen::class_constructor::Codegen for Generator {
    fn generate(&self, args: codegen::class_constructor::Args<'_>) {
        args.annotations.push(quote! {
            @$(&self.json_creator)
        })
    }
}

impl codegen::class_getter::Codegen for Generator {
    fn generate(&self, args: codegen::class_getter::Args<'_>) {
        args.annotations.push(quote! {
            @$(&self.json_property)($(quoted(args.field.name())))
        })
    }
}

impl Generator {
    fn serialize_type(&self, t: &mut java::Tokens, f: &RpField, d: &str, value: &str) {
        use Primitive::*;

        quote_in! { *t =>
            $(match &f.ty {
                Type::Primitive { primitive } | Type::Boxed { primitive } => $(match primitive {
                    Integer | Long | Float | Double => {
                        $d.writeNumber($value.$(f.safe_ident()));
                    },
                    Boolean => {
                        $d.writeBoolean($value.$(f.safe_ident()));
                    },
                }),
                Type::String => {
                    $d.writeString($value.$(f.safe_ident()));
                },
                _ => {
                    $d.writeObject($value.$(f.safe_ident()));
                },
            })
        }
    }

    fn deserialize_type(&self, t: &mut java::Tokens, f: &RpField, ctxt: &str, parser: &str) {
        use Primitive::*;

        quote_in! { *t =>
            $(match &f.ty {
                Type::Primitive { primitive } | Type::Boxed { primitive } => $(match primitive {
                    Integer => {
                        if ($parser.nextToken() != $(&self.json_token).VALUE_NUMBER_INT) {
                            throw $ctxt.wrongTokenException($parser, $(&self.json_token).VALUE_NUMBER_INT, null);
                        }

                        final int $(f.safe_ident()) = $parser.getIntValue();
                    }
                    Long => {
                        if ($parser.nextToken() != $(&self.json_token).VALUE_NUMBER_INT) {
                            throw $ctxt.wrongTokenException($parser, $(&self.json_token).VALUE_NUMBER_INT, null);
                        }

                        final long $(f.safe_ident()) = $parser.getLongValue();
                    }
                    Float => {
                        $parser.nextToken();

                        if ($parser.currentToken() != $(&self.json_token).VALUE_NUMBER_INT || $parser.currentToken() != $(&self.json_token).VALUE_NUMBER_FLOAT) {
                            throw $ctxt.wrongTokenException($parser, $(&self.json_token).VALUE_NUMBER_FLOAT, null);
                        }

                        final double $(f.safe_ident()) = $parser.getDoubleValue();
                    }
                    Double => {
                        $parser.nextToken();

                        if ($parser.currentToken() != $(&self.json_token).VALUE_NUMBER_INT || $parser.currentToken() != $(&self.json_token).VALUE_NUMBER_FLOAT) {
                            throw $ctxt.wrongTokenException($parser, $(&self.json_token).VALUE_NUMBER_FLOAT, null);
                        }

                        final float $(f.safe_ident()) = $parser.getFloatValue();
                    }
                    Boolean => {
                        final bool $(f.safe_ident()) = $parser.nextBooleanValue();
                    },
                }),
                Type::String => {
                    if ($parser.nextToken() != $(&self.json_token).VALUE_STRING) {
                        throw $ctxt.wrongTokenException($parser, $(&self.json_token).VALUE_STRING, null);
                    }

                    final String $(f.safe_ident()) = $parser.getText();
                },
                ty @ Type::Object | ty @ Type::Import { .. } | ty @ Type::DateTime { .. } => {
                    $parser.nextToken();

                    final $ty $(f.safe_ident()) = $parser.readValueAs($ty.class);
                },
                ty => {
                    $parser.nextToken();

                    final $ty $(f.safe_ident()) = $parser.readValueAs(new $(&self.type_reference)<$ty>() {});
                },
            })
        }
    }
}

impl codegen::tuple::Codegen for Generator {
    fn generate(&self, args: codegen::tuple::Args<'_>) {
        args.annotations.push(quote! {
            @$(&self.json_serialize)(using = $(args.ident).Serializer.class)
        });

        args.annotations.push(quote! {
            @$(&self.json_deserialize)(using = $(args.ident).Deserializer.class)
        });

        args.inner.push(quote! {
            public static class Serializer extends $(&self.json_serializer)<$(args.ident)> {
                @Override
                public void serialize(final $(args.ident) value_, final $(&self.json_generator) gen_, final $(&self.json_serializer_provider) provider_) throws $(&self.io_exception) {
                    gen_.writeStartArray();

                    $(for f in args.fields join ($['\n']) {
                        $(ref t => self.serialize_type(t, f, "gen_", "value_"))
                    })

                    gen_.writeEndArray();
                }
            }
        });

        args.inner.push(quote! {
            public static class Deserializer extends $(&self.json_deserializer)<$(args.ident)> {
                @Override
                public $(args.ident) deserialize(final $(&self.json_parser) parser_, final $(&self.deserialization_context) ctxt_) throws $(&self.io_exception) {
                    if (parser_.getCurrentToken() != $(&self.json_token).START_ARRAY) {
                        throw ctxt_.wrongTokenException(parser_, $(&self.json_token).START_ARRAY, null);
                    }

                    $(for f in args.fields join ($['\n']) {
                        $(ref t => self.deserialize_type(t, f, "ctxt_", "parser_"))
                    })

                    if (parser_.nextToken() != $(&self.json_token).END_ARRAY) {
                        throw ctxt_.wrongTokenException(parser_, $(&self.json_token).END_ARRAY, null);
                    }

                    return new $(args.ident)($(for f in args.fields join (, ) => $(f.safe_ident())));
                }
            }
        });
    }
}

impl codegen::enum_ty::Codegen for Generator {
    fn generate(&self, args: codegen::enum_ty::Args<'_>) {
        args.inner.push(quote! {
            @$(&self.json_creator)
            public static $(args.ident) fromValue(final $(args.enum_type) value) {
                for (final $(args.ident) v : values()) {
                    if ($(args.enum_type.equals(quote!(v.value), quote!(value)))) {
                        return v;
                    }
                }

                throw new IllegalArgumentException("value");
            }
        });

        args.inner.push(quote! {
            @$(&self.json_value)
            public $(args.enum_type) toValue() {
                return this.value;
            }
        });
    }
}

impl codegen::interface::Codegen for Generator {
    fn generate(&self, args: codegen::interface::Args<'_>) {
        match args.sub_type_strategy {
            RpSubTypeStrategy::Tagged { tag } => {
                args.annotations.push(quote! {
                    @$(&self.json_type_info)(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property=$(quoted(tag)))
                });

                args.annotations.push(quote!{
                    @$(&self.json_sub_types)({
                        $(for s in args.sub_types join ($['\r']) {
                            @$(&self.json_sub_types).Type(name=$(quoted(s.name())), value=$(args.ident).$(&s.ident).class),
                        })
                    })
                });
            }
            RpSubTypeStrategy::Untagged => {
                args.annotations.push(quote! {
                    @$(&self.json_deserialize)(using = $(args.ident).Deserializer.class)
                });

                args.inner.push(quote! {
                    public static class Deserializer extends $(&self.json_deserializer)<$(args.ident)> {
                        @Override
                        public Untagged deserialize(final $(&self.json_parser) parser, final $(&self.deserialization_context) ctxt) throws $(&self.io_exception) {
                            final $(&self.object_node) object = parser.readValueAs($(&self.object_node).class);

                            final $(&self.set)<String> tags = new $(&self.hash_set)<String>();
                            final $(&self.iterator)<String> it = object.fieldNames();

                            while (it.hasNext()) {
                                tags.add(it.next());
                            }

                            $(for s in args.sub_types join ($['\n']) {
                                if ($(for f in s.discriminating_fields() join ( && ) => tags.contains($(quoted(&f.ident))))) {
                                    return new $(&self.tree_traversing_parser)(object, parser.getCodec()).readValueAs($(args.ident).$(&s.ident).class);
                                }
                            })

                            throw ctxt.mappingException("no legal combination of fields available");
                        }
                    }
                });
            }
        }
    }
}

impl codegen::interface_sub_type::Codegen for Generator {
    fn generate(&self, args: codegen::interface_sub_type::Args<'_>) {
        match args.sub_type_strategy {
            RpSubTypeStrategy::Tagged { .. } => {}
            RpSubTypeStrategy::Untagged => {
                args.annotations.push(quote! {
                    @$(&self.json_deserialize)(using = $(&self.json_deserializer).None.class)
                });
            }
        }
    }
}
