using Bar._1_0_0;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Foo._4_0_0 {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Thing {
    [JsonProperty("name")]
    public String name {
      get;
    }
    [JsonProperty("other")]
    public Other other {
      get;
    }
    [JsonProperty("other2")]
    public Bar._2_0_0.Other other2 {
      get;
    }

    [JsonConstructor]
    public Thing(
      [JsonProperty("name")] String name,
      [JsonProperty("other")] Other other,
      [JsonProperty("other2")] Bar._2_0_0.Other other2
    ) {
      this.name = name;
      this.other = other;
      this.other2 = other2;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.name.GetHashCode();
      result = result * 31 + this.other.GetHashCode();
      result = result * 31 + this.other2.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Thing o = other as Thing;

      if (o == null) {
        return false;
      }

      if (this.name == null) {
        if (o.name != null) {
          return false;
        }
      } else {
        if (!this.name.Equals(o.name)) {
          return false;
        }
      }

      if (this.other == null) {
        if (o.other != null) {
          return false;
        }
      } else {
        if (!this.other.Equals(o.other)) {
          return false;
        }
      }

      if (this.other2 == null) {
        if (o.other2 != null) {
          return false;
        }
      } else {
        if (!this.other2.Equals(o.other2)) {
          return false;
        }
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Thing");
      b.Append("(");
      b.Append("name=");
      b.Append(this.name);
      b.Append(", ");
      b.Append("other=");
      b.Append(this.other);
      b.Append(", ");
      b.Append("other2=");
      b.Append(this.other2);
      b.Append(")");

      return b.ToString();
    }
  }
}
