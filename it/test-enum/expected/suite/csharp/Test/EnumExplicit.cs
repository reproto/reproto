using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
  [JsonConverter(typeof(StringEnumConverter))]
  public enum EnumExplicit {
    [EnumMember(Value = "foo")]
    A,
    [EnumMember(Value = "bar")]
    B
  }
}
