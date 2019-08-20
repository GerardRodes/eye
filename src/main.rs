#![allow(dead_code)]

mod gears;

const FILE: &str = "./models/triangle.mdl";

// const WIDTH: u16 = 500;
// const HEIGHT: u16 = 500;

fn main() {
  let triangle_mdl = gears::model::Model::from_mdl(FILE);

  println!("{:?}", triangle_mdl);
  println!("done!");
}
