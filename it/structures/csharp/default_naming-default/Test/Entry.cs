using LowerCamel;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("lower_camel")]
        public Value lowerCamel {
            get;
        }

        [JsonProperty("lower_snake")]
        public LowerSnake.Value lowerSnake {
            get;
        }

        [JsonProperty("upper_camel")]
        public UpperCamel.Value upperCamel {
            get;
        }

        [JsonProperty("upper_snake")]
        public UpperSnake.Value upperSnake {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("lower_camel")] Value lowerCamel,
            [JsonProperty("lower_snake")] LowerSnake.Value lowerSnake,
            [JsonProperty("upper_camel")] UpperCamel.Value upperCamel,
            [JsonProperty("upper_snake")] UpperSnake.Value upperSnake
        ) {
            this.lowerCamel = lowerCamel;
            this.lowerSnake = lowerSnake;
            this.upperCamel = upperCamel;
            this.upperSnake = upperSnake;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (this.lowerCamel == null) {
                if (o.lowerCamel != null) {
                    return false;
                }
            } else {
                if (!this.lowerCamel.Equals(o.lowerCamel)) {
                    return false;
                }
            }

            if (this.lowerSnake == null) {
                if (o.lowerSnake != null) {
                    return false;
                }
            } else {
                if (!this.lowerSnake.Equals(o.lowerSnake)) {
                    return false;
                }
            }

            if (this.upperCamel == null) {
                if (o.upperCamel != null) {
                    return false;
                }
            } else {
                if (!this.upperCamel.Equals(o.upperCamel)) {
                    return false;
                }
            }

            if (this.upperSnake == null) {
                if (o.upperSnake != null) {
                    return false;
                }
            } else {
                if (!this.upperSnake.Equals(o.upperSnake)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.lowerCamel.GetHashCode();
            result = result * 31 + this.lowerSnake.GetHashCode();
            result = result * 31 + this.upperCamel.GetHashCode();
            result = result * 31 + this.upperSnake.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("lower_camel=");
            b.Append(this.lowerCamel);
            b.Append(", ");
            b.Append("lower_snake=");
            b.Append(this.lowerSnake);
            b.Append(", ");
            b.Append("upper_camel=");
            b.Append(this.upperCamel);
            b.Append(", ");
            b.Append("upper_snake=");
            b.Append(this.upperSnake);
            b.Append(")");

            return b.ToString();
        }
    }
}
