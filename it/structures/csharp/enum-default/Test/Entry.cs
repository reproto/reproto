using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Entry {
        [JsonProperty("explicit")]
        public EnumExplicit? _explicit {
            get;
        }

        [JsonProperty("implicit")]
        public EnumImplicit? _implicit {
            get;
        }

        [JsonProperty("enum_u32")]
        public EnumU32? enumU32 {
            get;
        }

        [JsonProperty("enum_u64")]
        public EnumU64? enumU64 {
            get;
        }

        [JsonProperty("enum_i32")]
        public EnumI32? enumI32 {
            get;
        }

        [JsonProperty("enum_i64")]
        public EnumI64? enumI64 {
            get;
        }

        [JsonConstructor]
        public Entry (
            [JsonProperty("explicit")] EnumExplicit? _explicit,
            [JsonProperty("implicit")] EnumImplicit? _implicit,
            [JsonProperty("enum_u32")] EnumU32? enumU32,
            [JsonProperty("enum_u64")] EnumU64? enumU64,
            [JsonProperty("enum_i32")] EnumI32? enumI32,
            [JsonProperty("enum_i64")] EnumI64? enumI64
        ) {
            this._explicit = _explicit;
            this._implicit = _implicit;
            this.enumU32 = enumU32;
            this.enumU64 = enumU64;
            this.enumI32 = enumI32;
            this.enumI64 = enumI64;
        }

        public override bool Equals(Object other) {
            Entry o = other as Entry;

            if (o == null) {
                return false;
            }

            if (!this._explicit.Equals(o._explicit)) {
                return false;
            }

            if (!this._implicit.Equals(o._implicit)) {
                return false;
            }

            if (!this.enumU32.Equals(o.enumU32)) {
                return false;
            }

            if (!this.enumU64.Equals(o.enumU64)) {
                return false;
            }

            if (!this.enumI32.Equals(o.enumI32)) {
                return false;
            }

            if (!this.enumI64.Equals(o.enumI64)) {
                return false;
            }

            return true;
        }

        public override int GetHashCode() {
            int result = 1;
            result = result * 31 + this._explicit.GetHashCode();
            result = result * 31 + this._implicit.GetHashCode();
            result = result * 31 + this.enumU32.GetHashCode();
            result = result * 31 + this.enumU64.GetHashCode();
            result = result * 31 + this.enumI32.GetHashCode();
            result = result * 31 + this.enumI64.GetHashCode();
            return result;
        }

        public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Entry(");
            b.Append("explicit=");
            b.Append(this._explicit);
            b.Append(", ");
            b.Append("implicit=");
            b.Append(this._implicit);
            b.Append(", ");
            b.Append("enum_u32=");
            b.Append(this.enumU32);
            b.Append(", ");
            b.Append("enum_u64=");
            b.Append(this.enumU64);
            b.Append(", ");
            b.Append("enum_i32=");
            b.Append(this.enumI32);
            b.Append(", ");
            b.Append("enum_i64=");
            b.Append(this.enumI64);
            b.Append(")");

            return b.ToString();
        }
    }
}
