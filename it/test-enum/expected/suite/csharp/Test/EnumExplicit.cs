
namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
  public enum EnumExplicit {
    [System.Runtime.Serialization.EnumMember(Value = "foo")]
    A,
    [System.Runtime.Serialization.EnumMember(Value = "bar")]
    B
  }
}
