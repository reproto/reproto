package _true;

import com.fasterxml.jackson.annotation.JsonCreator;

public class Empty {

    @JsonCreator
    public Empty() {}

    @Override
    public String toString() {
        return "Empty()";
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

        if (!(other_ instanceof Empty)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Empty o_ = (Empty)other_;

        return true;
    }

    public static class Builder {

        private Builder() {}

        public Empty build() {

            return new Empty();
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }
}
