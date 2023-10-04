// src/sketch/model
mod figure;
mod epicycle;
#[cfg(test)]
mod tests;

use nannou::prelude::{ Point2, pt2 };
use rustfft::num_complex::Complex32;
use figure::random_vertices;
use epicycle::{ Epicycle, epicycles_from_cfds };

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

fn vec2_to_complex(v2: Point2) -> Complex32 {
	Complex32 {
		re: v2.x,
		im: v2.y
	}
}

fn complex_to_vec2(c: Complex32) -> Point2 {
	pt2(c.re, c.im)
}

impl Model {
  pub fn new_random_polygon(&mut self, left: f32, bottom: f32) {
		//! Debug
    self.vertices = random_vertices(Model::N_POINTS, left, bottom);
		// Conversion from similar Point2 -> Complex32 needed.
		let cfds: Vec<Complex32> = self.vertices.iter()
			.map(|v2| vec2_to_complex(*v2))
			.collect();
		// Create epicycles that describe the sequence of vertices.
		let epicycles: Vec<Epicycle> = epicycles_from_cfds(&cfds);
  }
}