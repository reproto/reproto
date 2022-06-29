using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System.Runtime.Serialization;

namespace Test {
    /// Variants with long names.
    [JsonConverter(typeof(StringEnumConverter))]
    public enum EnumLongNames {
        [EnumMember(Value = "FooBar")]
        FOO_BAR,
        [EnumMember(Value = "Baz")]
        BAZ
    }
}
