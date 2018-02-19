package foo._4_0_0;

import bar._1_0_0.Other;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Thing {
  @JsonProperty("name")
  private final Optional<String> name;
  @JsonProperty("other")
  private final Optional<Other> other;
  @JsonProperty("other2")
  private final Optional<bar._2_0_0.Other> other2;

  @JsonCreator
  public Thing(
    @JsonProperty("name") final Optional<String> name,
    @JsonProperty("other") final Optional<Other> other,
    @JsonProperty("other2") final Optional<bar._2_0_0.Other> other2
  ) {
    Objects.requireNonNull(name, "name");
    this.name = name;
    Objects.requireNonNull(other, "other");
    this.other = other;
    Objects.requireNonNull(other2, "other2");
    this.other2 = other2;
  }

  public Optional<String> getName() {
    return this.name;
  }

  public Optional<Other> getOther() {
    return this.other;
  }

  public Optional<bar._2_0_0.Other> getOther2() {
    return this.other2;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.name.hashCode();
    result = result * 31 + this.other.hashCode();
    result = result * 31 + this.other2.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Thing)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Thing o = (Thing) other;

    if (!this.name.equals(o.name)) {
      return false;
    }

    if (!this.other.equals(o.other)) {
      return false;
    }

    if (!this.other2.equals(o.other2)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Thing");
    b.append("(");
    b.append("name=");
    b.append(this.name.toString());
    b.append(", ");
    b.append("other=");
    b.append(this.other.toString());
    b.append(", ");
    b.append("other2=");
    b.append(this.other2.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> name = Optional.empty();
    private Optional<Other> other = Optional.empty();
    private Optional<bar._2_0_0.Other> other2 = Optional.empty();

    public Builder name(final String name) {
      this.name = Optional.of(name);
      return this;
    }

    public Builder other(final Other other) {
      this.other = Optional.of(other);
      return this;
    }

    public Builder other2(final bar._2_0_0.Other other2) {
      this.other2 = Optional.of(other2);
      return this;
    }

    public Thing build() {
      final Optional<String> name = this.name;
      final Optional<Other> other = this.other;
      final Optional<bar._2_0_0.Other> other2 = this.other2;

      return new Thing(name, other, other2);
    }
  }
}
