package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final Optional<Foo> foo;

  @JsonCreator
  public Entry(
    @JsonProperty("foo") final Optional<Foo> foo
  ) {
    Objects.requireNonNull(foo, "foo");
    this.foo = foo;
  }

  public Optional<Foo> getFoo() {
    return this.foo;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.foo.hashCode();
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

    if (!this.foo.equals(o.foo)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("foo=");
    b.append(this.foo.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Foo> foo = Optional.empty();

    public Builder foo(final Foo foo) {
      this.foo = Optional.of(foo);
      return this;
    }

    public Entry build() {
      final Optional<Foo> foo = this.foo;

      return new Entry(foo);
    }
  }
}
