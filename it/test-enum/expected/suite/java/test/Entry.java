package test;

import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final EnumExplicit explicit;
  private final EnumImplicit implicit;

  public Entry(
    final EnumExplicit explicit, final EnumImplicit implicit
  ) {
    Objects.requireNonNull(explicit, "explicit");
    this.explicit = explicit;
    Objects.requireNonNull(implicit, "implicit");
    this.implicit = implicit;
  }

  public EnumExplicit getExplicit() {
    return this.explicit;
  }

  public EnumImplicit getImplicit() {
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
      final EnumExplicit explicit = this.explicit.orElseThrow(() -> new RuntimeException("explicit: is required"));
      final EnumImplicit implicit = this.implicit.orElseThrow(() -> new RuntimeException("implicit: is required"));

      return new Entry(explicit, implicit);
    }
  }
}
