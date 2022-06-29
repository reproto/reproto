package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("foo")
    final Optional<Foo> foo;

    @JsonCreator
    public Entry(
        @JsonProperty("foo") Optional<Foo> foo
    ) {
        this.foo = foo;
    }

    /**
     * The foo field.
     */
    @JsonProperty("foo")
    public Optional<Foo> getFoo() {
        return this.foo;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("foo=");
        b.append(this.foo.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.foo.hashCode();
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

        if (!this.foo.equals(o_.foo)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<Foo> foo;

        private Builder() {
            this.foo = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.foo
            );
        }

        public Builder foo(final Foo foo) {
            this.foo = Optional.of(foo);
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
