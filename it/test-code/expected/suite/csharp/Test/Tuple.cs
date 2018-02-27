using System;

namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Tuple.Json_Net_Converter))]
  public class Tuple {
    public Tuple() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Tuple o = other as Tuple;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Tuple");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }

    public class Json_Net_Converter : Newtonsoft.Json.JsonConverter {
      public override Boolean CanConvert(System.Type objectType) {
        return objectType == typeof(Tuple);
      }

      public override void WriteJson(Newtonsoft.Json.JsonWriter writer, System.Object obj, Newtonsoft.Json.JsonSerializer serializer) {
        Tuple o = (Tuple)obj;
        Newtonsoft.Json.Linq.JArray array = new Newtonsoft.Json.Linq.JArray();
        array.WriteTo(writer);
      }

      public override System.Object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, System.Object existingValue, Newtonsoft.Json.JsonSerializer serializer) {
        Newtonsoft.Json.Linq.JArray array = Newtonsoft.Json.Linq.JArray.Load(reader);
        System.Collections.Generic.IEnumerator<Newtonsoft.Json.Linq.JToken> enumerator = array.GetEnumerator();
        return new Tuple();
      }
    }
  }
}
