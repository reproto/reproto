package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("tagged")
  private final Optional<Tagged> tagged;
  @JsonProperty("required_fields")
  private final Optional<RequiredFields> requiredFields;

  @JsonCreator
  public Entry(
    @JsonProperty("tagged") final Optional<Tagged> tagged,
    @JsonProperty("required_fields") final Optional<RequiredFields> requiredFields
  ) {
    Objects.requireNonNull(tagged, "tagged");
    this.tagged = tagged;
    Objects.requireNonNull(requiredFields, "required_fields");
    this.requiredFields = requiredFields;
  }

  @JsonProperty("tagged")
  public Optional<Tagged> getTagged() {
    return this.tagged;
  }

  @JsonProperty("required_fields")
  public Optional<RequiredFields> getRequiredFields() {
    return this.requiredFields;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.tagged.hashCode();
    result = result * 31 + this.requiredFields.hashCode();
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

    if (!this.requiredFields.equals(o.requiredFields)) {
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
    b.append("required_fields=");
    b.append(this.requiredFields.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Tagged> tagged = Optional.empty();
    private Optional<RequiredFields> requiredFields = Optional.empty();

    public Builder tagged(final Tagged tagged) {
      this.tagged = Optional.of(tagged);
      return this;
    }

    public Builder requiredFields(final RequiredFields requiredFields) {
      this.requiredFields = Optional.of(requiredFields);
      return this;
    }

    public Entry build() {
      final Optional<Tagged> tagged = this.tagged;
      final Optional<RequiredFields> requiredFields = this.requiredFields;

      return new Entry(tagged, requiredFields);
    }
  }
}
