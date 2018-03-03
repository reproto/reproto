using Newtonsoft.Json;
using System;
using System.Text;

namespace Bar._2_0_0 {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Other {
    [JsonProperty("name2", Required = Required.DisallowNull)]
    public String name2 {
      get;
    }

    [JsonConstructor]
    public Other(
      [JsonProperty("name2", Required = Required.DisallowNull)] String name2
    ) {
      this.name2 = name2;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name2.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Other o = other as Other;

      if (o == null) {
        return false;
      }

      if (!this.name2.Equals(o.name2)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Other");
      b.Append("(");
      b.Append("name2=");
      b.Append(this.name2);
      b.Append(")");

      return b.ToString();
    }
  }
}
