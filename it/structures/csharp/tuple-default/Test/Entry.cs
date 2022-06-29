using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("tuple1")]
        public Tuple1 tuple1 {
            get;
        }

        [JsonProperty("tuple2")]
        public Tuple2 tuple2 {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("tuple1")] Tuple1 tuple1,
            [JsonProperty("tuple2")] Tuple2 tuple2
        ) {
            this.tuple1 = tuple1;
            this.tuple2 = tuple2;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (this.tuple1 == null) {
                if (o.tuple1 != null) {
                    return false;
                }
            } else {
                if (!this.tuple1.Equals(o.tuple1)) {
                    return false;
                }
            }

            if (this.tuple2 == null) {
                if (o.tuple2 != null) {
                    return false;
                }
            } else {
                if (!this.tuple2.Equals(o.tuple2)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.tuple1.GetHashCode();
            result = result * 31 + this.tuple2.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("tuple1=");
            b.Append(this.tuple1);
            b.Append(", ");
            b.Append("tuple2=");
            b.Append(this.tuple2);
            b.Append(")");

            return b.ToString();
        }
    }
}
