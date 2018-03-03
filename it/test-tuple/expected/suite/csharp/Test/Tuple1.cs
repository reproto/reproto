using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Text;

namespace Test {
  [JsonConverter(typeof(Tuple1.Json_Net_Converter))]
  public class Tuple1 {
    public String a {
      get;
    }
    public UInt64 b {
      get;
    }

    public Tuple1(
      String a,
      UInt64 b
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
      Tuple1 o = other as Tuple1;

      if (o == null) {
        return false;
      }

      if (!this.a.Equals(o.a)) {
        return false;
      }

      if (this.b != o.b) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Tuple1");
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
        String a = enumerator.Current.ToObject<String>(serializer);;
        if (!enumerator.MoveNext()) {
          throw new InvalidOperationException("expected more items in array");
        }
        UInt64 b = enumerator.Current.ToObject<UInt64>(serializer);;
        return new Tuple1(a, b);
      }
    }
  }
}
