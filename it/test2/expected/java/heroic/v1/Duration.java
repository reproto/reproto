package heroic.v1;

import java.util.Optional;

public interface Duration {
  public static class Absolute implements Duration {
    private final long start;
    private final long end;

    public Absolute(final long start, final long end) {
      this.start = start;
      this.end = end;
    }

    public long getStart() {
      return this.start;
    }

    public long getEnd() {
      return this.end;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + Long.hashCode(this.start);
      result = result * 31 + Long.hashCode(this.end);
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Duration.Absolute)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Duration.Absolute o = (Duration.Absolute) other;

      if (this.start != o.start) {
        return false;
      }

      if (this.end != o.end) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Duration.Absolute");
      b.append("(");
      b.append("start=");
      b.append(Long.toString(this.start));
      b.append(", ");
      b.append("end=");
      b.append(Long.toString(this.end));
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<Long> start = Optional.empty();
      private Optional<Long> end = Optional.empty();

      public Builder start(final long start) {
        this.start = Optional.of(start);
        return this;
      }

      public Builder end(final long end) {
        this.end = Optional.of(end);
        return this;
      }

      public Duration.Absolute build() {
        final long start = this.start.orElseThrow(() -> new RuntimeException("start: is required"));
        final long end = this.end.orElseThrow(() -> new RuntimeException("end: is required"));

        return new Duration.Absolute(start, end);
      }
    }
  }
}
