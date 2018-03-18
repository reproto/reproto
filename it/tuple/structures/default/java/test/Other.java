package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Other {
  @JsonProperty("a")
  private final String a;

  @JsonCreator
  public Other(
    @JsonProperty("a") final String a
  ) {
    Objects.requireNonNull(a, "a");
    this.a = a;
  }

  @JsonProperty("a")
  public String getA() {
    return this.a;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.a.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Other)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Other o = (Other) other;

    if (!this.a.equals(o.a)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Other");
    b.append("(");
    b.append("a=");
    b.append(this.a.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> a = Optional.empty();

    public Builder a(final String a) {
      this.a = Optional.of(a);
      return this;
    }

    public Other build() {
      final String a = this.a.orElseThrow(() -> new RuntimeException("a: is required"));

      return new Other(a);
    }
  }
}
