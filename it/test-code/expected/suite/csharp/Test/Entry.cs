using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Entry {
    [JsonConstructor]
    public Entry() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }
  }
}
