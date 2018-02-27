using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    [Newtonsoft.Json.JsonProperty("explicit")]
    public Test.EnumExplicit _explicit {
      get;
    }
    [Newtonsoft.Json.JsonProperty("implicit")]
    public Test.EnumImplicit _implicit {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Entry(
      [Newtonsoft.Json.JsonProperty("explicit")] Test.EnumExplicit _explicit,
      [Newtonsoft.Json.JsonProperty("implicit")] Test.EnumImplicit _implicit
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

    public override Boolean Equals(System.Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this._explicit == null) {
        if (o._explicit != null) {
          return false;
        }
      } else {
        if (!this._explicit.Equals(o._explicit)) {
          return false;
        }
      }

      if (this._implicit == null) {
        if (o._implicit != null) {
          return false;
        }
      } else {
        if (!this._implicit.Equals(o._implicit)) {
          return false;
        }
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

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
