using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Foo {
        /// The field.
        [JsonProperty("field")]
        public String field {
            get;
        }

        [JsonConstructor]
        public Foo (
            [JsonProperty("field")] String field
        ) {
            this.field = field;
        }

        public override bool Equals(Object other) {
            Foo o = other as Foo;

            if (o == null) {
                return false;
            }

            if (this.field == null) {
                if (o.field != null) {
                    return false;
                }
            } else {
                if (!this.field.Equals(o.field)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.field.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Foo(");
            b.Append("field=");
            b.Append(this.field);
            b.Append(")");

            return b.ToString();
        }
    }
}
