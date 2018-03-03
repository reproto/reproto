using JsonSubTypes;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Runtime.Serialization;
using System.Text;

namespace Test {
  [JsonConverter(typeof(JsonSubtypes), "type")]
  [JsonSubtypes.KnownSubType(typeof(RootInterface.Foo), "Foo")]
  public abstract class RootInterface {
    [JsonProperty("type", Required = Required.DisallowNull)]
    private String TypeField {
      get;
    }

    public RootInterface(
      String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Foo : RootInterface {
      [JsonConstructor]
      public Foo(
        [JsonProperty("type", Required = Required.DisallowNull)] String TypeField
      ) : base(TypeField) {
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        return result;
      }

      public override Boolean Equals(Object other) {
        Foo o = other as Foo;

        if (o == null) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("Foo");
        b.Append("(");
        b.Append(")");

        return b.ToString();
      }

      [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
      public class NestedType {
        [JsonConstructor]
        public NestedType() {
        }

        public override Int32 GetHashCode() {
          Int32 result = 1;
          return result;
        }

        public override Boolean Equals(Object other) {
          NestedType o = other as NestedType;

          if (o == null) {
            return false;
          }

          return true;
        }

        public override String ToString() {
          StringBuilder b = new StringBuilder();

          b.Append("NestedType");
          b.Append("(");
          b.Append(")");

          return b.ToString();
        }
      }

      [JsonConverter(typeof(JsonSubtypes), "type")]
      [JsonSubtypes.KnownSubType(typeof(NestedInterface.NestedFoo), "NestedFoo")]
      public abstract class NestedInterface {
        [JsonProperty("type", Required = Required.DisallowNull)]
        private String TypeField {
          get;
        }

        public NestedInterface(
          String TypeField
        ) {
          this.TypeField = TypeField;
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class NestedFoo : NestedInterface {
          [JsonConstructor]
          public NestedFoo(
            [JsonProperty("type", Required = Required.DisallowNull)] String TypeField
          ) : base(TypeField) {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(Object other) {
            NestedFoo o = other as NestedFoo;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("NestedFoo");
            b.Append("(");
            b.Append(")");

            return b.ToString();
          }

          [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
          public class Nested {
            [JsonConstructor]
            public Nested() {
            }

            public override Int32 GetHashCode() {
              Int32 result = 1;
              return result;
            }

            public override Boolean Equals(Object other) {
              Nested o = other as Nested;

              if (o == null) {
                return false;
              }

              return true;
            }

            public override String ToString() {
              StringBuilder b = new StringBuilder();

              b.Append("Nested");
              b.Append("(");
              b.Append(")");

              return b.ToString();
            }
          }
        }
      }

      [JsonConverter(typeof(StringEnumConverter))]
      public enum NestedEnum {
        [EnumMember(Value = "Foo")]
        FOO
      }

      [JsonConverter(typeof(NestedTuple.Json_Net_Converter))]
      public class NestedTuple {
        public NestedTuple() {
        }

        public override Int32 GetHashCode() {
          Int32 result = 1;
          return result;
        }

        public override Boolean Equals(Object other) {
          NestedTuple o = other as NestedTuple;

          if (o == null) {
            return false;
          }

          return true;
        }

        public override String ToString() {
          StringBuilder b = new StringBuilder();

          b.Append("NestedTuple");
          b.Append("(");
          b.Append(")");

          return b.ToString();
        }

        public class Json_Net_Converter : JsonConverter {
          public override Boolean CanConvert(System.Type objectType) {
            return objectType == typeof(NestedTuple);
          }

          public override void WriteJson(JsonWriter writer, Object obj, JsonSerializer serializer) {
            NestedTuple o = (NestedTuple)obj;
            JArray array = new JArray();
            array.WriteTo(writer);
          }

          public override Object ReadJson(JsonReader reader, System.Type objectType, Object existingValue, JsonSerializer serializer) {
            JArray array = JArray.Load(reader);
            IEnumerator<JToken> enumerator = array.GetEnumerator();
            return new NestedTuple();
          }
        }

        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Nested {
          [JsonConstructor]
          public Nested() {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(Object other) {
            Nested o = other as Nested;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Nested");
            b.Append("(");
            b.Append(")");

            return b.ToString();
          }
        }
      }

      public abstract class NestedService {
        [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
        public class Nested {
          [JsonConstructor]
          public Nested() {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(Object other) {
            Nested o = other as Nested;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override String ToString() {
            StringBuilder b = new StringBuilder();

            b.Append("Nested");
            b.Append("(");
            b.Append(")");

            return b.ToString();
          }
        }
      }
    }
  }
}
