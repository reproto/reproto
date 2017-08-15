package test;

import java.util.Objects;
import java.util.Optional;

public class Bar {
  private final Bar.Inner field;

  public Bar(
    final Bar.Inner field
  ) {
    Objects.requireNonNull(field, "field");
    this.field = field;
  }

  public Bar.Inner getField() {
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

    if (!(other instanceof Bar)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Bar o = (Bar) other;

    if (!this.field.equals(o.field)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Bar");
    b.append("(");
    b.append("field=");
    b.append(this.field.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Bar.Inner> field = Optional.empty();

    public Builder field(final Bar.Inner field) {
      this.field = Optional.of(field);
      return this;
    }

    public Bar build() {
      final Bar.Inner field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

      return new Bar(field);
    }
  }

  public static class Inner {
    private final String field;

    public Inner(
      final String field
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

      if (!(other instanceof Inner)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Inner o = (Inner) other;

      if (!this.field.equals(o.field)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Inner");
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

      public Inner build() {
        final String field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

        return new Inner(field);
      }
    }
  }
}
