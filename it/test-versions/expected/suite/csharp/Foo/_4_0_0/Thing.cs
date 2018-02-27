using System;

namespace Foo._4_0_0 {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Thing {
    [Newtonsoft.Json.JsonProperty("name")]
    public System.String name {
      get;
    }
    [Newtonsoft.Json.JsonProperty("other")]
    public Bar._1_0_0.Other other {
      get;
    }
    [Newtonsoft.Json.JsonProperty("other2")]
    public Bar._2_0_0.Other other2 {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Thing(
      [Newtonsoft.Json.JsonProperty("name")] System.String name,
      [Newtonsoft.Json.JsonProperty("other")] Bar._1_0_0.Other other,
      [Newtonsoft.Json.JsonProperty("other2")] Bar._2_0_0.Other other2
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

    public override Boolean Equals(System.Object other) {
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

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

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
