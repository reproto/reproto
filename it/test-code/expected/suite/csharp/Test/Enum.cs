
namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
  public enum Enum {
    [System.Runtime.Serialization.EnumMember(Value = "Variant")]
    VARIANT
  }
}
