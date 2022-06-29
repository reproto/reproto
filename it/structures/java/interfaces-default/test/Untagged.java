package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.fasterxml.jackson.databind.node.TreeTraversingParser;
import java.io.IOException;
import java.util.HashSet;
import java.util.Iterator;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;

@JsonDeserialize(using = Untagged.Deserializer.class)
public interface Untagged {
    public String getShared();

    public Optional<String> getSharedIgnore();

    /**
     * Special case: fields shared with other sub-types.
     * NOTE: due to rust support through untagged, the types are matched in-order.
     */
    @JsonDeserialize(using = JsonDeserializer.None.class)
    public static class A implements Untagged {
        @JsonProperty("shared")
        final String shared;
        @JsonProperty("shared_ignore")
        final Optional<String> sharedIgnore;
        @JsonProperty("a")
        final String a;
        @JsonProperty("b")
        final String b;
        @JsonProperty("ignore")
        final Optional<String> ignore;

        @JsonCreator
        public A(
            @JsonProperty("shared") String shared,
            @JsonProperty("shared_ignore") Optional<String> sharedIgnore,
            @JsonProperty("a") String a,
            @JsonProperty("b") String b,
            @JsonProperty("ignore") Optional<String> ignore
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
            this.sharedIgnore = sharedIgnore;
            Objects.requireNonNull(a, "a: must not be null");
            this.a = a;
            Objects.requireNonNull(b, "b: must not be null");
            this.b = b;
            this.ignore = ignore;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @JsonProperty("shared_ignore")
        @Override
        public Optional<String> getSharedIgnore() {
            return this.sharedIgnore;
        }

        @JsonProperty("a")
        public String getA() {
            return this.a;
        }

        @JsonProperty("b")
        public String getB() {
            return this.b;
        }

        @JsonProperty("ignore")
        public Optional<String> getIgnore() {
            return this.ignore;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("A(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(", ");
            b.append("shared_ignore=");
            b.append(this.sharedIgnore.toString());
            b.append(", ");
            b.append("a=");
            b.append(this.a.toString());
            b.append(", ");
            b.append("b=");
            b.append(this.b.toString());
            b.append(", ");
            b.append("ignore=");
            b.append(this.ignore.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
            result = result * 31 + this.sharedIgnore.hashCode();
            result = result * 31 + this.a.hashCode();
            result = result * 31 + this.b.hashCode();
            result = result * 31 + this.ignore.hashCode();
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

            if (!this.sharedIgnore.equals(o_.sharedIgnore)) {
                return false;
            }

            if (!this.a.equals(o_.a)) {
                return false;
            }

            if (!this.b.equals(o_.b)) {
                return false;
            }

            if (!this.ignore.equals(o_.ignore)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;
            private Optional<String> sharedIgnore;
            private Optional<String> a;
            private Optional<String> b;
            private Optional<String> ignore;

            private Builder() {
                this.shared = Optional.empty();
                this.sharedIgnore = Optional.empty();
                this.a = Optional.empty();
                this.b = Optional.empty();
                this.ignore = Optional.empty();
            }

            public A build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));
                final String a = this.a
                    .orElseThrow(() -> new RuntimeException("a: missing required value"));
                final String b = this.b
                    .orElseThrow(() -> new RuntimeException("b: missing required value"));

                return new A(
                    shared,
                    this.sharedIgnore,
                    a,
                    b,
                    this.ignore
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
                return this;
            }

            public Builder sharedIgnore(final String sharedIgnore) {
                this.sharedIgnore = Optional.of(sharedIgnore);
                return this;
            }

            public Builder a(final String a) {
                this.a = Optional.of(a);
                return this;
            }

            public Builder b(final String b) {
                this.b = Optional.of(b);
                return this;
            }

            public Builder ignore(final String ignore) {
                this.ignore = Optional.of(ignore);
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

    @JsonDeserialize(using = JsonDeserializer.None.class)
    public static class B implements Untagged {
        @JsonProperty("shared")
        final String shared;
        @JsonProperty("shared_ignore")
        final Optional<String> sharedIgnore;
        @JsonProperty("a")
        final String a;
        @JsonProperty("ignore")
        final Optional<String> ignore;

        @JsonCreator
        public B(
            @JsonProperty("shared") String shared,
            @JsonProperty("shared_ignore") Optional<String> sharedIgnore,
            @JsonProperty("a") String a,
            @JsonProperty("ignore") Optional<String> ignore
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
            this.sharedIgnore = sharedIgnore;
            Objects.requireNonNull(a, "a: must not be null");
            this.a = a;
            this.ignore = ignore;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @JsonProperty("shared_ignore")
        @Override
        public Optional<String> getSharedIgnore() {
            return this.sharedIgnore;
        }

        @JsonProperty("a")
        public String getA() {
            return this.a;
        }

        @JsonProperty("ignore")
        public Optional<String> getIgnore() {
            return this.ignore;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("B(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(", ");
            b.append("shared_ignore=");
            b.append(this.sharedIgnore.toString());
            b.append(", ");
            b.append("a=");
            b.append(this.a.toString());
            b.append(", ");
            b.append("ignore=");
            b.append(this.ignore.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
            result = result * 31 + this.sharedIgnore.hashCode();
            result = result * 31 + this.a.hashCode();
            result = result * 31 + this.ignore.hashCode();
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

            if (!this.sharedIgnore.equals(o_.sharedIgnore)) {
                return false;
            }

            if (!this.a.equals(o_.a)) {
                return false;
            }

            if (!this.ignore.equals(o_.ignore)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;
            private Optional<String> sharedIgnore;
            private Optional<String> a;
            private Optional<String> ignore;

            private Builder() {
                this.shared = Optional.empty();
                this.sharedIgnore = Optional.empty();
                this.a = Optional.empty();
                this.ignore = Optional.empty();
            }

            public B build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));
                final String a = this.a
                    .orElseThrow(() -> new RuntimeException("a: missing required value"));

                return new B(
                    shared,
                    this.sharedIgnore,
                    a,
                    this.ignore
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
                return this;
            }

            public Builder sharedIgnore(final String sharedIgnore) {
                this.sharedIgnore = Optional.of(sharedIgnore);
                return this;
            }

            public Builder a(final String a) {
                this.a = Optional.of(a);
                return this;
            }

            public Builder ignore(final String ignore) {
                this.ignore = Optional.of(ignore);
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

    @JsonDeserialize(using = JsonDeserializer.None.class)
    public static class C implements Untagged {
        @JsonProperty("shared")
        final String shared;
        @JsonProperty("shared_ignore")
        final Optional<String> sharedIgnore;
        @JsonProperty("b")
        final String b;
        @JsonProperty("ignore")
        final Optional<String> ignore;

        @JsonCreator
        public C(
            @JsonProperty("shared") String shared,
            @JsonProperty("shared_ignore") Optional<String> sharedIgnore,
            @JsonProperty("b") String b,
            @JsonProperty("ignore") Optional<String> ignore
        ) {
            Objects.requireNonNull(shared, "shared: must not be null");
            this.shared = shared;
            this.sharedIgnore = sharedIgnore;
            Objects.requireNonNull(b, "b: must not be null");
            this.b = b;
            this.ignore = ignore;
        }

        @JsonProperty("shared")
        @Override
        public String getShared() {
            return this.shared;
        }

        @JsonProperty("shared_ignore")
        @Override
        public Optional<String> getSharedIgnore() {
            return this.sharedIgnore;
        }

        @JsonProperty("b")
        public String getB() {
            return this.b;
        }

        @JsonProperty("ignore")
        public Optional<String> getIgnore() {
            return this.ignore;
        }

        @Override
        public String toString() {
            final StringBuilder b = new StringBuilder();

            b.append("C(");
            b.append("shared=");
            b.append(this.shared.toString());
            b.append(", ");
            b.append("shared_ignore=");
            b.append(this.sharedIgnore.toString());
            b.append(", ");
            b.append("b=");
            b.append(this.b.toString());
            b.append(", ");
            b.append("ignore=");
            b.append(this.ignore.toString());
            b.append(")");

            return b.toString();
        }

        @Override
        public int hashCode() {
            int result = 1;
            final StringBuilder b = new StringBuilder();
            result = result * 31 + this.shared.hashCode();
            result = result * 31 + this.sharedIgnore.hashCode();
            result = result * 31 + this.b.hashCode();
            result = result * 31 + this.ignore.hashCode();
            return result;
        }

        @Override
        public boolean equals(final Object other_) {
            if (other_ == null) {
                return false;
            }

            if (!(other_ instanceof C)) {
                return false;
            }

            @SuppressWarnings("unchecked")
            final C o_ = (C)other_;

            if (!this.shared.equals(o_.shared)) {
                return false;
            }

            if (!this.sharedIgnore.equals(o_.sharedIgnore)) {
                return false;
            }

            if (!this.b.equals(o_.b)) {
                return false;
            }

            if (!this.ignore.equals(o_.ignore)) {
                return false;
            }

            return true;
        }

        public static class Builder {
            private Optional<String> shared;
            private Optional<String> sharedIgnore;
            private Optional<String> b;
            private Optional<String> ignore;

            private Builder() {
                this.shared = Optional.empty();
                this.sharedIgnore = Optional.empty();
                this.b = Optional.empty();
                this.ignore = Optional.empty();
            }

            public C build() {
                final String shared = this.shared
                    .orElseThrow(() -> new RuntimeException("shared: missing required value"));
                final String b = this.b
                    .orElseThrow(() -> new RuntimeException("b: missing required value"));

                return new C(
                    shared,
                    this.sharedIgnore,
                    b,
                    this.ignore
                );
            }

            public Builder shared(final String shared) {
                this.shared = Optional.of(shared);
                return this;
            }

            public Builder sharedIgnore(final String sharedIgnore) {
                this.sharedIgnore = Optional.of(sharedIgnore);
                return this;
            }

            public Builder b(final String b) {
                this.b = Optional.of(b);
                return this;
            }

            public Builder ignore(final String ignore) {
                this.ignore = Optional.of(ignore);
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

    public static class Deserializer extends JsonDeserializer<Untagged> {
        @Override
        public Untagged deserialize(final JsonParser parser, final DeserializationContext ctxt) throws IOException {
            final ObjectNode object = parser.readValueAs(ObjectNode.class);

            final Set<String> tags = new HashSet<String>();
            final Iterator<String> it = object.fieldNames();

            while (it.hasNext()) {
                tags.add(it.next());
            }

            if (tags.contains("a") && tags.contains("b")) {
                return new TreeTraversingParser(object, parser.getCodec()).readValueAs(Untagged.A.class);
            }

            if (tags.contains("a")) {
                return new TreeTraversingParser(object, parser.getCodec()).readValueAs(Untagged.B.class);
            }

            if (tags.contains("b")) {
                return new TreeTraversingParser(object, parser.getCodec()).readValueAs(Untagged.C.class);
            }

            throw ctxt.mappingException("no legal combination of fields available");
        }
    }
}
