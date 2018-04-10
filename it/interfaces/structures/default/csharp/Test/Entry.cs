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
    [JsonProperty("required_fields")]
    public RequiredFields requiredFields {
      get;
    }

    [JsonConstructor]
    public Entry(
      [JsonProperty("tagged")] Tagged tagged,
      [JsonProperty("required_fields")] RequiredFields requiredFields
    ) {
      this.tagged = tagged;
      this.requiredFields = requiredFields;
    }

    public override Int32 GetHashCode() {
      Int32 result = 1;
      result = result * 31 + this.tagged.GetHashCode();
      result = result * 31 + this.requiredFields.GetHashCode();
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

      if (this.requiredFields == null) {
        if (o.requiredFields != null) {
          return false;
        }
      } else {
        if (!this.requiredFields.Equals(o.requiredFields)) {
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
      b.Append("required_fields=");
      b.Append(this.requiredFields);
      b.Append(")");

      return b.ToString();
    }
  }
}
