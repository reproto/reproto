using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
  public class Entry {
    [JsonProperty("tagged")]
    public Tagged tagged {
      get;
    }
    [JsonProperty("untagged")]
    public Untagged untagged {
      get;
    }

    [JsonConstructor]
    public Entry(
      [JsonProperty("tagged")] Tagged tagged,
      [JsonProperty("untagged")] Untagged untagged
    ) {
      this.tagged = tagged;
      this.untagged = untagged;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.tagged.GetHashCode();
      result = result * 31 + this.untagged.GetHashCode();
      return result;
    }

    public override Boolean Equals(Object other) {
      Entry o = other as Entry;

      if (o == null) {
        return false;
      }

      if (this.tagged == null) {
        if (o.tagged != null) {
          return false;
        }
      } else {
        if (!this.tagged.Equals(o.tagged)) {
          return false;
        }
      }

      if (this.untagged == null) {
        if (o.untagged != null) {
          return false;
        }
      } else {
        if (!this.untagged.Equals(o.untagged)) {
          return false;
        }
      }

      return true;
    }

    public override String ToString() {
      StringBuilder b = new StringBuilder();

      b.Append("Entry");
      b.Append("(");
      b.Append("tagged=");
      b.Append(this.tagged);
      b.Append(", ");
      b.Append("untagged=");
      b.Append(this.untagged);
      b.Append(")");

      return b.ToString();
    }
  }
}
