package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import foo._4_0_0.Thing;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("thing")
  private final Optional<Thing> thing;

  @JsonCreator
  public Entry(
    @JsonProperty("thing") final Optional<Thing> thing
  ) {
    Objects.requireNonNull(thing, "thing");
    this.thing = thing;
  }

  public Optional<Thing> getThing() {
    return this.thing;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.thing.hashCode();
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

    if (!this.thing.equals(o.thing)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("thing=");
    b.append(this.thing.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Thing> thing = Optional.empty();

    public Builder thing(final Thing thing) {
      this.thing = Optional.of(thing);
      return this;
    }

    public Entry build() {
      final Optional<Thing> thing = this.thing;

      return new Entry(thing);
    }
  }
}
