#![allow(dead_code)]
mod gears;
extern crate image;
use image::{ImageBuffer, RgbImage, Rgb};

const MDL_NAME: &str = "piramid";
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
  let input_mdl = format!("./models/{}.mdl", MDL_NAME);
  let piramid = gears::model::Model::from_mdl(&*input_mdl);

  println!("{:?}", piramid);

  let mut imgbuf: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

  let get_point_coordinates = |p: &gears::point::Point| -> [u32; 2] {
    [
      (p.x as u32) + (WIDTH / 2),
      (p.y as u32) + (HEIGHT / 2),
    ]
  };

  for t in piramid.triangles {
    for p in t.vertices.iter() {
      let coordinates = get_point_coordinates(p);

      if coordinates[0] >= WIDTH || coordinates[1] >= HEIGHT {
        continue
      }

      let pixel = imgbuf.get_pixel_mut(coordinates[0], coordinates[1]);
      *pixel = Rgb([255, 0, 0]);
    }
  }

  let output_png = format!("./out/{}.png", MDL_NAME);
  imgbuf.save(output_png).unwrap();
}
