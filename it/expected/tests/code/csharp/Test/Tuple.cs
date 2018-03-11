using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Text;

namespace Test {
  [JsonConverter(typeof(Tuple.Json_Net_Converter))]
  public class Tuple {
    public Tuple() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(Object other) {
      Tuple o = other as Tuple;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Tuple");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }

    public class Json_Net_Converter : JsonConverter {
      public override Boolean CanConvert(System.Type objectType) {
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
