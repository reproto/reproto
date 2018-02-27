using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Type {
    [Newtonsoft.Json.JsonConstructor]
    public Type() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Type o = other as Type;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Type");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }
  }
}
