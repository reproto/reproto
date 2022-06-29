package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.nio.ByteBuffer;
import java.time.Instant;
import java.util.List;
import java.util.Map;
import java.util.Optional;

public class Entry {
    @JsonProperty("boolean_type")
    final Optional<Boolean> booleanType;
    @JsonProperty("string_type")
    final Optional<String> stringType;
    @JsonProperty("datetime_type")
    @JsonFormat(shape = JsonFormat.Shape.STRING)
    final Optional<Instant> datetimeType;
    @JsonProperty("unsigned_32")
    final Optional<Integer> unsigned32;
    @JsonProperty("unsigned_64")
    final Optional<Long> unsigned64;
    @JsonProperty("signed_32")
    final Optional<Integer> signed32;
    @JsonProperty("signed_64")
    final Optional<Long> signed64;
    @JsonProperty("float_type")
    final Optional<Float> floatType;
    @JsonProperty("double_type")
    final Optional<Double> doubleType;
    @JsonProperty("bytes_type")
    final Optional<ByteBuffer> bytesType;
    @JsonProperty("any_type")
    final Optional<Object> anyType;
    @JsonProperty("array_type")
    final Optional<List<Entry>> arrayType;
    @JsonProperty("array_of_array_type")
    final Optional<List<List<Entry>>> arrayOfArrayType;
    @JsonProperty("map_type")
    final Optional<Map<String, Entry>> mapType;

    @JsonCreator
    public Entry(
        @JsonProperty("boolean_type") Optional<Boolean> booleanType,
        @JsonProperty("string_type") Optional<String> stringType,
        @JsonProperty("datetime_type") Optional<Instant> datetimeType,
        @JsonProperty("unsigned_32") Optional<Integer> unsigned32,
        @JsonProperty("unsigned_64") Optional<Long> unsigned64,
        @JsonProperty("signed_32") Optional<Integer> signed32,
        @JsonProperty("signed_64") Optional<Long> signed64,
        @JsonProperty("float_type") Optional<Float> floatType,
        @JsonProperty("double_type") Optional<Double> doubleType,
        @JsonProperty("bytes_type") Optional<ByteBuffer> bytesType,
        @JsonProperty("any_type") Optional<Object> anyType,
        @JsonProperty("array_type") Optional<List<Entry>> arrayType,
        @JsonProperty("array_of_array_type") Optional<List<List<Entry>>> arrayOfArrayType,
        @JsonProperty("map_type") Optional<Map<String, Entry>> mapType
    ) {
        this.booleanType = booleanType;
        this.stringType = stringType;
        this.datetimeType = datetimeType;
        this.unsigned32 = unsigned32;
        this.unsigned64 = unsigned64;
        this.signed32 = signed32;
        this.signed64 = signed64;
        this.floatType = floatType;
        this.doubleType = doubleType;
        this.bytesType = bytesType;
        this.anyType = anyType;
        this.arrayType = arrayType;
        this.arrayOfArrayType = arrayOfArrayType;
        this.mapType = mapType;
    }

    @JsonProperty("boolean_type")
    public Optional<Boolean> getBooleanType() {
        return this.booleanType;
    }

    @JsonProperty("string_type")
    public Optional<String> getStringType() {
        return this.stringType;
    }

    @JsonProperty("datetime_type")
    public Optional<Instant> getDatetimeType() {
        return this.datetimeType;
    }

    @JsonProperty("unsigned_32")
    public Optional<Integer> getUnsigned32() {
        return this.unsigned32;
    }

    @JsonProperty("unsigned_64")
    public Optional<Long> getUnsigned64() {
        return this.unsigned64;
    }

    @JsonProperty("signed_32")
    public Optional<Integer> getSigned32() {
        return this.signed32;
    }

    @JsonProperty("signed_64")
    public Optional<Long> getSigned64() {
        return this.signed64;
    }

    @JsonProperty("float_type")
    public Optional<Float> getFloatType() {
        return this.floatType;
    }

    @JsonProperty("double_type")
    public Optional<Double> getDoubleType() {
        return this.doubleType;
    }

    @JsonProperty("bytes_type")
    public Optional<ByteBuffer> getBytesType() {
        return this.bytesType;
    }

    @JsonProperty("any_type")
    public Optional<Object> getAnyType() {
        return this.anyType;
    }

    @JsonProperty("array_type")
    public Optional<List<Entry>> getArrayType() {
        return this.arrayType;
    }

    @JsonProperty("array_of_array_type")
    public Optional<List<List<Entry>>> getArrayOfArrayType() {
        return this.arrayOfArrayType;
    }

    @JsonProperty("map_type")
    public Optional<Map<String, Entry>> getMapType() {
        return this.mapType;
    }

    @Override
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
        b.append("boolean_type=");
        b.append(this.booleanType.toString());
        b.append(", ");
        b.append("string_type=");
        b.append(this.stringType.toString());
        b.append(", ");
        b.append("datetime_type=");
        b.append(this.datetimeType.toString());
        b.append(", ");
        b.append("unsigned_32=");
        b.append(this.unsigned32.toString());
        b.append(", ");
        b.append("unsigned_64=");
        b.append(this.unsigned64.toString());
        b.append(", ");
        b.append("signed_32=");
        b.append(this.signed32.toString());
        b.append(", ");
        b.append("signed_64=");
        b.append(this.signed64.toString());
        b.append(", ");
        b.append("float_type=");
        b.append(this.floatType.toString());
        b.append(", ");
        b.append("double_type=");
        b.append(this.doubleType.toString());
        b.append(", ");
        b.append("bytes_type=");
        b.append(this.bytesType.toString());
        b.append(", ");
        b.append("any_type=");
        b.append(this.anyType.toString());
        b.append(", ");
        b.append("array_type=");
        b.append(this.arrayType.toString());
        b.append(", ");
        b.append("array_of_array_type=");
        b.append(this.arrayOfArrayType.toString());
        b.append(", ");
        b.append("map_type=");
        b.append(this.mapType.toString());
        b.append(")");

        return b.toString();
    }

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.booleanType.hashCode();
        result = result * 31 + this.stringType.hashCode();
        result = result * 31 + this.datetimeType.hashCode();
        result = result * 31 + this.unsigned32.hashCode();
        result = result * 31 + this.unsigned64.hashCode();
        result = result * 31 + this.signed32.hashCode();
        result = result * 31 + this.signed64.hashCode();
        result = result * 31 + this.floatType.hashCode();
        result = result * 31 + this.doubleType.hashCode();
        result = result * 31 + this.bytesType.hashCode();
        result = result * 31 + this.anyType.hashCode();
        result = result * 31 + this.arrayType.hashCode();
        result = result * 31 + this.arrayOfArrayType.hashCode();
        result = result * 31 + this.mapType.hashCode();
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

        if (!this.booleanType.equals(o_.booleanType)) {
            return false;
        }

        if (!this.stringType.equals(o_.stringType)) {
            return false;
        }

        if (!this.datetimeType.equals(o_.datetimeType)) {
            return false;
        }

        if (!this.unsigned32.equals(o_.unsigned32)) {
            return false;
        }

        if (!this.unsigned64.equals(o_.unsigned64)) {
            return false;
        }

        if (!this.signed32.equals(o_.signed32)) {
            return false;
        }

        if (!this.signed64.equals(o_.signed64)) {
            return false;
        }

        if (!this.floatType.equals(o_.floatType)) {
            return false;
        }

        if (!this.doubleType.equals(o_.doubleType)) {
            return false;
        }

        if (!this.bytesType.equals(o_.bytesType)) {
            return false;
        }

        if (!this.anyType.equals(o_.anyType)) {
            return false;
        }

        if (!this.arrayType.equals(o_.arrayType)) {
            return false;
        }

        if (!this.arrayOfArrayType.equals(o_.arrayOfArrayType)) {
            return false;
        }

        if (!this.mapType.equals(o_.mapType)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<Boolean> booleanType;
        private Optional<String> stringType;
        private Optional<Instant> datetimeType;
        private Optional<Integer> unsigned32;
        private Optional<Long> unsigned64;
        private Optional<Integer> signed32;
        private Optional<Long> signed64;
        private Optional<Float> floatType;
        private Optional<Double> doubleType;
        private Optional<ByteBuffer> bytesType;
        private Optional<Object> anyType;
        private Optional<List<Entry>> arrayType;
        private Optional<List<List<Entry>>> arrayOfArrayType;
        private Optional<Map<String, Entry>> mapType;

        private Builder() {
            this.booleanType = Optional.empty();
            this.stringType = Optional.empty();
            this.datetimeType = Optional.empty();
            this.unsigned32 = Optional.empty();
            this.unsigned64 = Optional.empty();
            this.signed32 = Optional.empty();
            this.signed64 = Optional.empty();
            this.floatType = Optional.empty();
            this.doubleType = Optional.empty();
            this.bytesType = Optional.empty();
            this.anyType = Optional.empty();
            this.arrayType = Optional.empty();
            this.arrayOfArrayType = Optional.empty();
            this.mapType = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.booleanType,
                this.stringType,
                this.datetimeType,
                this.unsigned32,
                this.unsigned64,
                this.signed32,
                this.signed64,
                this.floatType,
                this.doubleType,
                this.bytesType,
                this.anyType,
                this.arrayType,
                this.arrayOfArrayType,
                this.mapType
            );
        }

        public Builder booleanType(final boolean booleanType) {
            this.booleanType = Optional.of(booleanType);
            return this;
        }

        public Builder stringType(final String stringType) {
            this.stringType = Optional.of(stringType);
            return this;
        }

        public Builder datetimeType(final Instant datetimeType) {
            this.datetimeType = Optional.of(datetimeType);
            return this;
        }

        public Builder unsigned32(final int unsigned32) {
            this.unsigned32 = Optional.of(unsigned32);
            return this;
        }

        public Builder unsigned64(final long unsigned64) {
            this.unsigned64 = Optional.of(unsigned64);
            return this;
        }

        public Builder signed32(final int signed32) {
            this.signed32 = Optional.of(signed32);
            return this;
        }

        public Builder signed64(final long signed64) {
            this.signed64 = Optional.of(signed64);
            return this;
        }

        public Builder floatType(final float floatType) {
            this.floatType = Optional.of(floatType);
            return this;
        }

        public Builder doubleType(final double doubleType) {
            this.doubleType = Optional.of(doubleType);
            return this;
        }

        public Builder bytesType(final ByteBuffer bytesType) {
            this.bytesType = Optional.of(bytesType);
            return this;
        }

        public Builder anyType(final Object anyType) {
            this.anyType = Optional.of(anyType);
            return this;
        }

        public Builder arrayType(final List<Entry> arrayType) {
            this.arrayType = Optional.of(arrayType);
            return this;
        }

        public Builder arrayOfArrayType(final List<List<Entry>> arrayOfArrayType) {
            this.arrayOfArrayType = Optional.of(arrayOfArrayType);
            return this;
        }

        public Builder mapType(final Map<String, Entry> mapType) {
            this.mapType = Optional.of(mapType);
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
