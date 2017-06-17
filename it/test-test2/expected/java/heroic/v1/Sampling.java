package heroic.v1;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Sampling {
  private final Optional<TimeUnit> unit;
  private final int size;
  private final Optional<Integer> extent;

  @JsonCreator
  public Sampling(
    @JsonProperty("unit") final Optional<TimeUnit> unit, 
    @JsonProperty("size") final int size, 
    @JsonProperty("extent") final Optional<Integer> extent
  ) {
    Objects.requireNonNull(unit, "unit");
    this.unit = unit;
    this.size = size;
    Objects.requireNonNull(extent, "extent");
    this.extent = extent;
  }

  public Optional<TimeUnit> getUnit() {
    return this.unit;
  }

  public int getSize() {
    return this.size;
  }

  public Optional<Integer> getExtent() {
    return this.extent;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.unit.hashCode();
    result = result * 31 + this.size;
    result = result * 31 + this.extent.hashCode();
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Sampling)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Sampling o = (Sampling) other;

    if (!this.unit.equals(o.unit)) {
      return false;
    }

    if (this.size != o.size) {
      return false;
    }

    if (!this.extent.equals(o.extent)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Sampling");
    b.append("(");
    b.append("unit=");
    b.append(this.unit.toString());
    b.append(", ");
    b.append("size=");
    b.append(Integer.toString(this.size));
    b.append(", ");
    b.append("extent=");
    b.append(this.extent.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<TimeUnit> unit = Optional.empty();
    private Optional<Integer> size = Optional.empty();
    private Optional<Integer> extent = Optional.empty();

    public Builder unit(final TimeUnit unit) {
      this.unit = Optional.of(unit);
      return this;
    }

    public Builder size(final int size) {
      this.size = Optional.of(size);
      return this;
    }

    public Builder extent(final int extent) {
      this.extent = Optional.of(extent);
      return this;
    }

    public Sampling build() {
      final Optional<TimeUnit> unit = this.unit;
      final int size = this.size.orElseThrow(() -> new RuntimeException("size: is required"));
      final Optional<Integer> extent = this.extent;

      return new Sampling(unit, size, extent);
    }
  }
}
