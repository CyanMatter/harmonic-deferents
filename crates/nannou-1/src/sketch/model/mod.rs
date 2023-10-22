// src/sketch/model
pub mod model;
pub mod figure;
pub mod epicycle;
mod resample;
#[cfg(test)]
mod tests;

pub use model::*;
use nannou::prelude::Point2;
use rustfft::num_complex::Complex32;

pub trait ToComplex32 {
	fn as_complex32(&self) -> Complex32;
}
pub trait ToMultiComplex32 {
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