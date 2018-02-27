package test;


public interface Interface {
  public void interfaceMethod();

  public static class SubType implements Interface {
    public SubType() {
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

      if (!(other instanceof SubType)) {
        return false;
      }

      @SuppressWarnings("unchecked")
      final SubType o = (SubType) other;

      return true;
    }

    @Override
    public String toString() {
      final StringBuilder b = new StringBuilder();

      b.append("SubType");
      b.append("(");
      b.append(")");

      return b.toString();
    }

    @Override
    public void interfaceMethod() {
    }

    public static class Builder {
      public SubType build() {

        return new SubType();
      }
    }
  }
}
