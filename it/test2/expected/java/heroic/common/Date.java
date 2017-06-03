package heroic.common;

import java.util.Optional;

public class Date {
  private final long field;

  public Date(final long field) {
    this.field = field;
  }

  public long getField() {
    return this.field;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + Long.hashCode(this.field);
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Date)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Date o = (Date) other;

    if (this.field != o.field) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Date");
    b.append("(");
    b.append("field=");
    b.append(Long.toString(this.field));
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Long> field = Optional.empty();

    public Builder field(final long field) {
      this.field = Optional.of(field);
      return this;
    }

    public Date build() {
      final long field = this.field.orElseThrow(() -> new RuntimeException("field: is required"));

      return new Date(field);
    }
  }
}
