using JsonSubTypes;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonConverter(typeof(JsonSubtypes), "type")]
  [JsonSubtypes.KnownSubType(typeof(Interface.SubType), "SubType")]
  public abstract class Interface {
    [JsonProperty("type", Required = Required.DisallowNull)]
    private String TypeField {
      get;
    }

    public Interface(
      String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class SubType : Interface {
      [JsonConstructor]
      public SubType(
        [JsonProperty("type", Required = Required.DisallowNull)] String TypeField
      ) : base(TypeField) {
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        return result;
      }

      public override Boolean Equals(Object other) {
        SubType o = other as SubType;

        if (o == null) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("SubType");
        b.Append("(");
        b.Append(")");

        return b.ToString();
      }
    }
  }
}
