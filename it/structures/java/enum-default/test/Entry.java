package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;

public class Entry {
    @JsonProperty("explicit")
    final Optional<EnumExplicit> explicit;
    @JsonProperty("implicit")
    final Optional<EnumImplicit> implicit;
    @JsonProperty("enum_u32")
    final Optional<EnumU32> enumU32;
    @JsonProperty("enum_u64")
    final Optional<EnumU64> enumU64;
    @JsonProperty("enum_i32")
    final Optional<EnumI32> enumI32;
    @JsonProperty("enum_i64")
    final Optional<EnumI64> enumI64;

    @JsonCreator
    public Entry(
        @JsonProperty("explicit") Optional<EnumExplicit> explicit,
        @JsonProperty("implicit") Optional<EnumImplicit> implicit,
        @JsonProperty("enum_u32") Optional<EnumU32> enumU32,
        @JsonProperty("enum_u64") Optional<EnumU64> enumU64,
        @JsonProperty("enum_i32") Optional<EnumI32> enumI32,
        @JsonProperty("enum_i64") Optional<EnumI64> enumI64
    ) {
        this.explicit = explicit;
        this.implicit = implicit;
        this.enumU32 = enumU32;
        this.enumU64 = enumU64;
        this.enumI32 = enumI32;
        this.enumI64 = enumI64;
    }

    @JsonProperty("explicit")
    public Optional<EnumExplicit> getExplicit() {
        return this.explicit;
    }

    @JsonProperty("implicit")
    public Optional<EnumImplicit> getImplicit() {
        return this.implicit;
    }

    @JsonProperty("enum_u32")
    public Optional<EnumU32> getEnumU32() {
        return this.enumU32;
    }

    @JsonProperty("enum_u64")
    public Optional<EnumU64> getEnumU64() {
        return this.enumU64;
    }

    @JsonProperty("enum_i32")
    public Optional<EnumI32> getEnumI32() {
        return this.enumI32;
    }

    @JsonProperty("enum_i64")
    public Optional<EnumI64> getEnumI64() {
        return this.enumI64;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("explicit=");
        b.append(this.explicit.toString());
        b.append(", ");
        b.append("implicit=");
        b.append(this.implicit.toString());
        b.append(", ");
        b.append("enum_u32=");
        b.append(this.enumU32.toString());
        b.append(", ");
        b.append("enum_u64=");
        b.append(this.enumU64.toString());
        b.append(", ");
        b.append("enum_i32=");
        b.append(this.enumI32.toString());
        b.append(", ");
        b.append("enum_i64=");
        b.append(this.enumI64.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.explicit.hashCode();
        result = result * 31 + this.implicit.hashCode();
        result = result * 31 + this.enumU32.hashCode();
        result = result * 31 + this.enumU64.hashCode();
        result = result * 31 + this.enumI32.hashCode();
        result = result * 31 + this.enumI64.hashCode();
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

        if (!this.explicit.equals(o_.explicit)) {
            return false;
        }

        if (!this.implicit.equals(o_.implicit)) {
            return false;
        }

        if (!this.enumU32.equals(o_.enumU32)) {
            return false;
        }

        if (!this.enumU64.equals(o_.enumU64)) {
            return false;
        }

        if (!this.enumI32.equals(o_.enumI32)) {
            return false;
        }

        if (!this.enumI64.equals(o_.enumI64)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<EnumExplicit> explicit;
        private Optional<EnumImplicit> implicit;
        private Optional<EnumU32> enumU32;
        private Optional<EnumU64> enumU64;
        private Optional<EnumI32> enumI32;
        private Optional<EnumI64> enumI64;

        private Builder() {
            this.explicit = Optional.empty();
            this.implicit = Optional.empty();
            this.enumU32 = Optional.empty();
            this.enumU64 = Optional.empty();
            this.enumI32 = Optional.empty();
            this.enumI64 = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.explicit,
                this.implicit,
                this.enumU32,
                this.enumU64,
                this.enumI32,
                this.enumI64
            );
        }

        public Builder explicit(final EnumExplicit explicit) {
            this.explicit = Optional.of(explicit);
            return this;
        }

        public Builder implicit(final EnumImplicit implicit) {
            this.implicit = Optional.of(implicit);
            return this;
        }

        public Builder enumU32(final EnumU32 enumU32) {
            this.enumU32 = Optional.of(enumU32);
            return this;
        }

        public Builder enumU64(final EnumU64 enumU64) {
            this.enumU64 = Optional.of(enumU64);
            return this;
        }

        public Builder enumI32(final EnumI32 enumI32) {
            this.enumI32 = Optional.of(enumI32);
            return this;
        }

        public Builder enumI64(final EnumI64 enumI64) {
            this.enumI64 = Optional.of(enumI64);
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
