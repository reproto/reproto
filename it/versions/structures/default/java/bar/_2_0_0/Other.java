package bar._2_0_0;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Other {
  @JsonProperty("name2")
  private final String name2;

  @JsonCreator
  public Other(
    @JsonProperty("name2") final String name2
  ) {
    Objects.requireNonNull(name2, "name2");
    this.name2 = name2;
  }

  @JsonProperty("name2")
  public String getName2() {
    return this.name2;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.name2.hashCode();
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

    if (!this.name2.equals(o.name2)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Other");
    b.append("(");
    b.append("name2=");
    b.append(this.name2.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> name2 = Optional.empty();

    public Builder name2(final String name2) {
      this.name2 = Optional.of(name2);
      return this;
    }

    public Other build() {
      final String name2 = this.name2.orElseThrow(() -> new RuntimeException("name2: is required"));

      return new Other(name2);
    }
  }
}
