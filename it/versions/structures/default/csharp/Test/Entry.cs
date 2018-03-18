using Foo._4_0_0;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Entry {
    [JsonProperty("thing")]
    public Thing thing {
      get;
    }

    [JsonConstructor]
    public Entry(
      [JsonProperty("thing")] Thing thing
    ) {
      this.thing = thing;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.thing.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this.thing == null) {
        if (o.thing != null) {
          return false;
        }
      } else {
        if (!this.thing.Equals(o.thing)) {
          return false;
        }
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("thing=");
      b.Append(this.thing);
      b.Append(")");

      return b.ToString();
    }
  }
}
