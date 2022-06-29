using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    /// Complex object.
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Other {
        [JsonProperty("a")]
        public String a {
            get;
        }

        [JsonConstructor]
        public Other (
            [JsonProperty("a")] String a
        ) {
            this.a = a;
        }

        public override bool Equals(Object other) {
            Other o = other as Other;

            if (o == null) {
                return false;
            }

            if (this.a == null) {
                if (o.a != null) {
                    return false;
                }
            } else {
                if (!this.a.Equals(o.a)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.a.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Other(");
            b.Append("a=");
            b.Append(this.a);
            b.Append(")");

            return b.ToString();
        }
    }
}
