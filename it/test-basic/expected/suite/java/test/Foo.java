package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Foo {
  /**
   * <pre>
   * The field.
   * </pre>
   */
  @JsonProperty("field")
  private final String field;

  @JsonCreator
  public Foo(
    @JsonProperty("field") final String field
  ) {
    Objects.requireNonNull(field, "field");
    this.field = field;
  }

  /**
   * <pre>
   * The field.
   * </pre>
   */
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

    if (!(other instanceof Foo)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Foo o = (Foo) other;

    if (!this.field.equals(o.field)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Foo");
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

    public Foo build() {
      final String field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

      return new Foo(field);
    }
  }
}
