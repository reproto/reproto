package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("a")
  private final Optional<A> a;
  @JsonProperty("b")
  private final Optional<A.B> b;

  @JsonCreator
  public Entry(
    @JsonProperty("a") final Optional<A> a,
    @JsonProperty("b") final Optional<A.B> b
  ) {
    Objects.requireNonNull(a, "a");
    this.a = a;
    Objects.requireNonNull(b, "b");
    this.b = b;
  }

  @JsonProperty("a")
  public Optional<A> getA() {
    return this.a;
  }

  @JsonProperty("b")
  public Optional<A.B> getB() {
    return this.b;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.a.hashCode();
    result = result * 31 + this.b.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Entry)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Entry o = (Entry) other;

    if (!this.a.equals(o.a)) {
      return false;
    }

    if (!this.b.equals(o.b)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("a=");
    b.append(this.a.toString());
    b.append(", ");
    b.append("b=");
    b.append(this.b.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<A> a = Optional.empty();
    private Optional<A.B> b = Optional.empty();

    public Builder a(final A a) {
      this.a = Optional.of(a);
      return this;
    }

    public Builder b(final A.B b) {
      this.b = Optional.of(b);
      return this;
    }

    public Entry build() {
      final Optional<A> a = this.a;
      final Optional<A.B> b = this.b;

      return new Entry(a, b);
    }
  }
}
