package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Foo {
    @JsonProperty("field")
    final String field;

    @JsonCreator
    public Foo(
        @JsonProperty("field") String field
    ) {
        Objects.requireNonNull(field, "field: must not be null");
        this.field = field;
    }

    /**
     * The field.
     */
    @JsonProperty("field")
    public String getField() {
        return this.field;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Foo(");
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

        if (!(other_ instanceof Foo)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Foo o_ = (Foo)other_;

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

        public Foo build() {
            final String field = this.field
                .orElseThrow(() -> new RuntimeException("field: missing required value"));

            return new Foo(
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
