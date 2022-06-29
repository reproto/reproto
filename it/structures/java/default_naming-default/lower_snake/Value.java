package lower_snake;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

public class Value {
    @JsonProperty("foo_bar")
    final String fooBar;

    @JsonCreator
    public Value(
        @JsonProperty("foo_bar") String fooBar
    ) {
        Objects.requireNonNull(fooBar, "foo_bar: must not be null");
        this.fooBar = fooBar;
    }

    @JsonProperty("foo_bar")
    public String getFooBar() {
        return this.fooBar;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Value(");
        b.append("foo_bar=");
        b.append(this.fooBar.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.fooBar.hashCode();
        return result;
    }

    @Override
    public boolean equals(final Object other_) {
        if (other_ == null) {
            return false;
        }

        if (!(other_ instanceof Value)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Value o_ = (Value)other_;

        if (!this.fooBar.equals(o_.fooBar)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<String> fooBar;

        private Builder() {
            this.fooBar = Optional.empty();
        }

        public Value build() {
            final String fooBar = this.fooBar
                .orElseThrow(() -> new RuntimeException("foo_bar: missing required value"));

            return new Value(
                fooBar
            );
        }

        public Builder fooBar(final String fooBar) {
            this.fooBar = Optional.of(fooBar);
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
