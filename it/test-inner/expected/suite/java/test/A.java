package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class A {
  @JsonProperty("b")
  private final A.B b;

  @JsonCreator
  public A(
    @JsonProperty("b") final A.B b
  ) {
    Objects.requireNonNull(b, "b");
    this.b = b;
  }

  public A.B getB() {
    return this.b;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.b.hashCode();
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

    if (!this.b.equals(o.b)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("A");
    b.append("(");
    b.append("b=");
    b.append(this.b.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<A.B> b = Optional.empty();

    public Builder b(final A.B b) {
      this.b = Optional.of(b);
      return this;
    }

    public A build() {
      final A.B b = this.b.orElseThrow(() -> new RuntimeException("b: is required"));

      return new A(b);
    }
  }

  public static class B {
    @JsonProperty("field")
    private final String field;

    @JsonCreator
    public B(
      @JsonProperty("field") final String field
    ) {
      Objects.requireNonNull(field, "field");
      this.field = field;
    }

    public String getField() {
      return this.field;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.field.hashCode();
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

      if (!this.field.equals(o.field)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("B");
      b.append("(");
      b.append("field=");
      b.append(this.field.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<String> field = Optional.empty();

      public Builder field(final String field) {
        this.field = Optional.of(field);
        return this;
      }

      public B build() {
        final String field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

        return new B(field);
      }
    }
  }
}
