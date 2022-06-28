use crate::codegen;
use crate::flavored::*;
use crate::Options;
use genco::prelude::*;
use reproto_core::Spanned;
use std::rc::Rc;

pub fn initialize(opt: &mut Options) {
    let codegen = Rc::new(Codegen::new());
    opt.gen.class.push(codegen.clone());
    opt.gen.class_field.push(codegen.clone());
    opt.gen.class_constructor.push(codegen.clone());
    opt.gen.class_constructor_arg.push(codegen.clone());
    opt.gen.enum_type.push(codegen.clone());
    opt.gen.enum_variant.push(codegen.clone());
    opt.gen.tuple.push(codegen.clone());
    opt.gen.interface.push(codegen.clone());
    opt.gen.interface_tag_constructor_arg.push(codegen.clone());
}

struct Codegen {
    object: csharp::Import,
    invalid_operation_exception: csharp::Import,
    i_enumerator: csharp::Import,

    json_object: csharp::Import,
    json_property: csharp::Import,
    json_constructor: csharp::Import,
    null_value_handling: csharp::Import,

    json_converter: csharp::Import,
    enum_member: csharp::Import,
    string_enum_converter: csharp::Import,

    j_object: csharp::Import,
    j_array: csharp::Import,
    j_token: csharp::Import,
    json_reader: csharp::Import,
    json_writer: csharp::Import,
    json_serializer: csharp::Import,

    json_sub_types: csharp::Import,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            object: csharp::import("System", "Object"),
            invalid_operation_exception: csharp::import("System", "InvalidOperationException"),
            i_enumerator: csharp::import("System.Collections.Generic", "IEnumerator"),

            json_object: csharp::import("Newtonsoft.Json", "JsonObject"),
            json_property: csharp::import("Newtonsoft.Json", "JsonProperty"),
            json_constructor: csharp::import("Newtonsoft.Json", "JsonConstructor"),
            null_value_handling: csharp::import("Newtonsoft.Json", "NullValueHandling"),

            json_converter: csharp::import("Newtonsoft.Json", "JsonConverter"),
            enum_member: csharp::import("System.Runtime.Serialization", "EnumMember"),
            string_enum_converter: csharp::import(
                "Newtonsoft.Json.Converters",
                "StringEnumConverter",
            ),

            j_object: csharp::import("Newtonsoft.Json.Linq", "JObject"),
            j_array: csharp::import("Newtonsoft.Json.Linq", "JArray"),
            j_token: csharp::import("Newtonsoft.Json.Linq", "JToken"),
            json_reader: csharp::import("Newtonsoft.Json", "JsonReader"),
            json_writer: csharp::import("Newtonsoft.Json", "JsonWriter"),
            json_serializer: csharp::import("Newtonsoft.Json", "JsonSerializer"),

            json_sub_types: csharp::import("JsonSubTypes", "JsonSubtypes"),
        }
    }

    pub(crate) fn read_field<'a>(&'a self, f: &'a Spanned<Field>) -> impl FormatInto<Csharp> + 'a {
        quote_fn! {
            if (!enumerator.MoveNext()) {
                throw new #(&self.invalid_operation_exception)("expected more items in array");
            }

            #(&f.ty) #(f.safe_ident()) = enumerator.Current.ToObject<#(&f.ty)>(serializer);
        }
    }

    pub(crate) fn try_read_untagged_sub_type<'a>(
        &'a self,
        sub_type: &'a RpSubType,
    ) -> impl FormatInto<Csharp> + 'a {
        quote_fn! {
            if (#(for f in sub_type.discriminating_fields() join ( && ) => o.ContainsKey(#(quoted(f.name()))))) {
                _isInsideRead = true;
                try {
                    return serializer.Deserialize(o.CreateReader(), typeof(#(&sub_type.ident)));
                } finally {
                    _isInsideRead = false;
                }
            }
        }
    }
}

impl codegen::class::Codegen for Codegen {
    fn generate(&self, e: codegen::class::Args<'_>) {
        e.annotations.push(quote! {
            [#(&self.json_object)(ItemNullValueHandling = #(&self.null_value_handling).Ignore)]
        })
    }
}

impl codegen::class_field::Codegen for Codegen {
    fn generate(&self, e: codegen::class_field::Args<'_>) {
        e.annotations.push(quote! {
            [#(&self.json_property)(#(quoted(e.field.name())))]
        })
    }
}

impl codegen::class_constructor::Codegen for Codegen {
    fn generate(&self, e: codegen::class_constructor::Args<'_>) {
        e.annotations.push(quote! {
            [#(&self.json_constructor)]
        })
    }
}

impl codegen::class_constructor_arg::Codegen for Codegen {
    fn generate(&self, e: codegen::class_constructor_arg::Args<'_>) {
        e.annotations.push(quote! {
            [#(&self.json_property)(#(quoted(e.field.name())))]
        })
    }
}

impl codegen::enum_type::Codegen for Codegen {
    fn generate(&self, e: codegen::enum_type::Args<'_>) {
        match e.variants {
            RpVariants::String { .. } => e.annotations.push(quote! {
                [#(&self.json_converter)(typeof(#(&self.string_enum_converter)))]
            }),
            _ => (),
        }
    }
}

impl codegen::enum_variant::Codegen for Codegen {
    fn generate(&self, e: codegen::enum_variant::Args<'_>) {
        match e.variant.value {
            RpVariantValue::String(string) => e.annotations.push(quote! {
                [#(&self.enum_member)(Value = #(quoted(string)))]
            }),
            RpVariantValue::Number(number) => {
                *e.value = Some(quote!(#(display(number))));
            }
        }
    }
}

impl codegen::tuple::Codegen for Codegen {
    fn generate(&self, args: codegen::tuple::Args<'_>) {
        let object = &self.object;
        let i_enumerator = &self.i_enumerator;

        let j_array = &self.j_array;
        let j_token = &self.j_token;
        let json_writer = &self.json_writer;
        let json_reader = &self.json_reader;
        let json_serializer = &self.json_serializer;
        let json_converter = &self.json_converter;

        args.annotations.push(quote! {
            [#json_converter(typeof(#(args.ident).Json_Net_Converter))]
        });

        args.inner.push(quote!{
            public class Json_Net_Converter : #json_converter {
                public override bool CanConvert(System.Type objectType) {
                    return objectType == typeof(#(args.ident));
                }

                public override void WriteJson(#json_writer writer, #object obj, #json_serializer serializer) {
                    #(args.ident) o = (#(args.ident))obj;
                    #j_array array = new #j_array();

                    #(for f in args.fields join (#<line>) {
                        array.Add(#j_token.FromObject(o.#(f.safe_ident()), serializer));
                    })

                    array.WriteTo(writer);
                }

                public override #object ReadJson(#json_reader reader, System.Type objectType, #object existingValue, #json_serializer serializer) {
                    #j_array array = #j_array.Load(reader);
                    #i_enumerator<#j_token> enumerator = array.GetEnumerator();

                    #(for f in args.fields join (#<line>) {
                        #(self.read_field(f))
                    })

                    return new #(args.ident)(#(for f in args.fields join (, ) => #(f.safe_ident())));
                }
            }
        });
    }
}

impl codegen::interface::Codegen for Codegen {
    fn generate(&self, args: codegen::interface::Args<'_>) {
        match args.sub_type_strategy {
            RpSubTypeStrategy::Tagged { tag } => {
                args.tag_annotations.push(quote! {
                    [JsonProperty(#(quoted(tag)), Required = Required.DisallowNull)]
                });

                args.annotations.push(quote! {
                    [#(&self.json_converter)(typeof(#(&self.json_sub_types)), #(quoted(tag)))]
                });

                for sub_type in args.sub_types {
                    args.annotations.push(quote!{
                        [#(&self.json_sub_types).KnownSubType(typeof(#(args.ident).#(&sub_type.ident)), #(quoted(sub_type.name())))]
                    });
                }
            }
            RpSubTypeStrategy::Untagged => {
                args.annotations.push(quote! {
                    [#(&self.json_converter)(typeof(#(args.ident).Json_Net_Converter))]
                });

                args.inner.push(quote!{
                    public class Json_Net_Converter : #(&self.json_converter) {
                        [ThreadStatic]
                        private static bool _isInsideRead;
                        public override bool CanWrite {
                            get { return false; }
                        }
                        public override bool CanRead {
                          get {
                            return !_isInsideRead;
                          }
                        }

                        public override bool CanConvert(System.Type objectType) {
                            return false;
                        }

                        public override void WriteJson(#(&self.json_writer) writer, #(&self.object) obj, #(&self.json_serializer) serializer) {
                            throw new #(&self.invalid_operation_exception)("not implemented");
                        }

                        public override #(&self.object) ReadJson(#(&self.json_reader) reader, System.Type objectType, #(&self.object) existingValue, #(&self.json_serializer) serializer) {
                            #(&self.j_object) o = #(&self.j_object).Load(reader);

                            #(for sub_type in args.sub_types {
                                #(self.try_read_untagged_sub_type(sub_type))
                            })

                            throw new #(&self.invalid_operation_exception)("no legal combination of fields");
                        }
                    }
                });
            }
        }
    }
}

impl codegen::interface_tag_constructor_arg::Codegen for Codegen {
    fn generate(&self, args: codegen::interface_tag_constructor_arg::Args<'_>) {
        args.annotations.push(quote! {
            [#(&self.json_property)(#(quoted(args.tag)), Required = Required.DisallowNull)]
        });
    }
}
