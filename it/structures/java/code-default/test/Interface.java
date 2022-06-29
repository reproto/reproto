package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="type")
@JsonSubTypes({
    @JsonSubTypes.Type(name="SubType", value=Interface.SubType.class),
})
public interface Interface {

    public void interfaceMethod();

    public static class SubType implements Interface {

        @JsonCreator
        public SubType() {}

        @Override
        public String toString() {
            return "SubType()";
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

            if (!(other_ instanceof SubType)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final SubType o_ = (SubType)other_;

            return true;
        }

        public static class Builder {

            private Builder() {}

            public SubType build() {

                return new SubType();
            }
        }

        /**
         * Construct a new builder.
         */
        public static Builder builder() {
            return new Builder();
        }

        @Override
        public void interfaceMethod() {
        }
    };
}
