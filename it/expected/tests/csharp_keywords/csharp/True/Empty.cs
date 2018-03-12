using Newtonsoft.Json;
using System;
using System.Text;

namespace True {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Empty {
    [JsonConstructor]
    public Empty() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(Object other) {
      Empty o = other as Empty;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Empty");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }
  }
}
