use nannou::prelude::Point2;

mod figure;
use figure::random_vertices;

pub struct Model {
	pub vertices: Vec<Point2>,
}

pub trait Constants {
	const WEIGHT: f32 = 3.0;
	const N_POINTS: usize = 8;
}

impl Default for Model {
	fn default() -> Model {
		Model {
			vertices: Vec::new(),
		}
	}
}

impl Model {
  pub fn new_random_polygon(&mut self, left: f32, bottom: f32) {
    self.vertices = random_vertices(Model::N_POINTS, left, bottom);
  }
}