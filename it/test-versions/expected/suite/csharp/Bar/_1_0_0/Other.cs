using System;

namespace Bar._1_0_0 {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Other {
    [Newtonsoft.Json.JsonProperty("name", Required = Newtonsoft.Json.Required.DisallowNull)]
    public System.String name {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Other(
      [Newtonsoft.Json.JsonProperty("name", Required = Newtonsoft.Json.Required.DisallowNull)] System.String name
    ) {
      this.name = name;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.name.Equals(o.name)) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("name=");
      b.Append(this.name);
      b.Append(")");

      return b.ToString();
    }
  }
}
