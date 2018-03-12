using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Text;

namespace Test {
  [JsonConverter(typeof(Tuple2.Json_Net_Converter))]
  public class Tuple2 {
    public String a {
      get;
    }
    public Other b {
      get;
    }

    public Tuple2(
      String a,
      Other b
    ) {
      this.a = a;
      this.b = b;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.a.GetHashCode();
      result = result * 31 + this.b.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Tuple2 o = other as Tuple2;

      if (o == null) {
        return false;
      }

      if (!this.a.Equals(o.a)) {
        return false;
      }

      if (!this.b.Equals(o.b)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Tuple2");
      b.Append("(");
      b.Append("a=");
      b.Append(this.a);
      b.Append(", ");
      b.Append("b=");
      b.Append(this.b);
      b.Append(")");

      return b.ToString();
    }

    public class Json_Net_Converter : JsonConverter {
      public override Boolean CanConvert(System.Type objectType) {
        return objectType == typeof(Tuple2);
      }

      public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
        Tuple2 o = (Tuple2)obj;
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
        String a = enumerator.Current.ToObject<String>(serializer);;
        if (!enumerator.MoveNext()) {
          throw new InvalidOperationException("expected more items in array");
        }
        Other b = enumerator.Current.ToObject<Other>(serializer);;
        return new Tuple2(a, b);
      }
    }
  }
}
