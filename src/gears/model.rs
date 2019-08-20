use super::triangle::Triangle;
use super::mdl::MDL;

#[derive(Debug)]
pub struct Model {
  pub triangles: Vec<Triangle>,
}

impl Model {
  pub fn new (triangles: Vec<Triangle>) -> Model {
    Model{ triangles }
  }

  pub fn from_mdl(file_path: &str) -> Model {
    let mdl = MDL::open(file_path);

    Model {
      triangles: mdl.triangles,
    }
  }
}
