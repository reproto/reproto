using JsonSubTypes;
using Newtonsoft.Json;
using System;

namespace Test {
    [JsonConverter(typeof(JsonSubtypes), "type")][JsonSubtypes.KnownSubType(typeof(RootInterface.Foo), "Foo")]
    public abstract class RootInterface {
        [JsonProperty("type", Required = Required.DisallowNull)]
        private String TypeField {
            get;
        }

        public RootInterface(String TypeField) {
            this.TypeField = TypeField;
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Foo : RootInterface {

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
}
