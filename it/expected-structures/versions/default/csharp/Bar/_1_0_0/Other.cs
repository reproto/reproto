using Newtonsoft.Json;
using System;
using System.Text;

namespace Bar._1_0_0 {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Other {
    [JsonProperty("name", Required = Required.DisallowNull)]
    public String name {
      get;
    }

    [JsonConstructor]
    public Other(
      [JsonProperty("name", Required = Required.DisallowNull)] String name
    ) {
      this.name = name;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.name.Equals(o.name)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("name=");
      b.Append(this.name);
      b.Append(")");

      return b.ToString();
    }
  }
}
