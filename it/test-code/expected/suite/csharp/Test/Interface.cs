using System;

namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(JsonSubTypes.JsonSubtypes), "type")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(Interface.SubType), "SubType")]
  public abstract class Interface {
    [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)]
    private System.String TypeField {
      get;
    }

    public Interface(
      System.String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class SubType : Interface {
      [Newtonsoft.Json.JsonConstructor]
      public SubType(
        [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField
      ) : base(TypeField) {
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        return result;
      }

      public override Boolean Equals(System.Object other) {
        SubType o = other as SubType;

        if (o == null) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

        b.Append("SubType");
        b.Append("(");
        b.Append(")");

        return b.ToString();
      }
    }
  }
}
