#[derive(Debug)]
pub struct Point {
  x: i64,
  y: i64,
  z: i64,
}

impl Point {
  pub fn new(x: i64, y: i64, z: i64) -> Point {
    Point{x, y, z}
  }
}
