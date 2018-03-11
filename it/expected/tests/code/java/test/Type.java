package test;

import com.fasterxml.jackson.annotation.JsonCreator;

public class Type {
  @JsonCreator
  public Type() {
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

    if (!(other instanceof Type)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Type o = (Type) other;

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Type");
    b.append("(");
    b.append(")");

    return b.toString();
  }

  public void typeMethod() {
  }

  public static class Builder {
    public Type build() {

      return new Type();
    }
  }
}
