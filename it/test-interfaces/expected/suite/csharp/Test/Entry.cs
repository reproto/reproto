using System;

namespace Test {
  [Newtonsoft.Json.JsonConverter(typeof(JsonSubTypes.JsonSubtypes), "@type")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(Entry.A), "foo")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(Entry.B), "b")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(Entry.Bar), "Bar")]
  [JsonSubTypes.JsonSubtypes.KnownSubType(typeof(Entry.Baz), "Baz")]
  public abstract class Entry {
    [Newtonsoft.Json.JsonProperty("@type", Required = Newtonsoft.Json.Required.DisallowNull)]
    private System.String TypeField {
      get;
    }

    public Entry(
      System.String TypeField
    ) {
      this.TypeField = TypeField;
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class A : Entry {
      [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String shared {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public A(
        [Newtonsoft.Json.JsonProperty("@type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField,
        [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)] System.String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(System.Object other) {
        A o = other as A;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

        b.Append("A");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class B : Entry {
      [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String shared {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public B(
        [Newtonsoft.Json.JsonProperty("@type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField,
        [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)] System.String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(System.Object other) {
        B o = other as B;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

        b.Append("B");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class Bar : Entry {
      [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String shared {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public Bar(
        [Newtonsoft.Json.JsonProperty("@type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField,
        [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)] System.String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(System.Object other) {
        Bar o = other as Bar;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

        b.Append("Bar");
        b.Append("(");
        b.Append("shared=");
        b.Append(this.shared);
        b.Append(")");

        return b.ToString();
      }
    }

    [Newtonsoft.Json.JsonObject(ItemNullValueHandling = Newtonsoft.Json.NullValueHandling.Ignore)]
    public class Baz : Entry {
      [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)]
      public System.String shared {
        get;
      }

      [Newtonsoft.Json.JsonConstructor]
      public Baz(
        [Newtonsoft.Json.JsonProperty("@type", Required = Newtonsoft.Json.Required.DisallowNull)] System.String TypeField,
        [Newtonsoft.Json.JsonProperty("shared", Required = Newtonsoft.Json.Required.DisallowNull)] System.String shared
      ) : base(TypeField) {
        this.shared = shared;
      }

      public override Int32 GetHashCode() {
        Int32 result = 1;
        result = result * 31 + this.shared.GetHashCode();
        return result;
      }

      public override Boolean Equals(System.Object other) {
        Baz o = other as Baz;

        if (o == null) {
          return false;
        }

        if (!this.shared.Equals(o.shared)) {
          return false;
        }

        return true;
      }

      public override System.String ToString() {
        System.Text.StringBuilder b = new System.Text.StringBuilder();

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
