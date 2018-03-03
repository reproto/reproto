using JsonSubTypes;
using Newtonsoft.Json;
using System;
using System.Text;

namespace Test {
  [JsonConverter(typeof(JsonSubtypes), "@type")]
  [JsonSubtypes.KnownSubType(typeof(Entry.A), "foo")]
  [JsonSubtypes.KnownSubType(typeof(Entry.B), "b")]
  [JsonSubtypes.KnownSubType(typeof(Entry.Bar), "Bar")]
  [JsonSubtypes.KnownSubType(typeof(Entry.Baz), "Baz")]
  public abstract class Entry {
    [JsonProperty("@type", Required = Required.DisallowNull)]
    private String TypeField {
      get;
    }

    public Entry(
      String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class A : Entry {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }

      [JsonConstructor]
      public A(
        [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        A o = other as A;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("A");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class B : Entry {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }

      [JsonConstructor]
      public B(
        [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        B o = other as B;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("B");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Bar : Entry {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }

      [JsonConstructor]
      public Bar(
        [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        Bar o = other as Bar;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("Bar");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [JsonObject(ItemNullValueHandling = NullValueHandling.Ignore)]
    public class Baz : Entry {
      [JsonProperty("shared", Required = Required.DisallowNull)]
      public String shared {
        get;
      }

      [JsonConstructor]
      public Baz(
        [JsonProperty("@type", Required = Required.DisallowNull)] String TypeField,
        [JsonProperty("shared", Required = Required.DisallowNull)] String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(Object other) {
        Baz o = other as Baz;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override String ToString() {
        StringBuilder b = new StringBuilder();

        b.Append("Baz");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }
  }
}
