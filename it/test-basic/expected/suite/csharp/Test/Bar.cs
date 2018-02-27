using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Bar {
    /// <summary>
    /// The inner field.
    /// </summary>
    [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)]
    public Test.Bar.Inner field {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Bar(
      [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)] Test.Bar.Inner field
    ) {
      this.field = field;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.field.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Bar o = other as Bar;

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

      b.Append("Bar");
      b.Append("(");
      b.Append("field=");
      b.Append(this.field);
      b.Append(")");

      return b.ToString();
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class Inner {
      /// <summary>
      /// The field.
      /// </summary>
      [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String field {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public Inner(
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
        Inner o = other as Inner;

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

        b.Append("Inner");
        b.Append("(");
        b.Append("field=");
        b.Append(this.field);
        b.Append(")");

        return b.ToString();
      }
    }
  }
}
