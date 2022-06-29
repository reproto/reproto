using Newtonsoft.Json;
using System;

namespace True {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Empty {

        [JsonConstructor]
        public Empty () {}

        public override bool Equals(Object other) {
            Empty o = other as Empty;

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
            return "Empty()";
        }
    }
}
