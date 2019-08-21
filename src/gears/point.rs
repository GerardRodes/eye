#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: i64,
  pub y: i64,
  pub z: i64,
}

impl Point {
  pub fn new(x: i64, y: i64, z: i64) -> Point {
    Point{x, y, z}
  }
}
