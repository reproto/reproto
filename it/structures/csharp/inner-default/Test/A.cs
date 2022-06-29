using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class A {
        [JsonProperty("b")]
        public A.B b {
            get;
        }

        [JsonConstructor]
        public A (
            [JsonProperty("b")] A.B b
        ) {
            this.b = b;
        }

        public override bool Equals(Object other) {
            A o = other as A;

            if (o == null) {
                return false;
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
            result = result * 31 + this.b.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("A(");
            b.Append("b=");
            b.Append(this.b);
            b.Append(")");

            return b.ToString();
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class B {
            [JsonProperty("field")]
            public String field {
                get;
            }

            [JsonConstructor]
            public B (
                [JsonProperty("field")] String field
            ) {
                this.field = field;
            }

            public override bool Equals(Object other) {
                B o = other as B;

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

                b.Append("B(");
                b.Append("field=");
                b.Append(this.field);
                b.Append(")");

                return b.ToString();
            }
        }
    }
}
