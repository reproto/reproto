using System;

namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(JsonSubTypes.JsonSubtypes), "type")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(RootInterface.Foo), "Foo")]
  public abstract class RootInterface {
    [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)]
    private System.String TypeField {
      get;
    }

    public RootInterface(
      System.String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class Foo : RootInterface {
      [Newtonsoft.Json.JsonConstructor]
      public Foo(
        [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField
      ) : base(TypeField) {
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        return result;
      }

      public override Boolean Equals(System.Object other) {
        Foo o = other as Foo;

        if (o == null) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

        b.Append("Foo");
        b.Append("(");
        b.Append(")");

        return b.ToString();
      }

      [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
      public class NestedType {
        [Newtonsoft.Json.JsonConstructor]
        public NestedType() {
        }

        public override Int32 GetHashCode() {
          Int32 result = 1;
          return result;
        }

        public override Boolean Equals(System.Object other) {
          NestedType o = other as NestedType;

          if (o == null) {
            return false;
          }

          return true;
        }

        public override System.String ToString() {
          System.Text.StringBuilder b = new System.Text.StringBuilder();

          b.Append("NestedType");
          b.Append("(");
          b.Append(")");

          return b.ToString();
        }
      }

      [Newtonsoft.Json.JsonConverter(typeof(JsonSubTypes.JsonSubtypes), "type")]
      [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(NestedInterface.NestedFoo), "NestedFoo")]
      public abstract class NestedInterface {
        [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)]
        private System.String TypeField {
          get;
        }

        public NestedInterface(
          System.String TypeField
        ) {
          this.TypeField = TypeField;
        }

        [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
        public class NestedFoo : NestedInterface {
          [Newtonsoft.Json.JsonConstructor]
          public NestedFoo(
            [Newtonsoft.Json.JsonProperty("type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField
          ) : base(TypeField) {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(System.Object other) {
            NestedFoo o = other as NestedFoo;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override System.String ToString() {
            System.Text.StringBuilder b = new System.Text.StringBuilder();

            b.Append("NestedFoo");
            b.Append("(");
            b.Append(")");

            return b.ToString();
          }

          [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
          public class Nested {
            [Newtonsoft.Json.JsonConstructor]
            public Nested() {
            }

            public override Int32 GetHashCode() {
              Int32 result = 1;
              return result;
            }

            public override Boolean Equals(System.Object other) {
              Nested o = other as Nested;

              if (o == null) {
                return false;
              }

              return true;
            }

            public override System.String ToString() {
              System.Text.StringBuilder b = new System.Text.StringBuilder();

              b.Append("Nested");
              b.Append("(");
              b.Append(")");

              return b.ToString();
            }
          }
        }
      }

      [Newtonsoft.Json.JsonConverter(typeof(Newtonsoft.Json.Converters.StringEnumConverter))]
      public enum NestedEnum {
        [System.Runtime.Serialization.EnumMember(Value = "Foo")]
        FOO
      }

      [Newtonsoft.Json.JsonConverter(typeof(NestedTuple.Json_Net_Converter))]
      public class NestedTuple {
        public NestedTuple() {
        }

        public override Int32 GetHashCode() {
          Int32 result = 1;
          return result;
        }

        public override Boolean Equals(System.Object other) {
          NestedTuple o = other as NestedTuple;

          if (o == null) {
            return false;
          }

          return true;
        }

        public override System.String ToString() {
          System.Text.StringBuilder b = new System.Text.StringBuilder();

          b.Append("NestedTuple");
          b.Append("(");
          b.Append(")");

          return b.ToString();
        }

        public class Json_Net_Converter : Newtonsoft.Json.JsonConverter {
          public override Boolean CanConvert(System.Type objectType) {
            return objectType == typeof(NestedTuple);
          }

          public override void WriteJson(Newtonsoft.Json.JsonWriter writer, System.Object obj, Newtonsoft.Json.JsonSerializer serializer) {
            NestedTuple o = (NestedTuple)obj;
            Newtonsoft.Json.Linq.JArray array = new Newtonsoft.Json.Linq.JArray();
            array.WriteTo(writer);
          }

          public override System.Object ReadJson(Newtonsoft.Json.JsonReader reader, System.Type objectType, System.Object existingValue, Newtonsoft.Json.JsonSerializer serializer) {
            Newtonsoft.Json.Linq.JArray array = Newtonsoft.Json.Linq.JArray.Load(reader);
            System.Collections.Generic.IEnumerator<Newtonsoft.Json.Linq.JToken> enumerator = array.GetEnumerator();
            return new NestedTuple();
          }
        }

        [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
        public class Nested {
          [Newtonsoft.Json.JsonConstructor]
          public Nested() {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(System.Object other) {
            Nested o = other as Nested;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override System.String ToString() {
            System.Text.StringBuilder b = new System.Text.StringBuilder();

            b.Append("Nested");
            b.Append("(");
            b.Append(")");

            return b.ToString();
          }
        }
      }

      public abstract class NestedService {
        [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
        public class Nested {
          [Newtonsoft.Json.JsonConstructor]
          public Nested() {
          }

          public override Int32 GetHashCode() {
            Int32 result = 1;
            return result;
          }

          public override Boolean Equals(System.Object other) {
            Nested o = other as Nested;

            if (o == null) {
              return false;
            }

            return true;
          }

          public override System.String ToString() {
            System.Text.StringBuilder b = new System.Text.StringBuilder();

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
