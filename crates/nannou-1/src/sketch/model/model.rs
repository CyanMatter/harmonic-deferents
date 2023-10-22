use std::time::Duration;
use nannou::prelude::{ Point2, PI };
use rustfft::num_complex::Complex32;
use super::resample::*;
use super::{figure::random_vertices, figure::simple_square, ToMultiComplex32};
use super::epicycle::{ Epicycle, epicycles_from_cfds, sort_by_radius };
use crate::fourier::fft;

#[derive(Default)]
pub struct Model {
	pub sketch_vertices: Vec<Point2>,
	pub resampled_vertices: Vec<Point2>,
	pub epicycles: Vec<Epicycle>,
	pub epicycle_path: Vec<Point2>,
  pub t_elapsed: u128,
  pub t_fq: f32,
	pub t_period: u128,
	pub t_frac_of_period_elapsed: f32,
  pub is_repeat_period: bool,
}

pub trait Constants {
	const WEIGHT: f32 = 3.0;
	const N_POINTS: usize = 8;
}

impl Model {
	pub fn new_random_polygon(&mut self, left: f32, bottom: f32) {
		//! Debug
		self.t_elapsed = 0;
    self.sketch_vertices = random_vertices(Model::N_POINTS, left, bottom);
		self.resampled_vertices = resample_polygon(&self.sketch_vertices, 50);
		// Conversion from similar structs Point2 -> Complex32 needed.
		let mut cfds: Vec<Complex32> = self.resampled_vertices.as_complex32_vec();
		// Create epicycles that describe the sequence of vertices.
		fft(&mut cfds);
		self.epicycles = epicycles_from_cfds(&cfds);
		sort_by_radius(&mut self.epicycles);
  }

	pub fn load_simple_square(&mut self) {
		self.t_elapsed = 0;
		self.sketch_vertices = simple_square();
		self.resampled_vertices = self.sketch_vertices.clone();
		// Conversion from similar structs Point2 -> Complex32 needed.
		let mut cfds: Vec<Complex32> = self.sketch_vertices.as_complex32_vec();
		// Create epicycles that describe the sequence of vertices.
		fft(&mut cfds);
		crate::console::log(format!("sketch_vertices:\n{:#?}", self.sketch_vertices));
		crate::console::log(format!("cfds:\n{:#?}", cfds));
		self.epicycles = epicycles_from_cfds(&cfds);
		sort_by_radius(&mut self.epicycles);
	}
	
	pub fn set_period_duration(&mut self, seconds: f32) {
		let dur = Duration::from_secs_f32(seconds);
    self.t_period = dur.as_millis();
    self.t_fq = 2_f32 * PI / (self.t_period as f32);
	}

	pub fn advance_time(&mut self, t_since_last_update: Duration) {
    if self.is_repeat_period { self.is_repeat_period = false; }
		self.t_elapsed += t_since_last_update.as_millis();
    if self.t_elapsed >= self.t_period {
      self.t_elapsed %= self.t_period;
      self.is_repeat_period = true;
    }
    self.t_frac_of_period_elapsed = self.t_fq * (self.t_elapsed as f32);
	}
}