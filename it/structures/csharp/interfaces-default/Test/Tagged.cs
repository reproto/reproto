using JsonSubTypes;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonConverter(typeof(JsonSubtypes), "@type")][JsonSubtypes.KnownSubType(typeof(Tagged.A), "foo")][JsonSubtypes.KnownSubType(typeof(Tagged.B), "b")][JsonSubtypes.KnownSubType(typeof(Tagged.Bar), "Bar")][JsonSubtypes.KnownSubType(typeof(Tagged.Baz), "Baz")]
    public abstract class Tagged {
        [JsonProperty("@type", Required = Required.DisallowNull)]
        private String TypeField {
            get;
        }

        public Tagged(String TypeField) {
            this.TypeField = TypeField;
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class A : Tagged {
            [JsonProperty("shared")]
            public String shared {
                get;
            }

            [JsonConstructor]
            public A (
                [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
                [JsonProperty("shared")] String shared
            ) : base(TypeField) {
                this.shared = shared;
            }

            public override bool Equals(Object other) {
                A o = other as A;

                if (o == null) {
                    return false;
                }

                if (this.shared == null) {
                    if (o.shared != null) {
                        return false;
                    }
                } else {
                    if (!this.shared.Equals(o.shared)) {
                        return false;
                    }
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                result = result * 31 + this.shared.GetHashCode();
                return result;
            }

            public override String ToString() {
                StringBuilder b = new StringBuilder();

                b.Append("A(");
                b.Append("shared=");
                b.Append(this.shared);
                b.Append(")");

                return b.ToString();
            }
        }
        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class B : Tagged {
            [JsonProperty("shared")]
            public String shared {
                get;
            }

            [JsonConstructor]
            public B (
                [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
                [JsonProperty("shared")] String shared
            ) : base(TypeField) {
                this.shared = shared;
            }

            public override bool Equals(Object other) {
                B o = other as B;

                if (o == null) {
                    return false;
                }

                if (this.shared == null) {
                    if (o.shared != null) {
                        return false;
                    }
                } else {
                    if (!this.shared.Equals(o.shared)) {
                        return false;
                    }
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                result = result * 31 + this.shared.GetHashCode();
                return result;
            }

            public override String ToString() {
                StringBuilder b = new StringBuilder();

                b.Append("B(");
                b.Append("shared=");
                b.Append(this.shared);
                b.Append(")");

                return b.ToString();
            }
        }
        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Bar : Tagged {
            [JsonProperty("shared")]
            public String shared {
                get;
            }

            [JsonConstructor]
            public Bar (
                [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
                [JsonProperty("shared")] String shared
            ) : base(TypeField) {
                this.shared = shared;
            }

            public override bool Equals(Object other) {
                Bar o = other as Bar;

                if (o == null) {
                    return false;
                }

                if (this.shared == null) {
                    if (o.shared != null) {
                        return false;
                    }
                } else {
                    if (!this.shared.Equals(o.shared)) {
                        return false;
                    }
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                result = result * 31 + this.shared.GetHashCode();
                return result;
            }

            public override String ToString() {
                StringBuilder b = new StringBuilder();

                b.Append("Bar(");
                b.Append("shared=");
                b.Append(this.shared);
                b.Append(")");

                return b.ToString();
            }
        }
        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Baz : Tagged {
            [JsonProperty("shared")]
            public String shared {
                get;
            }

            [JsonConstructor]
            public Baz (
                [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
                [JsonProperty("shared")] String shared
            ) : base(TypeField) {
                this.shared = shared;
            }

            public override bool Equals(Object other) {
                Baz o = other as Baz;

                if (o == null) {
                    return false;
                }

                if (this.shared == null) {
                    if (o.shared != null) {
                        return false;
                    }
                } else {
                    if (!this.shared.Equals(o.shared)) {
                        return false;
                    }
                }

                return true;
            }

            public override int GetHashCode() {
                int result = 1;
                result = result * 31 + this.shared.GetHashCode();
                return result;
            }

            public override String ToString() {
                StringBuilder b = new StringBuilder();

                b.Append("Baz(");
                b.Append("shared=");
                b.Append(this.shared);
                b.Append(")");

                return b.ToString();
            }
        }
    }
}
