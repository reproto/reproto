using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Entry {
    [JsonProperty("explicit")]
    public EnumExplicit? _explicit {
      get;
    }
    [JsonProperty("implicit")]
    public EnumImplicit? _implicit {
      get;
    }

    [JsonConstructor]
    public Entry(
      [JsonProperty("explicit")] EnumExplicit? _explicit,
      [JsonProperty("implicit")] EnumImplicit? _implicit
    ) {
      this._explicit = _explicit;
      this._implicit = _implicit;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this._explicit.GetHashCode();
      result = result * 31 + this._implicit.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (!this._explicit.Equals(o._explicit)) {
        return false;
      }

      if (!this._implicit.Equals(o._implicit)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("explicit=");
      b.Append(this._explicit);
      b.Append(", ");
      b.Append("implicit=");
      b.Append(this._implicit);
      b.Append(")");

      return b.ToString();
    }
  }
}
