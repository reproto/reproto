package bar._1_0_0;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Other {
  private final String name;

  @JsonCreator
  public Other(
    @JsonProperty("name") final String name
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

    if (!(other instanceof Other)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Other o = (Other) other;

    if (!this.name.equals(o.name)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Other");
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

    public Other build() {
      final String name = this.name.orElseThrow(() -> new RuntimeException("name: is required"));

      return new Other(name);
    }
  }
}
