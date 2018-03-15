using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Other {
    [JsonProperty("a", Required = Required.DisallowNull)]
    public String a {
      get;
    }

    [JsonConstructor]
    public Other(
      [JsonProperty("a", Required = Required.DisallowNull)] String a
    ) {
      this.a = a;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.a.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.a.Equals(o.a)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("a=");
      b.Append(this.a);
      b.Append(")");

      return b.ToString();
    }
  }
}
