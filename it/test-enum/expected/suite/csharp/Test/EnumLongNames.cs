
namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
  public enum EnumLongNames {
    [System.Runtime.Serialization.EnumMember(Value = "FooBar")]
    FOO_BAR,
    [System.Runtime.Serialization.EnumMember(Value = "Baz")]
    BAZ
  }
}
