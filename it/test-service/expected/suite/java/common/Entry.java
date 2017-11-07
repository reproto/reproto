package common;

import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final String name;

  public Entry(
    final String name
  ) {
    Objects.requireNonNull(name, "name");
    this.name = name;
  }

  public String getName() {
    return this.name;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.name.hashCode();
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

    if (!this.name.equals(o.name)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("name=");
    b.append(this.name.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> name = Optional.empty();

    public Builder name(final String name) {
      this.name = Optional.of(name);
      return this;
    }

    public Entry build() {
      final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));

      return new Entry(name);
    }
  }
}
