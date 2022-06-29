using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Bar {
        /// The inner field.
        [JsonProperty("field")]
        public Bar.Inner field {
            get;
        }

        [JsonConstructor]
        public Bar (
            [JsonProperty("field")] Bar.Inner field
        ) {
            this.field = field;
        }

        public override bool Equals(Object other) {
            Bar o = other as Bar;

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

            b.Append("Bar(");
            b.Append("field=");
            b.Append(this.field);
            b.Append(")");

            return b.ToString();
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Inner {
            /// The field.
            [JsonProperty("field")]
            public String field {
                get;
            }

            [JsonConstructor]
            public Inner (
                [JsonProperty("field")] String field
            ) {
                this.field = field;
            }

            public override bool Equals(Object other) {
                Inner o = other as Inner;

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

                b.Append("Inner(");
                b.Append("field=");
                b.Append(this.field);
                b.Append(")");

                return b.ToString();
            }
        }
    }
}
