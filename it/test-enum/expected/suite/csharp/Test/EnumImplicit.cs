
namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
  public enum EnumImplicit {
    [System.Runtime.Serialization.EnumMember(Value = "A")]
    A,
    [System.Runtime.Serialization.EnumMember(Value = "B")]
    B
  }
}
