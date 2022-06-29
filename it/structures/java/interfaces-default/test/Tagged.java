package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import java.util.Objects;
import java.util.Optional;

@JsonTypeInfo(use=JsonTypeInfo.Id.NAME, include=JsonTypeInfo.As.PROPERTY, property="@type")
@JsonSubTypes({
    @JsonSubTypes.Type(name="foo", value=Tagged.A.class),
    @JsonSubTypes.Type(name="b", value=Tagged.B.class),
    @JsonSubTypes.Type(name="Bar", value=Tagged.Bar.class),
    @JsonSubTypes.Type(name="Baz", value=Tagged.Baz.class),
})
public interface Tagged {
    public String getShared();

    public static class A implements Tagged {
        @JsonProperty("shared")
        final String shared;

        @JsonCreator
        public A(
            @JsonProperty("shared") String shared
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("A(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
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

            if (!this.shared.equals(o_.shared)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;

            private Builder() {
                this.shared = Optional.empty();
            }

            public A build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));

                return new A(
                    shared
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
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

    public static class B implements Tagged {
        @JsonProperty("shared")
        final String shared;

        @JsonCreator
        public B(
            @JsonProperty("shared") String shared
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("B(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
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

            if (!this.shared.equals(o_.shared)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;

            private Builder() {
                this.shared = Optional.empty();
            }

            public B build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));

                return new B(
                    shared
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
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

    public static class Bar implements Tagged {
        @JsonProperty("shared")
        final String shared;

        @JsonCreator
        public Bar(
            @JsonProperty("shared") String shared
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("Bar(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof Bar)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final Bar o_ = (Bar)other_;

            if (!this.shared.equals(o_.shared)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;

            private Builder() {
                this.shared = Optional.empty();
            }

            public Bar build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));

                return new Bar(
                    shared
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
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

    public static class Baz implements Tagged {
        @JsonProperty("shared")
        final String shared;

        @JsonCreator
        public Baz(
            @JsonProperty("shared") String shared
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("Baz(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof Baz)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final Baz o_ = (Baz)other_;

            if (!this.shared.equals(o_.shared)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;

            private Builder() {
                this.shared = Optional.empty();
            }

            public Baz build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));

                return new Baz(
                    shared
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
                return this;
            }
        }

        /**
         * Construct a new builder.
         */
        public static Builder builder() {
            return new Builder();
        }
    };
}
