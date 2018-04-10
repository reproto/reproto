using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Text;

namespace Test {
  [JsonConverter(typeof(Untagged.Json_Net_Converter))]
  public abstract class Untagged {
    public Untagged() {
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class A : Untagged {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }
      [JsonProperty("shared_ignore")]
      public String sharedIgnore {
        get;
      }
      [JsonProperty("a", Required = Required.DisallowNull)]
      public String a {
        get;
      }
      [JsonProperty("b", Required = Required.DisallowNull)]
      public String b {
        get;
      }
      [JsonProperty("ignore")]
      public String ignore {
        get;
      }

      [JsonConstructor]
      public A(
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared,
        [JsonProperty("shared_ignore")] String sharedIgnore,
        [JsonProperty("a", Required = Required.DisallowNull)] String a,
        [JsonProperty("b", Required = Required.DisallowNull)] String b,
        [JsonProperty("ignore")] String ignore
      ) {
        this.shared = shared;
        this.sharedIgnore = sharedIgnore;
        this.a = a;
        this.b = b;
        this.ignore = ignore;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        result = result * 31 + this.sharedIgnore.GetHashCode();
        result = result * 31 + this.a.GetHashCode();
        result = result * 31 + this.b.GetHashCode();
        result = result * 31 + this.ignore.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        A o = other as A;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        if (this.sharedIgnore == null) {
          if (o.sharedIgnore != null) {
            return false;
          }
        } else {
          if (!this.sharedIgnore.Equals(o.sharedIgnore)) {
            return false;
          }
        }

        if (!this.a.Equals(o.a)) {
          return false;
        }

        if (!this.b.Equals(o.b)) {
          return false;
        }

        if (this.ignore == null) {
          if (o.ignore != null) {
            return false;
          }
        } else {
          if (!this.ignore.Equals(o.ignore)) {
            return false;
          }
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("A");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(", ");
        b.Append("shared_ignore=");
        b.Append(this.sharedIgnore);
        b.Append(", ");
        b.Append("a=");
        b.Append(this.a);
        b.Append(", ");
        b.Append("b=");
        b.Append(this.b);
        b.Append(", ");
        b.Append("ignore=");
        b.Append(this.ignore);
        b.Append(")");

        return b.ToString();
      }
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class B : Untagged {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }
      [JsonProperty("shared_ignore")]
      public String sharedIgnore {
        get;
      }
      [JsonProperty("a", Required = Required.DisallowNull)]
      public String a {
        get;
      }
      [JsonProperty("ignore")]
      public String ignore {
        get;
      }

      [JsonConstructor]
      public B(
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared,
        [JsonProperty("shared_ignore")] String sharedIgnore,
        [JsonProperty("a", Required = Required.DisallowNull)] String a,
        [JsonProperty("ignore")] String ignore
      ) {
        this.shared = shared;
        this.sharedIgnore = sharedIgnore;
        this.a = a;
        this.ignore = ignore;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        result = result * 31 + this.sharedIgnore.GetHashCode();
        result = result * 31 + this.a.GetHashCode();
        result = result * 31 + this.ignore.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        B o = other as B;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        if (this.sharedIgnore == null) {
          if (o.sharedIgnore != null) {
            return false;
          }
        } else {
          if (!this.sharedIgnore.Equals(o.sharedIgnore)) {
            return false;
          }
        }

        if (!this.a.Equals(o.a)) {
          return false;
        }

        if (this.ignore == null) {
          if (o.ignore != null) {
            return false;
          }
        } else {
          if (!this.ignore.Equals(o.ignore)) {
            return false;
          }
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("B");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(", ");
        b.Append("shared_ignore=");
        b.Append(this.sharedIgnore);
        b.Append(", ");
        b.Append("a=");
        b.Append(this.a);
        b.Append(", ");
        b.Append("ignore=");
        b.Append(this.ignore);
        b.Append(")");

        return b.ToString();
      }
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class C : Untagged {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }
      [JsonProperty("shared_ignore")]
      public String sharedIgnore {
        get;
      }
      [JsonProperty("b", Required = Required.DisallowNull)]
      public String b {
        get;
      }
      [JsonProperty("ignore")]
      public String ignore {
        get;
      }

      [JsonConstructor]
      public C(
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared,
        [JsonProperty("shared_ignore")] String sharedIgnore,
        [JsonProperty("b", Required = Required.DisallowNull)] String b,
        [JsonProperty("ignore")] String ignore
      ) {
        this.shared = shared;
        this.sharedIgnore = sharedIgnore;
        this.b = b;
        this.ignore = ignore;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        result = result * 31 + this.sharedIgnore.GetHashCode();
        result = result * 31 + this.b.GetHashCode();
        result = result * 31 + this.ignore.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        C o = other as C;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        if (this.sharedIgnore == null) {
          if (o.sharedIgnore != null) {
            return false;
          }
        } else {
          if (!this.sharedIgnore.Equals(o.sharedIgnore)) {
            return false;
          }
        }

        if (!this.b.Equals(o.b)) {
          return false;
        }

        if (this.ignore == null) {
          if (o.ignore != null) {
            return false;
          }
        } else {
          if (!this.ignore.Equals(o.ignore)) {
            return false;
          }
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("C");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(", ");
        b.Append("shared_ignore=");
        b.Append(this.sharedIgnore);
        b.Append(", ");
        b.Append("b=");
        b.Append(this.b);
        b.Append(", ");
        b.Append("ignore=");
        b.Append(this.ignore);
        b.Append(")");

        return b.ToString();
      }
    }

    public class Json_Net_Converter : JsonConverter {
      [ThreadStatic]
      private static Boolean _isInsideRead;
      public override Boolean CanWrite {
        get { return false; }
      }
      public override Boolean CanRead {
        get {
          return !_isInsideRead;
        }
      }

      public override Boolean CanConvert(System.Type objectType) {
        return false;
      }

      public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
        throw new InvalidOperationException("not implemented");
      }

      public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
        JObject o = JObject.Load(reader);

        if (o.ContainsKey("shared") && o.ContainsKey("a") && o.ContainsKey("b")) {
          _isInsideRead = true;
          try {
            return serializer.Deserialize(o.CreateReader(), typeof(A));
          } finally {
            _isInsideRead = false;
          }
        }

        if (o.ContainsKey("shared") && o.ContainsKey("a")) {
          _isInsideRead = true;
          try {
            return serializer.Deserialize(o.CreateReader(), typeof(B));
          } finally {
            _isInsideRead = false;
          }
        }

        if (o.ContainsKey("shared") && o.ContainsKey("b")) {
          _isInsideRead = true;
          try {
            return serializer.Deserialize(o.CreateReader(), typeof(C));
          } finally {
            _isInsideRead = false;
          }
        }

        throw new InvalidOperationException("no legal combination of fields");
      }
    }
  }
}
