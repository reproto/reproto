using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Foo {
    /// <summary>
    /// The field.
    /// </summary>
    [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)]
    public System.String field {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Foo(
      [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)] System.String field
    ) {
      this.field = field;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.field.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Foo o = other as Foo;

      if (o == null) {
        return false;
      }

      if (!this.field.Equals(o.field)) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Foo");
      b.Append("(");
      b.Append("field=");
      b.Append(this.field);
      b.Append(")");

      return b.ToString();
    }
  }
}
