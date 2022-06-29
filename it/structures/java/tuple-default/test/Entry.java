package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("tuple1")
    final Optional<Tuple1> tuple1;
    @JsonProperty("tuple2")
    final Optional<Tuple2> tuple2;

    @JsonCreator
    public Entry(
        @JsonProperty("tuple1") Optional<Tuple1> tuple1,
        @JsonProperty("tuple2") Optional<Tuple2> tuple2
    ) {
        this.tuple1 = tuple1;
        this.tuple2 = tuple2;
    }

    @JsonProperty("tuple1")
    public Optional<Tuple1> getTuple1() {
        return this.tuple1;
    }

    @JsonProperty("tuple2")
    public Optional<Tuple2> getTuple2() {
        return this.tuple2;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("tuple1=");
        b.append(this.tuple1.toString());
        b.append(", ");
        b.append("tuple2=");
        b.append(this.tuple2.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.tuple1.hashCode();
        result = result * 31 + this.tuple2.hashCode();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof Entry)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Entry o_ = (Entry)other_;

        if (!this.tuple1.equals(o_.tuple1)) {
            return false;
        }

        if (!this.tuple2.equals(o_.tuple2)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<Tuple1> tuple1;
        private Optional<Tuple2> tuple2;

        private Builder() {
            this.tuple1 = Optional.empty();
            this.tuple2 = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.tuple1,
                this.tuple2
            );
        }

        public Builder tuple1(final Tuple1 tuple1) {
            this.tuple1 = Optional.of(tuple1);
            return this;
        }

        public Builder tuple2(final Tuple2 tuple2) {
            this.tuple2 = Optional.of(tuple2);
            return this;
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }
}
