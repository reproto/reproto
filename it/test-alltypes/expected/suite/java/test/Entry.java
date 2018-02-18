package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.nio.ByteBuffer;
import java.time.Instant;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

public class Entry {
  private final Optional<Boolean> booleanType;
  private final Optional<String> stringType;
  private final Optional<Instant> datetimeType;
  private final Optional<Integer> unsigned32;
  private final Optional<Integer> unsigned64;
  private final Optional<Integer> signed32;
  private final Optional<Long> signed64;
  private final Optional<Float> floatType;
  private final Optional<Double> doubleType;
  private final Optional<ByteBuffer> bytesType;
  private final Optional<Object> anyType;
  private final Optional<List<Entry>> arrayType;
  private final Optional<Map<String, Entry>> mapType;

  @JsonCreator
  public Entry(
    @JsonProperty("boolean_type") final Optional<Boolean> booleanType,
    @JsonProperty("string_type") final Optional<String> stringType,
    @JsonProperty("datetime_type") final Optional<Instant> datetimeType,
    @JsonProperty("unsigned_32") final Optional<Integer> unsigned32,
    @JsonProperty("unsigned_64") final Optional<Integer> unsigned64,
    @JsonProperty("signed_32") final Optional<Integer> signed32,
    @JsonProperty("signed_64") final Optional<Long> signed64,
    @JsonProperty("float_type") final Optional<Float> floatType,
    @JsonProperty("double_type") final Optional<Double> doubleType,
    @JsonProperty("bytes_type") final Optional<ByteBuffer> bytesType,
    @JsonProperty("any_type") final Optional<Object> anyType,
    @JsonProperty("array_type") final Optional<List<Entry>> arrayType,
    @JsonProperty("map_type") final Optional<Map<String, Entry>> mapType
  ) {
    Objects.requireNonNull(booleanType, "booleanType");
    this.booleanType = booleanType;
    Objects.requireNonNull(stringType, "stringType");
    this.stringType = stringType;
    Objects.requireNonNull(datetimeType, "datetimeType");
    this.datetimeType = datetimeType;
    Objects.requireNonNull(unsigned32, "unsigned32");
    this.unsigned32 = unsigned32;
    Objects.requireNonNull(unsigned64, "unsigned64");
    this.unsigned64 = unsigned64;
    Objects.requireNonNull(signed32, "signed32");
    this.signed32 = signed32;
    Objects.requireNonNull(signed64, "signed64");
    this.signed64 = signed64;
    Objects.requireNonNull(floatType, "floatType");
    this.floatType = floatType;
    Objects.requireNonNull(doubleType, "doubleType");
    this.doubleType = doubleType;
    Objects.requireNonNull(bytesType, "bytesType");
    this.bytesType = bytesType;
    Objects.requireNonNull(anyType, "anyType");
    this.anyType = anyType;
    Objects.requireNonNull(arrayType, "arrayType");
    this.arrayType = arrayType;
    Objects.requireNonNull(mapType, "mapType");
    this.mapType = mapType;
  }

  public Optional<Boolean> getBooleanType() {
    return this.booleanType;
  }

  public Optional<String> getStringType() {
    return this.stringType;
  }

  public Optional<Instant> getDatetimeType() {
    return this.datetimeType;
  }

  public Optional<Integer> getUnsigned32() {
    return this.unsigned32;
  }

  public Optional<Integer> getUnsigned64() {
    return this.unsigned64;
  }

  public Optional<Integer> getSigned32() {
    return this.signed32;
  }

  public Optional<Long> getSigned64() {
    return this.signed64;
  }

  public Optional<Float> getFloatType() {
    return this.floatType;
  }

  public Optional<Double> getDoubleType() {
    return this.doubleType;
  }

  public Optional<ByteBuffer> getBytesType() {
    return this.bytesType;
  }

  public Optional<Object> getAnyType() {
    return this.anyType;
  }

  public Optional<List<Entry>> getArrayType() {
    return this.arrayType;
  }

  public Optional<Map<String, Entry>> getMapType() {
    return this.mapType;
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
    result = result * 31 + this.arrayType.hashCode();
    result = result * 31 + this.mapType.hashCode();
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

    if (!this.arrayType.equals(o.arrayType)) {
      return false;
    }

    if (!this.mapType.equals(o.mapType)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("booleanType=");
    b.append(this.booleanType.toString());
    b.append(", ");
    b.append("stringType=");
    b.append(this.stringType.toString());
    b.append(", ");
    b.append("datetimeType=");
    b.append(this.datetimeType.toString());
    b.append(", ");
    b.append("unsigned32=");
    b.append(this.unsigned32.toString());
    b.append(", ");
    b.append("unsigned64=");
    b.append(this.unsigned64.toString());
    b.append(", ");
    b.append("signed32=");
    b.append(this.signed32.toString());
    b.append(", ");
    b.append("signed64=");
    b.append(this.signed64.toString());
    b.append(", ");
    b.append("floatType=");
    b.append(this.floatType.toString());
    b.append(", ");
    b.append("doubleType=");
    b.append(this.doubleType.toString());
    b.append(", ");
    b.append("bytesType=");
    b.append(this.bytesType.toString());
    b.append(", ");
    b.append("anyType=");
    b.append(this.anyType.toString());
    b.append(", ");
    b.append("arrayType=");
    b.append(this.arrayType.toString());
    b.append(", ");
    b.append("mapType=");
    b.append(this.mapType.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Boolean> booleanType = Optional.empty();
    private Optional<String> stringType = Optional.empty();
    private Optional<Instant> datetimeType = Optional.empty();
    private Optional<Integer> unsigned32 = Optional.empty();
    private Optional<Integer> unsigned64 = Optional.empty();
    private Optional<Integer> signed32 = Optional.empty();
    private Optional<Long> signed64 = Optional.empty();
    private Optional<Float> floatType = Optional.empty();
    private Optional<Double> doubleType = Optional.empty();
    private Optional<ByteBuffer> bytesType = Optional.empty();
    private Optional<Object> anyType = Optional.empty();
    private Optional<List<Entry>> arrayType = Optional.empty();
    private Optional<Map<String, Entry>> mapType = Optional.empty();

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

    public Builder unsigned64(final int unsigned64) {
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

    public Builder mapType(final Map<String, Entry> mapType) {
      this.mapType = Optional.of(mapType);
      return this;
    }

    public Entry build() {
      final Optional<Boolean> booleanType = this.booleanType;
      final Optional<String> stringType = this.stringType;
      final Optional<Instant> datetimeType = this.datetimeType;
      final Optional<Integer> unsigned32 = this.unsigned32;
      final Optional<Integer> unsigned64 = this.unsigned64;
      final Optional<Integer> signed32 = this.signed32;
      final Optional<Long> signed64 = this.signed64;
      final Optional<Float> floatType = this.floatType;
      final Optional<Double> doubleType = this.doubleType;
      final Optional<ByteBuffer> bytesType = this.bytesType;
      final Optional<Object> anyType = this.anyType;
      final Optional<List<Entry>> arrayType = this.arrayType;
      final Optional<Map<String, Entry>> mapType = this.mapType;

      return new Entry(booleanType, stringType, datetimeType, unsigned32, unsigned64, signed32, signed64, floatType, doubleType, bytesType, anyType, arrayType, mapType);
    }
  }
}
