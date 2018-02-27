using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    [Newtonsoft.Json.JsonConstructor]
    public Entry() {
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append(")");

      return b.ToString();
    }
  }
}
