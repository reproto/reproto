using Newtonsoft.Json;
using System;
using System.Text;

namespace LowerSnake {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Value {
    [JsonProperty("foo_bar", Required = Required.DisallowNull)]
    public String fooBar {
      get;
    }

    [JsonConstructor]
    public Value(
      [JsonProperty("foo_bar", Required = Required.DisallowNull)] String fooBar
    ) {
      this.fooBar = fooBar;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.fooBar.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Value o = other as Value;

      if (o == null) {
        return false;
      }

      if (!this.fooBar.Equals(o.fooBar)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Value");
      b.Append("(");
      b.Append("foo_bar=");
      b.Append(this.fooBar);
      b.Append(")");

      return b.ToString();
    }
  }
}
