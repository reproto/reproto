using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
  [JsonConverter(typeof(StringEnumConverter))]
  public enum EnumImplicit {
    [EnumMember(Value = "A")]
    A,
    [EnumMember(Value = "B")]
    B
  }
}
