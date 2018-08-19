package service;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.nio.ByteBuffer;
import java.time.Instant;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  @JsonProperty("boolean_type")
  private final Optional<Boolean> booleanType;
  @JsonProperty("string_type")
  private final Optional<String> stringType;
  @JsonProperty("datetime_type")
  @JsonFormat(shape = JsonFormat.Shape.STRING)
  private final Optional<Instant> datetimeType;
  @JsonProperty("unsigned_32")
  private final Optional<Integer> unsigned32;
  @JsonProperty("unsigned_64")
  private final Optional<Long> unsigned64;
  @JsonProperty("signed_32")
  private final Optional<Integer> signed32;
  @JsonProperty("signed_64")
  private final Optional<Long> signed64;
  @JsonProperty("float_type")
  private final Optional<Float> floatType;
  @JsonProperty("double_type")
  private final Optional<Double> doubleType;
  @JsonProperty("bytes_type")
  private final Optional<ByteBuffer> bytesType;
  @JsonProperty("any_type")
  private final Optional<Object> anyType;

  @JsonCreator
  public Entry(
    @JsonProperty("boolean_type") final Optional<Boolean> booleanType,
    @JsonProperty("string_type") final Optional<String> stringType,
    @JsonProperty("datetime_type") final Optional<Instant> datetimeType,
    @JsonProperty("unsigned_32") final Optional<Integer> unsigned32,
    @JsonProperty("unsigned_64") final Optional<Long> unsigned64,
    @JsonProperty("signed_32") final Optional<Integer> signed32,
    @JsonProperty("signed_64") final Optional<Long> signed64,
    @JsonProperty("float_type") final Optional<Float> floatType,
    @JsonProperty("double_type") final Optional<Double> doubleType,
    @JsonProperty("bytes_type") final Optional<ByteBuffer> bytesType,
    @JsonProperty("any_type") final Optional<Object> anyType
  ) {
    Objects.requireNonNull(booleanType, "boolean_type");
    this.booleanType = booleanType;
    Objects.requireNonNull(stringType, "string_type");
    this.stringType = stringType;
    Objects.requireNonNull(datetimeType, "datetime_type");
    this.datetimeType = datetimeType;
    Objects.requireNonNull(unsigned32, "unsigned_32");
    this.unsigned32 = unsigned32;
    Objects.requireNonNull(unsigned64, "unsigned_64");
    this.unsigned64 = unsigned64;
    Objects.requireNonNull(signed32, "signed_32");
    this.signed32 = signed32;
    Objects.requireNonNull(signed64, "signed_64");
    this.signed64 = signed64;
    Objects.requireNonNull(floatType, "float_type");
    this.floatType = floatType;
    Objects.requireNonNull(doubleType, "double_type");
    this.doubleType = doubleType;
    Objects.requireNonNull(bytesType, "bytes_type");
    this.bytesType = bytesType;
    Objects.requireNonNull(anyType, "any_type");
    this.anyType = anyType;
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

  @Override
  public int hashCode() {
    int result = 1;
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
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Entry)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Entry o = (Entry) other;

    if (!this.booleanType.equals(o.booleanType)) {
      return false;
    }

    if (!this.stringType.equals(o.stringType)) {
      return false;
    }

    if (!this.datetimeType.equals(o.datetimeType)) {
      return false;
    }

    if (!this.unsigned32.equals(o.unsigned32)) {
      return false;
    }

    if (!this.unsigned64.equals(o.unsigned64)) {
      return false;
    }

    if (!this.signed32.equals(o.signed32)) {
      return false;
    }

    if (!this.signed64.equals(o.signed64)) {
      return false;
    }

    if (!this.floatType.equals(o.floatType)) {
      return false;
    }

    if (!this.doubleType.equals(o.doubleType)) {
      return false;
    }

    if (!this.bytesType.equals(o.bytesType)) {
      return false;
    }

    if (!this.anyType.equals(o.anyType)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
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
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Boolean> booleanType = Optional.empty();
    private Optional<String> stringType = Optional.empty();
    private Optional<Instant> datetimeType = Optional.empty();
    private Optional<Integer> unsigned32 = Optional.empty();
    private Optional<Long> unsigned64 = Optional.empty();
    private Optional<Integer> signed32 = Optional.empty();
    private Optional<Long> signed64 = Optional.empty();
    private Optional<Float> floatType = Optional.empty();
    private Optional<Double> doubleType = Optional.empty();
    private Optional<ByteBuffer> bytesType = Optional.empty();
    private Optional<Object> anyType = Optional.empty();

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

    public Entry build() {
      final Optional<Boolean> booleanType = this.booleanType;
      final Optional<String> stringType = this.stringType;
      final Optional<Instant> datetimeType = this.datetimeType;
      final Optional<Integer> unsigned32 = this.unsigned32;
      final Optional<Long> unsigned64 = this.unsigned64;
      final Optional<Integer> signed32 = this.signed32;
      final Optional<Long> signed64 = this.signed64;
      final Optional<Float> floatType = this.floatType;
      final Optional<Double> doubleType = this.doubleType;
      final Optional<ByteBuffer> bytesType = this.bytesType;
      final Optional<Object> anyType = this.anyType;

      return new Entry(booleanType, stringType, datetimeType, unsigned32, unsigned64, signed32, signed64, floatType, doubleType, bytesType, anyType);
    }
  }
}
