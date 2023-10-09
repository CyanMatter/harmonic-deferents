// src/sketch/model
mod figure;
mod epicycle;
#[cfg(test)]
mod tests;

use nannou::{prelude::{ Point2, pt2 }, draw::primitive::ellipse::Ellipse };
use rustfft::num_complex::Complex32;
use figure::random_vertices;
use self::epicycle::{ Epicycle, epicycles_from_cfds, compute_all_renders };
use crate::fourier::fft;

pub struct Model {
	pub t_elapsed: f32,
	pub vertices: Vec<Point2>,
	pub epicycles: Vec<Epicycle>,
	pub ellipses: Vec<Ellipse>,
	pub epicycle_path: Vec<Point2>,
}

pub trait Constants {
	const WEIGHT: f32 = 3.0;
	const N_POINTS: usize = 8;
}

impl Default for Model {
	fn default() -> Model {
		Model {
			vertices: Vec::new(),
			epicycles: Vec::new(),
			ellipses: Vec::new(),
			epicycle_path: Vec::new(),

			t_elapsed: 0_f32,
		}
	}
}

trait ToComplex32 {
	fn as_complex32(&self) -> Complex32;
}
trait ToMultiComplex32 {
	fn as_complex32_vec(&self) -> Vec<Complex32>;
}

impl ToComplex32 for Point2 {
	fn as_complex32(&self) -> Complex32 {
		Complex32 { re: self.x, im: self.y }
	}
}

impl ToMultiComplex32 for Vec<Point2> {
	fn as_complex32_vec(&self) -> Vec<Complex32> {
		self.iter()
			.map(|v2| v2.as_complex32())
			.collect()
	}
}

impl Model {
  pub fn new_random_polygon(&mut self, left: f32, bottom: f32) {
		//! Debug
    self.vertices = random_vertices(Model::N_POINTS, left, bottom);
		// Conversion from similar structs Point2 -> Complex32 needed.
		let mut cfds: Vec<Complex32> = self.vertices.as_complex32_vec();
		// Create epicycles that describe the sequence of vertices.
		fft(&mut cfds);
		self.epicycles = epicycles_from_cfds(&cfds);
		compute_all_renders(self);
  }
}