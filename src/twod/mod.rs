use nalgebra::Point2;

trait PointUtil {
  fn manhattan(&self, other: &Self) -> i32;
}

impl PointUtil for Point2<i32> {
  fn manhattan(&self, other: &Self) -> i32 {
    return (self.x - other.x).abs() + (self.y - other.y).abs();
  }
}
