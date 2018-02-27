using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Other {
    [Newtonsoft.Json.JsonProperty("a", Required = Newtonsoft.Json.Required.DisallowNull)]
    public System.String a {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Other(
      [Newtonsoft.Json.JsonProperty("a", Required = Newtonsoft.Json.Required.DisallowNull)] System.String a
    ) {
      this.a = a;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.a.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.a.Equals(o.a)) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("a=");
      b.Append(this.a);
      b.Append(")");

      return b.ToString();
    }
  }
}
