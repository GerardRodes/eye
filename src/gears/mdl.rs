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
      let mut line = line.unwrap();
      line = line.trim().to_string();

      if line.is_empty() {
        continue
      }

      let first_char = line.chars().next().unwrap();

      match first_char {
        'p' => {
          let mut numbers: [i64; 3] = [0, 0, 0];

          let mut numbers_found: u8 = 0;
          for (index, item) in line.split_whitespace().skip(1).enumerate() {
            let number = item.parse::<i64>().unwrap();
            numbers[index] = number;
            numbers_found += 1;
          }

          if numbers_found != 3 {
            panic!("\nError at {}:{}\nLine must contain 3 numbers but obtained: {}", file_path, index + 1, line)
          }

          points.push(
            Point::new(
              numbers[0],
              numbers[1],
              numbers[2],
            )
          )
        },
        't' => {
          let mut numbers: [usize; 3] = [0, 0, 0];

          let mut numbers_found: u8 = 0;
          for (index, item) in line.split_whitespace().skip(1).enumerate() {
            let number = item.parse::<usize>().unwrap();
            numbers[index] = number;
            numbers_found += 1;
          }

          if numbers_found != 3 {
            panic!("\nError at {}:{}\nLine must contain 3 numbers but obtained: {}", file_path, index + 1, line)
          }

          triangles.push(
            Triangle::new(
              points[numbers[0]],
              points[numbers[1]],
              points[numbers[2]],
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