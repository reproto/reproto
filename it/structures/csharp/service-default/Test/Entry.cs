using Newtonsoft.Json;
using System;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {

        [JsonConstructor]
        public Entry () {}

        public override bool Equals(Object other) {
            Entry o = other as Entry;

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
            return "Entry()";
        }
    }
}
