using System;

namespace Bar._2_0_0 {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Other {
    [Newtonsoft.Json.JsonProperty("name2", Required = Newtonsoft.Json.Required.DisallowNull)]
    public System.String name2 {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Other(
      [Newtonsoft.Json.JsonProperty("name2", Required = Newtonsoft.Json.Required.DisallowNull)] System.String name2
    ) {
      this.name2 = name2;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name2.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.name2.Equals(o.name2)) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("name2=");
      b.Append(this.name2);
      b.Append(")");

      return b.ToString();
    }
  }
}
