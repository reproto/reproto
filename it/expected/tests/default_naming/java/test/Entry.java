package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;
import lower_camel.Value;

public class Entry {
  @JsonProperty("lower_camel")
  private final Optional<Value> lowerCamel;
  @JsonProperty("lower_snake")
  private final Optional<lower_snake.Value> lowerSnake;
  @JsonProperty("upper_camel")
  private final Optional<upper_camel.Value> upperCamel;
  @JsonProperty("upper_snake")
  private final Optional<upper_snake.Value> upperSnake;

  @JsonCreator
  public Entry(
    @JsonProperty("lower_camel") final Optional<Value> lowerCamel,
    @JsonProperty("lower_snake") final Optional<lower_snake.Value> lowerSnake,
    @JsonProperty("upper_camel") final Optional<upper_camel.Value> upperCamel,
    @JsonProperty("upper_snake") final Optional<upper_snake.Value> upperSnake
  ) {
    Objects.requireNonNull(lowerCamel, "lower_camel");
    this.lowerCamel = lowerCamel;
    Objects.requireNonNull(lowerSnake, "lower_snake");
    this.lowerSnake = lowerSnake;
    Objects.requireNonNull(upperCamel, "upper_camel");
    this.upperCamel = upperCamel;
    Objects.requireNonNull(upperSnake, "upper_snake");
    this.upperSnake = upperSnake;
  }

  @JsonProperty("lower_camel")
  public Optional<Value> getLowerCamel() {
    return this.lowerCamel;
  }

  @JsonProperty("lower_snake")
  public Optional<lower_snake.Value> getLowerSnake() {
    return this.lowerSnake;
  }

  @JsonProperty("upper_camel")
  public Optional<upper_camel.Value> getUpperCamel() {
    return this.upperCamel;
  }

  @JsonProperty("upper_snake")
  public Optional<upper_snake.Value> getUpperSnake() {
    return this.upperSnake;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.lowerCamel.hashCode();
    result = result * 31 + this.lowerSnake.hashCode();
    result = result * 31 + this.upperCamel.hashCode();
    result = result * 31 + this.upperSnake.hashCode();
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

    if (!this.lowerCamel.equals(o.lowerCamel)) {
      return false;
    }

    if (!this.lowerSnake.equals(o.lowerSnake)) {
      return false;
    }

    if (!this.upperCamel.equals(o.upperCamel)) {
      return false;
    }

    if (!this.upperSnake.equals(o.upperSnake)) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Entry");
    b.append("(");
    b.append("lower_camel=");
    b.append(this.lowerCamel.toString());
    b.append(", ");
    b.append("lower_snake=");
    b.append(this.lowerSnake.toString());
    b.append(", ");
    b.append("upper_camel=");
    b.append(this.upperCamel.toString());
    b.append(", ");
    b.append("upper_snake=");
    b.append(this.upperSnake.toString());
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<Value> lowerCamel = Optional.empty();
    private Optional<lower_snake.Value> lowerSnake = Optional.empty();
    private Optional<upper_camel.Value> upperCamel = Optional.empty();
    private Optional<upper_snake.Value> upperSnake = Optional.empty();

    public Builder lowerCamel(final Value lowerCamel) {
      this.lowerCamel = Optional.of(lowerCamel);
      return this;
    }

    public Builder lowerSnake(final lower_snake.Value lowerSnake) {
      this.lowerSnake = Optional.of(lowerSnake);
      return this;
    }

    public Builder upperCamel(final upper_camel.Value upperCamel) {
      this.upperCamel = Optional.of(upperCamel);
      return this;
    }

    public Builder upperSnake(final upper_snake.Value upperSnake) {
      this.upperSnake = Optional.of(upperSnake);
      return this;
    }

    public Entry build() {
      final Optional<Value> lowerCamel = this.lowerCamel;
      final Optional<lower_snake.Value> lowerSnake = this.lowerSnake;
      final Optional<upper_camel.Value> upperCamel = this.upperCamel;
      final Optional<upper_snake.Value> upperSnake = this.upperSnake;

      return new Entry(lowerCamel, lowerSnake, upperCamel, upperSnake);
    }
  }
}
