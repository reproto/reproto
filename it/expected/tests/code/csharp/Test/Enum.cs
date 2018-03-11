using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
  [JsonConverter(typeof(StringEnumConverter))]
  public enum Enum {
    [EnumMember(Value = "Variant")]
    VARIANT
  }
}
