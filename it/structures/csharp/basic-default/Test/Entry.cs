using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        /// The foo field.
        [JsonProperty("foo")]
        public Foo foo {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("foo")] Foo foo
        ) {
            this.foo = foo;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (this.foo == null) {
                if (o.foo != null) {
                    return false;
                }
            } else {
                if (!this.foo.Equals(o.foo)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.foo.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("foo=");
            b.Append(this.foo);
            b.Append(")");

            return b.ToString();
        }
    }
}
