using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("boolean_type")]
        public bool? booleanType {
            get;
        }

        [JsonProperty("string_type")]
        public String stringType {
            get;
        }

        [JsonProperty("datetime_type")]
        public DateTime? datetimeType {
            get;
        }

        [JsonProperty("unsigned_32")]
        public uint? unsigned32 {
            get;
        }

        [JsonProperty("unsigned_64")]
        public ulong? unsigned64 {
            get;
        }

        [JsonProperty("signed_32")]
        public int? signed32 {
            get;
        }

        [JsonProperty("signed_64")]
        public long? signed64 {
            get;
        }

        [JsonProperty("float_type")]
        public float? floatType {
            get;
        }

        [JsonProperty("double_type")]
        public double? doubleType {
            get;
        }

        [JsonProperty("bytes_type")]
        public byte[] bytesType {
            get;
        }

        [JsonProperty("any_type")]
        public Object anyType {
            get;
        }

        [JsonProperty("array_type")]
        public List<Entry> arrayType {
            get;
        }

        [JsonProperty("array_of_array_type")]
        public List<List<Entry>> arrayOfArrayType {
            get;
        }

        [JsonProperty("map_type")]
        public Dictionary<String, Entry> mapType {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("boolean_type")] bool? booleanType,
            [JsonProperty("string_type")] String stringType,
            [JsonProperty("datetime_type")] DateTime? datetimeType,
            [JsonProperty("unsigned_32")] uint? unsigned32,
            [JsonProperty("unsigned_64")] ulong? unsigned64,
            [JsonProperty("signed_32")] int? signed32,
            [JsonProperty("signed_64")] long? signed64,
            [JsonProperty("float_type")] float? floatType,
            [JsonProperty("double_type")] double? doubleType,
            [JsonProperty("bytes_type")] byte[] bytesType,
            [JsonProperty("any_type")] Object anyType,
            [JsonProperty("array_type")] List<Entry> arrayType,
            [JsonProperty("array_of_array_type")] List<List<Entry>> arrayOfArrayType,
            [JsonProperty("map_type")] Dictionary<String, Entry> mapType
        ) {
            this.booleanType = booleanType;
            this.stringType = stringType;
            this.datetimeType = datetimeType;
            this.unsigned32 = unsigned32;
            this.unsigned64 = unsigned64;
            this.signed32 = signed32;
            this.signed64 = signed64;
            this.floatType = floatType;
            this.doubleType = doubleType;
            this.bytesType = bytesType;
            this.anyType = anyType;
            this.arrayType = arrayType;
            this.arrayOfArrayType = arrayOfArrayType;
            this.mapType = mapType;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (!this.booleanType.Equals(o.booleanType)) {
                return false;
            }

            if (this.stringType == null) {
                if (o.stringType != null) {
                    return false;
                }
            } else {
                if (!this.stringType.Equals(o.stringType)) {
                    return false;
                }
            }

            if (!this.datetimeType.Equals(o.datetimeType)) {
                return false;
            }

            if (!this.unsigned32.Equals(o.unsigned32)) {
                return false;
            }

            if (!this.unsigned64.Equals(o.unsigned64)) {
                return false;
            }

            if (!this.signed32.Equals(o.signed32)) {
                return false;
            }

            if (!this.signed64.Equals(o.signed64)) {
                return false;
            }

            if (!this.floatType.Equals(o.floatType)) {
                return false;
            }

            if (!this.doubleType.Equals(o.doubleType)) {
                return false;
            }

            if (this.bytesType == null) {
                if (o.bytesType != null) {
                    return false;
                }
            } else {
                if (!this.bytesType.Equals(o.bytesType)) {
                    return false;
                }
            }

            if (this.anyType == null) {
                if (o.anyType != null) {
                    return false;
                }
            } else {
                if (!this.anyType.Equals(o.anyType)) {
                    return false;
                }
            }

            if (this.arrayType == null) {
                if (o.arrayType != null) {
                    return false;
                }
            } else {
                if (!this.arrayType.Equals(o.arrayType)) {
                    return false;
                }
            }

            if (this.arrayOfArrayType == null) {
                if (o.arrayOfArrayType != null) {
                    return false;
                }
            } else {
                if (!this.arrayOfArrayType.Equals(o.arrayOfArrayType)) {
                    return false;
                }
            }

            if (this.mapType == null) {
                if (o.mapType != null) {
                    return false;
                }
            } else {
                if (!this.mapType.Equals(o.mapType)) {
                    return false;
                }
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this.booleanType.GetHashCode();
            result = result * 31 + this.stringType.GetHashCode();
            result = result * 31 + this.datetimeType.GetHashCode();
            result = result * 31 + this.unsigned32.GetHashCode();
            result = result * 31 + this.unsigned64.GetHashCode();
            result = result * 31 + this.signed32.GetHashCode();
            result = result * 31 + this.signed64.GetHashCode();
            result = result * 31 + this.floatType.GetHashCode();
            result = result * 31 + this.doubleType.GetHashCode();
            result = result * 31 + this.bytesType.GetHashCode();
            result = result * 31 + this.anyType.GetHashCode();
            result = result * 31 + this.arrayType.GetHashCode();
            result = result * 31 + this.arrayOfArrayType.GetHashCode();
            result = result * 31 + this.mapType.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("boolean_type=");
            b.Append(this.booleanType);
            b.Append(", ");
            b.Append("string_type=");
            b.Append(this.stringType);
            b.Append(", ");
            b.Append("datetime_type=");
            b.Append(this.datetimeType);
            b.Append(", ");
            b.Append("unsigned_32=");
            b.Append(this.unsigned32);
            b.Append(", ");
            b.Append("unsigned_64=");
            b.Append(this.unsigned64);
            b.Append(", ");
            b.Append("signed_32=");
            b.Append(this.signed32);
            b.Append(", ");
            b.Append("signed_64=");
            b.Append(this.signed64);
            b.Append(", ");
            b.Append("float_type=");
            b.Append(this.floatType);
            b.Append(", ");
            b.Append("double_type=");
            b.Append(this.doubleType);
            b.Append(", ");
            b.Append("bytes_type=");
            b.Append(this.bytesType);
            b.Append(", ");
            b.Append("any_type=");
            b.Append(this.anyType);
            b.Append(", ");
            b.Append("array_type=");
            b.Append(this.arrayType);
            b.Append(", ");
            b.Append("array_of_array_type=");
            b.Append(this.arrayOfArrayType);
            b.Append(", ");
            b.Append("map_type=");
            b.Append(this.mapType);
            b.Append(")");

            return b.ToString();
        }
    }
}
