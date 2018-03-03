using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
  [JsonConverter(typeof(StringEnumConverter))]
  public enum RootEnum {
    [EnumMember(Value = "Foo")]
    FOO
  }
}
