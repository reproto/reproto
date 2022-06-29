package test;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Optional;
import lower_camel.Value;

public class Entry {
    @JsonProperty("lower_camel")
    final Optional<Value> lowerCamel;
    @JsonProperty("lower_snake")
    final Optional<lower_snake.Value> lowerSnake;
    @JsonProperty("upper_camel")
    final Optional<upper_camel.Value> upperCamel;
    @JsonProperty("upper_snake")
    final Optional<upper_snake.Value> upperSnake;

    @JsonCreator
    public Entry(
        @JsonProperty("lower_camel") Optional<Value> lowerCamel,
        @JsonProperty("lower_snake") Optional<lower_snake.Value> lowerSnake,
        @JsonProperty("upper_camel") Optional<upper_camel.Value> upperCamel,
        @JsonProperty("upper_snake") Optional<upper_snake.Value> upperSnake
    ) {
        this.lowerCamel = lowerCamel;
        this.lowerSnake = lowerSnake;
        this.upperCamel = upperCamel;
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
    public String toString() {
        final StringBuilder b = new StringBuilder();

        b.append("Entry(");
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

    @Override
    public int hashCode() {
        int result = 1;
        final StringBuilder b = new StringBuilder();
        result = result * 31 + this.lowerCamel.hashCode();
        result = result * 31 + this.lowerSnake.hashCode();
        result = result * 31 + this.upperCamel.hashCode();
        result = result * 31 + this.upperSnake.hashCode();
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

        if (!this.lowerCamel.equals(o_.lowerCamel)) {
            return false;
        }

        if (!this.lowerSnake.equals(o_.lowerSnake)) {
            return false;
        }

        if (!this.upperCamel.equals(o_.upperCamel)) {
            return false;
        }

        if (!this.upperSnake.equals(o_.upperSnake)) {
            return false;
        }

        return true;
    }

    public static class Builder {
        private Optional<Value> lowerCamel;
        private Optional<lower_snake.Value> lowerSnake;
        private Optional<upper_camel.Value> upperCamel;
        private Optional<upper_snake.Value> upperSnake;

        private Builder() {
            this.lowerCamel = Optional.empty();
            this.lowerSnake = Optional.empty();
            this.upperCamel = Optional.empty();
            this.upperSnake = Optional.empty();
        }

        public Entry build() {

            return new Entry(
                this.lowerCamel,
                this.lowerSnake,
                this.upperCamel,
                this.upperSnake
            );
        }

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
    }

    /**
     * Construct a new builder.
     */
    public static Builder builder() {
        return new Builder();
    }
}
