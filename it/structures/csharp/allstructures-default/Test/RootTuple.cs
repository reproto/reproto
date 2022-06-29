using JsonSubTypes;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace Test {
    [JsonConverter(typeof(RootTuple.Json_Net_Converter))]
    public class RootTuple {

        [JsonConstructor]
        public RootTuple () {}

        public override bool Equals(Object other) {
            RootTuple o = other as RootTuple;

            if (o == null) {
                return false;
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            return result;
        }

        public override String ToString() {
            return "RootTuple()";
        }

        public class Json_Net_Converter : JsonConverter {
            public override bool CanConvert(System.Type objectType) {
                return objectType == typeof(RootTuple);
            }

            public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
                RootTuple o = (RootTuple)obj;
                JArray array = new JArray();

                array.WriteTo(writer);
            }

            public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
                JArray array = JArray.Load(reader);
                IEnumerator<JToken> enumerator = array.GetEnumerator();

                return new RootTuple();
            }
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class NestedType {

            [JsonConstructor]
            public NestedType () {}

            public override bool Equals(Object other) {
                NestedType o = other as NestedType;

                if (o == null) {
                    return false;
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                return result;
            }

            public override String ToString() {
                return "NestedType()";
            }
        }
        [JsonConverter(typeof(JsonSubtypes), "type")][JsonSubtypes.KnownSubType(typeof(NestedInterface.Foo), "Foo")]
        public abstract class NestedInterface {
            [JsonProperty("type", Required = Required.DisallowNull)]
            private String TypeField {
                get;
            }

            public NestedInterface(String TypeField) {
                this.TypeField = TypeField;
            }

            [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
            public class Foo : NestedInterface {

                [JsonConstructor]
                public Foo (
                    [JsonProperty("type", Required = Required.DisallowNull)] String TypeField
                ) : base(TypeField) {}

                public override bool Equals(Object other) {
                    Foo o = other as Foo;

                    if (o == null) {
                        return false;
                    }

                    return true;
                }

                public override int GetHashCode() {
                    int result = 1;
                    return result;
                }

                public override String ToString() {
                    return "Foo()";
                }
            }
        }
        [JsonConverter(typeof(StringEnumConverter))]
        public enum NestedEnum {
            [EnumMember(Value = "Foo")]
            FOO
        }
        [JsonConverter(typeof(NestedTuple.Json_Net_Converter))]
        public class NestedTuple {

            [JsonConstructor]
            public NestedTuple () {}

            public override bool Equals(Object other) {
                NestedTuple o = other as NestedTuple;

                if (o == null) {
                    return false;
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                return result;
            }

            public override String ToString() {
                return "NestedTuple()";
            }

            public class Json_Net_Converter : JsonConverter {
                public override bool CanConvert(System.Type objectType) {
                    return objectType == typeof(NestedTuple);
                }

                public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
                    NestedTuple o = (NestedTuple)obj;
                    JArray array = new JArray();

                    array.WriteTo(writer);
                }

                public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
                    JArray array = JArray.Load(reader);
                    IEnumerator<JToken> enumerator = array.GetEnumerator();

                    return new NestedTuple();
                }
            }

            [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
            public class Nested {

                [JsonConstructor]
                public Nested () {}

                public override bool Equals(Object other) {
                    Nested o = other as Nested;

                    if (o == null) {
                        return false;
                    }

                    return true;
                }

                public override int GetHashCode() {
                    int result = 1;
                    return result;
                }

                public override String ToString() {
                    return "Nested()";
                }
            }
        }
    }
}
