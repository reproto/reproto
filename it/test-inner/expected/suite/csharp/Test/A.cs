using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class A {
    [JsonProperty("b", Required = Required.DisallowNull)]
    public A.B b {
      get;
    }

    [JsonConstructor]
    public A(
      [JsonProperty("b", Required = Required.DisallowNull)] A.B b
    ) {
      this.b = b;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.b.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      A o = other as A;

      if (o == null) {
        return false;
      }

      if (!this.b.Equals(o.b)) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("A");
      b.Append("(");
      b.Append("b=");
      b.Append(this.b);
      b.Append(")");

      return b.ToString();
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class B {
      [JsonProperty("field", Required = Required.DisallowNull)]
      public String field {
        get;
      }

      [JsonConstructor]
      public B(
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
        B o = other as B;

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

        b.Append("B");
        b.Append("(");
        b.Append("field=");
        b.Append(this.field);
        b.Append(")");

        return b.ToString();
      }
    }
  }
}
