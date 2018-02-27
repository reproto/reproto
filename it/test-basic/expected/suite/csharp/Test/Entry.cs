using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    /// <summary>
    /// The foo field.
    /// </summary>
    [Newtonsoft.Json.JsonProperty("foo")]
    public Test.Foo foo {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Entry(
      [Newtonsoft.Json.JsonProperty("foo")] Test.Foo foo
    ) {
      this.foo = foo;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.foo.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this.foo == null) {
        if (o.foo != null) {
          return false;
        }
      } else {
        if (!this.foo.Equals(o.foo)) {
          return false;
        }
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("foo=");
      b.Append(this.foo);
      b.Append(")");

      return b.ToString();
    }
  }
}
