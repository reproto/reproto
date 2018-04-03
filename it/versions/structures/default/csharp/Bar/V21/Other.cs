using Newtonsoft.Json;
using System;
using System.Text;

namespace Bar.V21 {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Other {
    [JsonProperty("name21", Required = Required.DisallowNull)]
    public String name21 {
      get;
    }

    [JsonConstructor]
    public Other(
      [JsonProperty("name21", Required = Required.DisallowNull)] String name21
    ) {
      this.name21 = name21;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name21.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.name21.Equals(o.name21)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("name21=");
      b.Append(this.name21);
      b.Append(")");

      return b.ToString();
    }
  }
}
