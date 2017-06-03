package heroic.v1;

import java.util.List;
import java.util.Objects;
import java.util.Optional;

public interface Aggregation {
  public static class Average implements Aggregation {
    private final Optional<Sampling> sampling;
    private final Optional<Duration> size;
    private final Optional<Duration> extent;

    public Average(final Optional<Sampling> sampling, final Optional<Duration> size, final Optional<Duration> extent) {
      Objects.requireNonNull(sampling, "sampling");
      this.sampling = sampling;
      Objects.requireNonNull(size, "size");
      this.size = size;
      Objects.requireNonNull(extent, "extent");
      this.extent = extent;
    }

    public Optional<Sampling> getSampling() {
      return this.sampling;
    }

    public Optional<Duration> getSize() {
      return this.size;
    }

    public Optional<Duration> getExtent() {
      return this.extent;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.sampling.hashCode();
      result = result * 31 + this.size.hashCode();
      result = result * 31 + this.extent.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Aggregation.Average)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Aggregation.Average o = (Aggregation.Average) other;

      if (!this.sampling.equals(o.sampling)) {
        return false;
      }

      if (!this.size.equals(o.size)) {
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

      b.append("Aggregation.Average");
      b.append("(");
      b.append("sampling=");
      b.append(this.sampling.toString());
      b.append(", ");
      b.append("size=");
      b.append(this.size.toString());
      b.append(", ");
      b.append("extent=");
      b.append(this.extent.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<Sampling> sampling = Optional.empty();
      private Optional<Duration> size = Optional.empty();
      private Optional<Duration> extent = Optional.empty();

      public Builder sampling(final Sampling sampling) {
        this.sampling = Optional.of(sampling);
        return this;
      }

      public Builder size(final Duration size) {
        this.size = Optional.of(size);
        return this;
      }

      public Builder extent(final Duration extent) {
        this.extent = Optional.of(extent);
        return this;
      }

      public Aggregation.Average build() {
        final Optional<Sampling> sampling = this.sampling;
        final Optional<Duration> size = this.size;
        final Optional<Duration> extent = this.extent;

        return new Aggregation.Average(sampling, size, extent);
      }
    }
  }

  public static class Chain implements Aggregation {
    private final List<Aggregation> chain;

    public Chain(final List<Aggregation> chain) {
      Objects.requireNonNull(chain, "chain");
      this.chain = chain;
    }

    public List<Aggregation> getChain() {
      return this.chain;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.chain.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Aggregation.Chain)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Aggregation.Chain o = (Aggregation.Chain) other;

      if (!this.chain.equals(o.chain)) {
        return false;
      }

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("Aggregation.Chain");
      b.append("(");
      b.append("chain=");
      b.append(this.chain.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<List<Aggregation>> chain = Optional.empty();

      public Builder chain(final List<Aggregation> chain) {
        this.chain = Optional.of(chain);
        return this;
      }

      public Aggregation.Chain build() {
        final List<Aggregation> chain = this.chain.orElseThrow(() -> new RuntimeException("chain: is required"));

        return new Aggregation.Chain(chain);
      }
    }
  }

  public static class Sum implements Aggregation {
    private final Optional<Sampling> sampling;
    private final Optional<Duration> size;
    private final Optional<Duration> extent;

    public Sum(final Optional<Sampling> sampling, final Optional<Duration> size, final Optional<Duration> extent) {
      Objects.requireNonNull(sampling, "sampling");
      this.sampling = sampling;
      Objects.requireNonNull(size, "size");
      this.size = size;
      Objects.requireNonNull(extent, "extent");
      this.extent = extent;
    }

    public Optional<Sampling> getSampling() {
      return this.sampling;
    }

    public Optional<Duration> getSize() {
      return this.size;
    }

    public Optional<Duration> getExtent() {
      return this.extent;
    }

    @Override
    public int hashCode() {
      int result = 1;
      result = result * 31 + this.sampling.hashCode();
      result = result * 31 + this.size.hashCode();
      result = result * 31 + this.extent.hashCode();
      return result;
    }

    @Override
    public boolean equals(final Object other) {
      if (other == null) {
        return false;
      }

      if (!(other instanceof Aggregation.Sum)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final Aggregation.Sum o = (Aggregation.Sum) other;

      if (!this.sampling.equals(o.sampling)) {
        return false;
      }

      if (!this.size.equals(o.size)) {
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

      b.append("Aggregation.Sum");
      b.append("(");
      b.append("sampling=");
      b.append(this.sampling.toString());
      b.append(", ");
      b.append("size=");
      b.append(this.size.toString());
      b.append(", ");
      b.append("extent=");
      b.append(this.extent.toString());
      b.append(")");

      return b.toString();
    }

    public static class Builder {
      private Optional<Sampling> sampling = Optional.empty();
      private Optional<Duration> size = Optional.empty();
      private Optional<Duration> extent = Optional.empty();

      public Builder sampling(final Sampling sampling) {
        this.sampling = Optional.of(sampling);
        return this;
      }

      public Builder size(final Duration size) {
        this.size = Optional.of(size);
        return this;
      }

      public Builder extent(final Duration extent) {
        this.extent = Optional.of(extent);
        return this;
      }

      public Aggregation.Sum build() {
        final Optional<Sampling> sampling = this.sampling;
        final Optional<Duration> size = this.size;
        final Optional<Duration> extent = this.extent;

        return new Aggregation.Sum(sampling, size, extent);
      }
    }
  }
}
