package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("a")
    final Optional<A> a;
    @JsonProperty("b")
    final Optional<A.B> b;

    @JsonCreator
    public Entry(
        @JsonProperty("a") Optional<A> a,
        @JsonProperty("b") Optional<A.B> b
    ) {
        this.a = a;
        this.b = b;
    }

    @JsonProperty("a")
    public Optional<A> getA() {
        return this.a;
    }

    @JsonProperty("b")
    public Optional<A.B> getB() {
        return this.b;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("a=");
        b.append(this.a.toString());
        b.append(", ");
        b.append("b=");
        b.append(this.b.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.a.hashCode();
        result = result * 31 + this.b.hashCode();
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

        if (!this.a.equals(o_.a)) {
            return false;
        }

        if (!this.b.equals(o_.b)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<A> a;
        private Optional<A.B> b;

        private Builder() {
            this.a = Optional.empty();
            this.b = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.a,
                this.b
            );
        }

        public Builder a(final A a) {
            this.a = Optional.of(a);
            return this;
        }

        public Builder b(final A.B b) {
            this.b = Optional.of(b);
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
