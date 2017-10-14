package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import java.util.Objects;
import java.util.Optional;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({
  @JsonSubTypes.Type(name="bar", value=Entry.Bar.class),
  @JsonSubTypes.Type(name="foo", value=Entry.Foo.class)
})
public interface Entry {
  public String getShared();

  public static class Bar implements Entry {
    @JsonProperty("shared")
    private final String shared;
    @JsonProperty("bar")
    private final String bar;

    @JsonCreator
    public Bar(
      @JsonProperty("shared") final String shared,
      @JsonProperty("bar") final String bar
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
      Objects.requireNonNull(bar, "bar");
      this.bar = bar;
    }

    @Override
    public String getShared() {
      return this.shared;
    }

    public String getBar() {
      return this.bar;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      result = result * 31 + this.bar.hashCode();
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

      if (!this.bar.equals(o.bar)) {
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
      b.append(", ");
      b.append("bar=");
      b.append(this.bar.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();
      private Optional<String> bar = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Builder bar(final String bar) {
        this.bar = Optional.of(bar);
        return this;
      }

      public Bar build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));
        final String bar = this.bar.orElseThrow(() -> new RuntimeException("bar: is required"));

        return new Bar(shared, bar);
      }
    }
  }

  public static class Foo implements Entry {
    @JsonProperty("shared")
    private final String shared;
    @JsonProperty("foo")
    private final String foo;

    @JsonCreator
    public Foo(
      @JsonProperty("shared") final String shared,
      @JsonProperty("foo") final String foo
    ) {
      Objects.requireNonNull(shared, "shared");
      this.shared = shared;
      Objects.requireNonNull(foo, "foo");
      this.foo = foo;
    }

    @Override
    public String getShared() {
      return this.shared;
    }

    public String getFoo() {
      return this.foo;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.shared.hashCode();
      result = result * 31 + this.foo.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Foo)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Foo o = (Foo) other;

      if (!this.shared.equals(o.shared)) {
        return false;
      }

      if (!this.foo.equals(o.foo)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Foo");
      b.append("(");
      b.append("shared=");
      b.append(this.shared.toString());
      b.append(", ");
      b.append("foo=");
      b.append(this.foo.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> shared = Optional.empty();
      private Optional<String> foo = Optional.empty();

      public Builder shared(final String shared) {
        this.shared = Optional.of(shared);
        return this;
      }

      public Builder foo(final String foo) {
        this.foo = Optional.of(foo);
        return this;
      }

      public Foo build() {
        final String shared = this.shared.orElseThrow(() -> new RuntimeException("shared: is required"));
        final String foo = this.foo.orElseThrow(() -> new RuntimeException("foo: is required"));

        return new Foo(shared, foo);
      }
    }
  }
}
