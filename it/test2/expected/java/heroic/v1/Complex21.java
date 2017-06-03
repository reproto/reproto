package heroic.v1;

import java.util.Objects;

public enum Complex21 {
  FIRST(new Point(123L, 42.1D)),
  SECOND(new Point(123L, 1234.12D));

  private final Point point;

  private Complex21(final Point point) {
    Objects.requireNonNull(point, "point");
    this.point = point;
  }

  public Point getPoint() {
    return this.point;
  }
}
