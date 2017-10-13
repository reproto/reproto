package common._1_0_0;

import java.util.Objects;
import java.util.Optional;

public class ErrorMessage {
  private final String message;
  private final int statusCode;

  public ErrorMessage(
    final String message,
    final int statusCode
  ) {
    Objects.requireNonNull(message, "message");
    this.message = message;
    this.statusCode = statusCode;
  }

  public String getMessage() {
    return this.message;
  }

  public int getStatusCode() {
    return this.statusCode;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + this.message.hashCode();
    result = result * 31 + this.statusCode;
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof ErrorMessage)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final ErrorMessage o = (ErrorMessage) other;

    if (!this.message.equals(o.message)) {
      return false;
    }

    if (this.statusCode != o.statusCode) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("ErrorMessage");
    b.append("(");
    b.append("message=");
    b.append(this.message.toString());
    b.append(", ");
    b.append("statusCode=");
    b.append(Integer.toString(this.statusCode));
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    private Optional<String> message = Optional.empty();
    private Optional<Integer> statusCode = Optional.empty();

    public Builder message(final String message) {
      this.message = Optional.of(message);
      return this;
    }

    public Builder statusCode(final int statusCode) {
      this.statusCode = Optional.of(statusCode);
      return this;
    }

    public ErrorMessage build() {
      final String message = this.message.orElseThrow(() -> new RuntimeException("message: is required"));
      final int statusCode = this.statusCode.orElseThrow(() -> new RuntimeException("statusCode: is required"));

      return new ErrorMessage(message, statusCode);
    }
  }
}
