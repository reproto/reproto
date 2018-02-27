package test;


public class Tuple {
  public Tuple() {
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

    if (!(other instanceof Tuple)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Tuple o = (Tuple) other;

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Tuple");
    b.append("(");
    b.append(")");

    return b.toString();
  }

  public void tupleMethod() {
  }
}
