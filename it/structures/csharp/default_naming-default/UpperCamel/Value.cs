using Newtonsoft.Json;
using System;
using System.Text;

namespace UpperCamel {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Value {
        [JsonProperty("FooBar")]
        public String fooBar {
            get;
        }

        [JsonConstructor]
        public Value (
            [JsonProperty("FooBar")] String fooBar
        ) {
            this.fooBar = fooBar;
        }

        public override bool Equals(Object other) {
            Value o = other as Value;

            if (o == null) {
                return false;
            }

            if (this.fooBar == null) {
                if (o.fooBar != null) {
                    return false;
                }
            } else {
                if (!this.fooBar.Equals(o.fooBar)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.fooBar.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Value(");
            b.Append("foo_bar=");
            b.Append(this.fooBar);
            b.Append(")");

            return b.ToString();
        }
    }
}
