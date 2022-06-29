package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("tagged")
    final Optional<Tagged> tagged;
    @JsonProperty("untagged")
    final Optional<Untagged> untagged;

    @JsonCreator
    public Entry(
        @JsonProperty("tagged") Optional<Tagged> tagged,
        @JsonProperty("untagged") Optional<Untagged> untagged
    ) {
        this.tagged = tagged;
        this.untagged = untagged;
    }

    @JsonProperty("tagged")
    public Optional<Tagged> getTagged() {
        return this.tagged;
    }

    @JsonProperty("untagged")
    public Optional<Untagged> getUntagged() {
        return this.untagged;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("tagged=");
        b.append(this.tagged.toString());
        b.append(", ");
        b.append("untagged=");
        b.append(this.untagged.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.tagged.hashCode();
        result = result * 31 + this.untagged.hashCode();
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

        if (!this.tagged.equals(o_.tagged)) {
            return false;
        }

        if (!this.untagged.equals(o_.untagged)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<Tagged> tagged;
        private Optional<Untagged> untagged;

        private Builder() {
            this.tagged = Optional.empty();
            this.untagged = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.tagged,
                this.untagged
            );
        }

        public Builder tagged(final Tagged tagged) {
            this.tagged = Optional.of(tagged);
            return this;
        }

        public Builder untagged(final Untagged untagged) {
            this.untagged = Optional.of(untagged);
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
