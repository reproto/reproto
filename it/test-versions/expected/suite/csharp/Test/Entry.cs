using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    [Newtonsoft.Json.JsonProperty("thing")]
    public Foo._4_0_0.Thing thing {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Entry(
      [Newtonsoft.Json.JsonProperty("thing")] Foo._4_0_0.Thing thing
    ) {
      this.thing = thing;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.thing.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
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

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("thing=");
      b.Append(this.thing);
      b.Append(")");

      return b.ToString();
    }
  }
}
