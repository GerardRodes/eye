use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::triangle::Triangle;
use super::point::Point;

pub struct MDL {
  pub triangles: Vec<Triangle>,
  pub points: Vec<Point>,
}

impl MDL {
  pub fn open (file_path: &str) -> MDL {
    let location = Path::new(file_path);

    let file = File::open(&location).unwrap();
    let reader = BufReader::new(file);

    let mut points: Vec<Point> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
      let line = line.unwrap();
      let first_char = line.chars().next().unwrap();

      match first_char {
        'p' => {
          let mut numbers: Vec<i64> = Vec::new();

          for item in line.split_whitespace().skip(1) {
            let number = item.parse::<i64>().unwrap();
            numbers.push(number);
          }

          if numbers.len() != 3 {
            panic!("Error at {}:{}\nPoint must contain 3 numbers but obtained: {}", file_path, index + 1, line)
          }

          points.push(
            Point::new(
              numbers[0],
              numbers[1],
              numbers[2],
            )
          )
        },
        _ => continue
      }
    }

    MDL {
      triangles: triangles,
      points: points,
    }
  }
}