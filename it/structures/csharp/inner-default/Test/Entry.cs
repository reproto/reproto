using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("a")]
        public A a {
            get;
        }

        [JsonProperty("b")]
        public A.B b {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("a")] A a,
            [JsonProperty("b")] A.B b
        ) {
            this.a = a;
            this.b = b;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

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

            if (this.b == null) {
                if (o.b != null) {
                    return false;
                }
            } else {
                if (!this.b.Equals(o.b)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.a.GetHashCode();
            result = result * 31 + this.b.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("a=");
            b.Append(this.a);
            b.Append(", ");
            b.Append("b=");
            b.Append(this.b);
            b.Append(")");

            return b.ToString();
        }
    }
}
