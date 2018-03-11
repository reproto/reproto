using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Bar {
    /// <summary>
    /// The inner field.
    /// </summary>
    [JsonProperty("field", Required = Required.DisallowNull)]
    public Bar.Inner field {
      get;
    }

    [JsonConstructor]
    public Bar(
      [JsonProperty("field", Required = Required.DisallowNull)] Bar.Inner field
    ) {
      this.field = field;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.field.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Bar o = other as Bar;

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

      b.Append("Bar");
      b.Append("(");
      b.Append("field=");
      b.Append(this.field);
      b.Append(")");

      return b.ToString();
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Inner {
      /// <summary>
      /// The field.
      /// </summary>
      [JsonProperty("field", Required = Required.DisallowNull)]
      public String field {
        get;
      }

      [JsonConstructor]
      public Inner(
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
        Inner o = other as Inner;

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
