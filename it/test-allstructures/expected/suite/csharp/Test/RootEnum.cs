
namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
  public enum RootEnum {
    [System.Runtime.Serialization.EnumMember(Value = "Foo")]
    FOO
  }
}
