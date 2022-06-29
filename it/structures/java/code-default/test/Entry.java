package test;

import com.fasterxml.jackson.annotation.JsonCreator;

public class Entry {

    @JsonCreator
    public Entry() {}

    @Override
    public String toString() {
        return "Entry()";
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
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

        return true;
    }

    public static class Builder {

        private Builder() {}

        public Entry build() {

            return new Entry();
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }
}
