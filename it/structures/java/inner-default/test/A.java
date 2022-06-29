package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class A {
    @JsonProperty("b")
    final A.B b;

    @JsonCreator
    public A(
        @JsonProperty("b") A.B b
    ) {
        Objects.requireNonNull(b, "b: must not be null");
        this.b = b;
    }

    @JsonProperty("b")
    public A.B getB() {
        return this.b;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("A(");
        b.append("b=");
        b.append(this.b.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.b.hashCode();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof A)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final A o_ = (A)other_;

        if (!this.b.equals(o_.b)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<A.B> b;

        private Builder() {
            this.b = Optional.empty();
        }

        public A build() {
            final A.B b = this.b
                .orElseThrow(() -> new RuntimeException("b: missing required value"));

            return new A(
                b
            );
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

    public static class B {
        @JsonProperty("field")
        final String field;

        @JsonCreator
        public B(
            @JsonProperty("field") String field
        ) {
            Objects.requireNonNull(field, "field: must not be null");
            this.field = field;
        }

        @JsonProperty("field")
        public String getField() {
            return this.field;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("B(");
            b.append("field=");
            b.append(this.field.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.field.hashCode();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof B)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final B o_ = (B)other_;

            if (!this.field.equals(o_.field)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> field;

            private Builder() {
                this.field = Optional.empty();
            }

            public B build() {
                final String field = this.field
                    .orElseThrow(() -> new RuntimeException("field: missing required value"));

                return new B(
                    field
                );
            }

            public Builder field(final String field) {
                this.field = Optional.of(field);
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
}
