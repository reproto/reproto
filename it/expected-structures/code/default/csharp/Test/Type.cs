using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Type {
    [JsonConstructor]
    public Type() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(Object other) {
      Type o = other as Type;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Type");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }
  }
}
