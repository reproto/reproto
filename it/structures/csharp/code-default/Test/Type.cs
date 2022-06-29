using Newtonsoft.Json;
using System;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Type {

        [JsonConstructor]
        public Type () {}

        public override bool Equals(Object other) {
            Type o = other as Type;

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
            return "Type()";
        }
    }
}
