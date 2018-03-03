using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Foo {
    /// <summary>
    /// The field.
    /// </summary>
    [JsonProperty("field", Required = Required.DisallowNull)]
    public String field {
      get;
    }

    [JsonConstructor]
    public Foo(
      [JsonProperty("field", Required = Required.DisallowNull)] String field
    ) {
      this.field = field;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.field.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Foo o = other as Foo;

      if (o == null) {
        return false;
      }

      if (!this.field.Equals(o.field)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Foo");
      b.Append("(");
      b.Append("field=");
      b.Append(this.field);
      b.Append(")");

      return b.ToString();
    }
  }
}
