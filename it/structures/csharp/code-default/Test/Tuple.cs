using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;

namespace Test {
    [JsonConverter(typeof(Tuple.Json_Net_Converter))]
    public class Tuple {

        [JsonConstructor]
        public Tuple () {}

        public override bool Equals(Object other) {
            Tuple o = other as Tuple;

            if (o == null) {
                return false;
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            return result;
        }

        public override String ToString() {
            return "Tuple()";
        }

        public class Json_Net_Converter : JsonConverter {
            public override bool CanConvert(System.Type objectType) {
                return objectType == typeof(Tuple);
            }

            public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
                Tuple o = (Tuple)obj;
                JArray array = new JArray();

                array.WriteTo(writer);
            }

            public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
                JArray array = JArray.Load(reader);
                IEnumerator<JToken> enumerator = array.GetEnumerator();

                return new Tuple();
            }
        }
    }
}
