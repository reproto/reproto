using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class A {
    [Newtonsoft.Json.JsonProperty("b", Required = Newtonsoft.Json.Required.DisallowNull)]
    public Test.A.B b {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public A(
      [Newtonsoft.Json.JsonProperty("b", Required = Newtonsoft.Json.Required.DisallowNull)] Test.A.B b
    ) {
      this.b = b;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.b.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      A o = other as A;

      if (o == null) {
        return false;
      }

      if (!this.b.Equals(o.b)) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("A");
      b.Append("(");
      b.Append("b=");
      b.Append(this.b);
      b.Append(")");

      return b.ToString();
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class B {
      [Newtonsoft.Json.JsonProperty("field", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String field {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public B(
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
        B o = other as B;

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
