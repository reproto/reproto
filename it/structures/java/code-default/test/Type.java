package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class Type {

    @JsonCreator
    public Type() {}

    @Override
    public String toString() {
        return "Type()";
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

        if (!(other_ instanceof Type)) {
            return false;
        }

        @SuppressWarnings("unchecked")
        final Type o_ = (Type)other_;

        return true;
    }

    public static class Builder {

        private Builder() {}

        public Type build() {

            return new Type();
        }
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }

    public List<Map<String, String>> typeMethod() {
      return new ArrayList<>();
    }
}
