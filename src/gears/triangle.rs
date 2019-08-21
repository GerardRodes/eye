use super::point::Point;

#[derive(Debug)]
pub struct Triangle {
  pub vertices: [Point; 3],
}

impl Triangle {
  pub fn new (a: Point, b: Point, c: Point) -> Triangle {
    Triangle{
      vertices: [a, b, c]
    }
  }
}
