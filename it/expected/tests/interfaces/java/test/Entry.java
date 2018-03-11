package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import java.util.Objects;
import java.util.Optional;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="@type")
@JsonSubTypes({
  @JsonSubTypes.Type(name="foo", value=Entry.A.class),
  @JsonSubTypes.Type(name="b", value=Entry.B.class),
  @JsonSubTypes.Type(name="Bar", value=Entry.Bar.class),
  @JsonSubTypes.Type(name="Baz", value=Entry.Baz.class)
})
public interface Entry {
  String getShared();

  public static class A implements Entry {
    @JsonProperty("shared")
    private final String shared;

    @JsonCreator
    public A(
      @JsonProperty("shared") final String shared
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof A)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final A o = (A) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("A");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public A build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));

        return new A(shared);
      }
    }
  }

  public static class B implements Entry {
    @JsonProperty("shared")
    private final String shared;

    @JsonCreator
    public B(
      @JsonProperty("shared") final String shared
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof B)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final B o = (B) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("B");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public B build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));

        return new B(shared);
      }
    }
  }

  public static class Bar implements Entry {
    @JsonProperty("shared")
    private final String shared;

    @JsonCreator
    public Bar(
      @JsonProperty("shared") final String shared
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Bar)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Bar o = (Bar) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Bar");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Bar build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));

        return new Bar(shared);
      }
    }
  }

  public static class Baz implements Entry {
    @JsonProperty("shared")
    private final String shared;

    @JsonCreator
    public Baz(
      @JsonProperty("shared") final String shared
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
    }

    @Override
    @JsonProperty("shared")
    public String getShared() {
      return this.shared;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Baz)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Baz o = (Baz) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Baz");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Baz build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));

        return new Baz(shared);
      }
    }
  }
}
