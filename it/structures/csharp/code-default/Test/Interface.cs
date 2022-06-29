using JsonSubTypes;
using Newtonsoft.Json;
using System;

namespace Test {
    [JsonConverter(typeof(JsonSubtypes), "type")][JsonSubtypes.KnownSubType(typeof(Interface.SubType), "SubType")]
    public abstract class Interface {
        [JsonProperty("type", Required = Required.DisallowNull)]
        private String TypeField {
            get;
        }

        public Interface(String TypeField) {
            this.TypeField = TypeField;
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class SubType : Interface {

            [JsonConstructor]
            public SubType (
                [JsonProperty("type", Required = Required.DisallowNull)] String TypeField
            ) : base(TypeField) {}

            public override bool Equals(Object other) {
                SubType o = other as SubType;

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
                return "SubType()";
            }
        }
    }
}
