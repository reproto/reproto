using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
    /// Explicitly assigned strings
    [JsonConverter(typeof(StringEnumConverter))]
    public enum EnumExplicit {
        [EnumMember(Value = "foo")]
        A,
        [EnumMember(Value = "bar")]
        B
    }
}
