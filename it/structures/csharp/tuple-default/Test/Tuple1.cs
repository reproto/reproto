using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Text;

namespace Test {
    /// Tuple containing primitive.
    [JsonConverter(typeof(Tuple1.Json_Net_Converter))]
    public class Tuple1 {
        [JsonProperty("a")]
        public String a {
            get;
        }

        [JsonProperty("b")]
        public ulong b {
            get;
        }

        [JsonConstructor]
        public Tuple1 (
            [JsonProperty("a")] String a,
            [JsonProperty("b")] ulong b
        ) {
            this.a = a;
            this.b = b;
        }

        public override bool Equals(Object other) {
            Tuple1 o = other as Tuple1;

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

            if (!this.b.Equals(o.b)) {
                return false;
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

            b.Append("Tuple1(");
            b.Append("a=");
            b.Append(this.a);
            b.Append(", ");
            b.Append("b=");
            b.Append(this.b);
            b.Append(")");

            return b.ToString();
        }

        public class Json_Net_Converter : JsonConverter {
            public override bool CanConvert(System.Type objectType) {
                return objectType == typeof(Tuple1);
            }

            public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
                Tuple1 o = (Tuple1)obj;
                JArray array = new JArray();

                array.Add(JToken.FromObject(o.a, serializer));

                array.Add(JToken.FromObject(o.b, serializer));

                array.WriteTo(writer);
            }

            public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
                JArray array = JArray.Load(reader);
                IEnumerator<JToken> enumerator = array.GetEnumerator();

                if (!enumerator.MoveNext()) {
                    throw new InvalidOperationException("expected more items in array");
                }

                String a = enumerator.Current.ToObject<String>(serializer);

                if (!enumerator.MoveNext()) {
                    throw new InvalidOperationException("expected more items in array");
                }

                ulong b = enumerator.Current.ToObject<ulong>(serializer);

                return new Tuple1(a, b);
            }
        }
    }
}
