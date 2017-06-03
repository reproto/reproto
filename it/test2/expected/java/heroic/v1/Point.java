package heroic.v1;

public class Point {
  private final long timestamp;
  private final double value;

  public Point(final long timestamp, final double value) {
    this.timestamp = timestamp;
    this.value = value;
  }

  public long getTimestamp() {
    return this.timestamp;
  }

  public double getValue() {
    return this.value;
  }

  @Override
  public int hashCode() {
    int result = 1;
    result = result * 31 + Long.hashCode(this.timestamp);
    result = result * 31 + Double.hashCode(this.value);
    return result;
  }

  @Override
  public boolean equals(final Object other) {
    if (other == null) {
      return false;
    }

    if (!(other instanceof Point)) {
      return false;
    }

    @SuppressWarnings("unchecked")
    final Point o = (Point) other;

    if (this.timestamp != o.timestamp) {
      return false;
    }

    if (this.value != o.value) {
      return false;
    }

    return true;
  }

  @Override
  public String toString() {
    final StringBuilder b = new StringBuilder();

    b.append("Point");
    b.append("(");
    b.append("timestamp=");
    b.append(Long.toString(this.timestamp));
    b.append(", ");
    b.append("value=");
    b.append(Double.toString(this.value));
    b.append(")");

    return b.toString();
  }
}
