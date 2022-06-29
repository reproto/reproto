package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Complex object.
 */
public class Other {
    @JsonProperty("a")
    final String a;

    @JsonCreator
    public Other(
        @JsonProperty("a") String a
    ) {
        Objects.requireNonNull(a, "a: must not be null");
        this.a = a;
    }

    @JsonProperty("a")
    public String getA() {
        return this.a;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Other(");
        b.append("a=");
        b.append(this.a.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.a.hashCode();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof Other)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Other o_ = (Other)other_;

        if (!this.a.equals(o_.a)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<String> a;

        private Builder() {
            this.a = Optional.empty();
        }

        public Other build() {
            final String a = this.a
                .orElseThrow(() -> new RuntimeException("a: missing required value"));

            return new Other(
                a
            );
        }

        public Builder a(final String a) {
            this.a = Optional.of(a);
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
