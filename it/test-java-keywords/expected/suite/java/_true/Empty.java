package _true;

import com.fasterxml.jackson.annotation.JsonCreator;

public class Empty {
  @JsonCreator
  public Empty() {
  }

  @Override
  public int hashCode() {
    int result = 1;
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Empty)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Empty o = (Empty) other;

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Empty");
    b.append("(");
    b.append(")");

    return b.toString();
  }

  public static class Builder {
    public Empty build() {

      return new Empty();
    }
  }
}
