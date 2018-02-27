using System;

namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Tuple1.Json_Net_Converter))]
  public class Tuple1 {
    public System.String a {
      get;
    }
    public UInt64 b {
      get;
    }

    public Tuple1(
      System.String a,
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

    public override Boolean Equals(System.Object other) {
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

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

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

    public class Json_Net_Converter : Newtonsoft.Json.JsonConverter {
      public override Boolean CanConvert(System.Type objectType) {
        return objectType == typeof(Tuple1);
      }

      public override void WriteJson(Newtonsoft.Json.JsonWriter writer, System.Object obj, Newtonsoft.Json.JsonSerializer serializer) {
        Tuple1 o = (Tuple1)obj;
        Newtonsoft.Json.Linq.JArray array = new Newtonsoft.Json.Linq.JArray();
        array.Add(Newtonsoft.Json.Linq.JToken.FromObject(o.a, serializer));
        array.Add(Newtonsoft.Json.Linq.JToken.FromObject(o.b, serializer));
        array.WriteTo(writer);
      }

      public override System.Object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, System.Object existingValue, Newtonsoft.Json.JsonSerializer serializer) {
        Newtonsoft.Json.Linq.JArray array = Newtonsoft.Json.Linq.JArray.Load(reader);
        System.Collections.Generic.IEnumerator<Newtonsoft.Json.Linq.JToken> enumerator = array.GetEnumerator();
        if (!enumerator.MoveNext()) {
          throw new System.InvalidOperationException("expected more items in array");
        }
        System.String a = enumerator.Current.ToObject<System.String>(serializer);;
        if (!enumerator.MoveNext()) {
          throw new System.InvalidOperationException("expected more items in array");
        }
        UInt64 b = enumerator.Current.ToObject<UInt64>(serializer);;
        return new Tuple1(a, b);
      }
    }
  }
}
