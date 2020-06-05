package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("tagged")
  private final Optional<Tagged> tagged;
  @JsonProperty("untagged")
  private final Optional<Untagged> untagged;

  @JsonCreator
  public Entry(
    @JsonProperty("tagged") final Optional<Tagged> tagged,
    @JsonProperty("untagged") final Optional<Untagged> untagged
  ) {
    Objects.requireNonNull(tagged, "tagged");
    this.tagged = tagged;
    Objects.requireNonNull(untagged, "untagged");
    this.untagged = untagged;
  }

  @JsonProperty("tagged")
  public Optional<Tagged> getTagged() {
    return this.tagged;
  }

  @JsonProperty("untagged")
  public Optional<Untagged> getUntagged() {
    return this.untagged;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.tagged.hashCode();
    result = result * 31 + this.untagged.hashCode();
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

    if (!this.tagged.equals(o.tagged)) {
      return false;
    }

    if (!this.untagged.equals(o.untagged)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("tagged=");
    b.append(this.tagged.toString());
    b.append(", ");
    b.append("untagged=");
    b.append(this.untagged.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Tagged> tagged = Optional.empty();
    private Optional<Untagged> untagged = Optional.empty();

    public Builder tagged(final Tagged tagged) {
      this.tagged = Optional.of(tagged);
      return this;
    }

    public Builder untagged(final Untagged untagged) {
      this.untagged = Optional.of(untagged);
      return this;
    }

    public Entry build() {
      final Optional<Tagged> tagged = this.tagged;
      final Optional<Untagged> untagged = this.untagged;

      return new Entry(tagged, untagged);
    }
  }
}
