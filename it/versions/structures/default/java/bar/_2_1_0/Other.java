package bar._2_1_0;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Other {
  @JsonProperty("name21")
  private final String name21;

  @JsonCreator
  public Other(
    @JsonProperty("name21") final String name21
  ) {
    Objects.requireNonNull(name21, "name21");
    this.name21 = name21;
  }

  @JsonProperty("name21")
  public String getName21() {
    return this.name21;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.name21.hashCode();
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

    if (!this.name21.equals(o.name21)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Other");
    b.append("(");
    b.append("name21=");
    b.append(this.name21.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> name21 = Optional.empty();

    public Builder name21(final String name21) {
      this.name21 = Optional.of(name21);
      return this;
    }

    public Other build() {
      final String name21 = this.name21.orElseThrow(() -> new RuntimeException("name21: is required"));

      return new Other(name21);
    }
  }
}
