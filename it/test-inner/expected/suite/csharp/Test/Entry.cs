using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    [Newtonsoft.Json.JsonProperty("a")]
    public Test.A a {
      get;
    }
    [Newtonsoft.Json.JsonProperty("b")]
    public Test.A.B b {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Entry(
      [Newtonsoft.Json.JsonProperty("a")] Test.A a,
      [Newtonsoft.Json.JsonProperty("b")] Test.A.B b
    ) {
      this.a = a;
      this.b = b;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.a.GetHashCode();
      result = result * 31 + this.b.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this.a == null) {
        if (o.a != null) {
          return false;
        }
      } else {
        if (!this.a.Equals(o.a)) {
          return false;
        }
      }

      if (this.b == null) {
        if (o.b != null) {
          return false;
        }
      } else {
        if (!this.b.Equals(o.b)) {
          return false;
        }
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("a=");
      b.Append(this.a);
      b.Append(", ");
      b.Append("b=");
      b.Append(this.b);
      b.Append(")");

      return b.ToString();
    }
  }
}
