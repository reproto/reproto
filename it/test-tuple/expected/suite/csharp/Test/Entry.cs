using System;

namespace Test {
  [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
  public class Entry {
    [Newtonsoft.Json.JsonProperty("tuple1")]
    public Test.Tuple1 tuple1 {
      get;
    }
    [Newtonsoft.Json.JsonProperty("tuple2")]
    public Test.Tuple2 tuple2 {
      get;
    }

    [Newtonsoft.Json.JsonConstructor]
    public Entry(
      [Newtonsoft.Json.JsonProperty("tuple1")] Test.Tuple1 tuple1,
      [Newtonsoft.Json.JsonProperty("tuple2")] Test.Tuple2 tuple2
    ) {
      this.tuple1 = tuple1;
      this.tuple2 = tuple2;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.tuple1.GetHashCode();
      result = result * 31 + this.tuple2.GetHashCode();
      return result;
    }

    public override Boolean Equals(System.Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this.tuple1 == null) {
        if (o.tuple1 != null) {
          return false;
        }
      } else {
        if (!this.tuple1.Equals(o.tuple1)) {
          return false;
        }
      }

      if (this.tuple2 == null) {
        if (o.tuple2 != null) {
          return false;
        }
      } else {
        if (!this.tuple2.Equals(o.tuple2)) {
          return false;
        }
      }

      return true;
    }

    public override System.String ToString() {
      System.Text.StringBuilder b = new System.Text.StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("tuple1=");
      b.Append(this.tuple1);
      b.Append(", ");
      b.Append("tuple2=");
      b.Append(this.tuple2);
      b.Append(")");

      return b.ToString();
    }
  }
}
