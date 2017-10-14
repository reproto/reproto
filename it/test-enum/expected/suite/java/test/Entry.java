package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("explicit")
  private final Optional<EnumExplicit> explicit;
  @JsonProperty("implicit")
  private final Optional<EnumImplicit> implicit;

  @JsonCreator
  public Entry(
    @JsonProperty("explicit") final Optional<EnumExplicit> explicit,
    @JsonProperty("implicit") final Optional<EnumImplicit> implicit
  ) {
    Objects.requireNonNull(explicit, "explicit");
    this.explicit = explicit;
    Objects.requireNonNull(implicit, "implicit");
    this.implicit = implicit;
  }

  public Optional<EnumExplicit> getExplicit() {
    return this.explicit;
  }

  public Optional<EnumImplicit> getImplicit() {
    return this.implicit;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.explicit.hashCode();
    result = result * 31 + this.implicit.hashCode();
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

    if (!this.explicit.equals(o.explicit)) {
      return false;
    }

    if (!this.implicit.equals(o.implicit)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("explicit=");
    b.append(this.explicit.toString());
    b.append(", ");
    b.append("implicit=");
    b.append(this.implicit.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<EnumExplicit> explicit = Optional.empty();
    private Optional<EnumImplicit> implicit = Optional.empty();

    public Builder explicit(final EnumExplicit explicit) {
      this.explicit = Optional.of(explicit);
      return this;
    }

    public Builder implicit(final EnumImplicit implicit) {
      this.implicit = Optional.of(implicit);
      return this;
    }

    public Entry build() {
      final Optional<EnumExplicit> explicit = this.explicit;
      final Optional<EnumImplicit> implicit = this.implicit;

      return new Entry(explicit, implicit);
    }
  }
}
