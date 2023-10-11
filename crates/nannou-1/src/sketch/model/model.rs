use std::time::Duration;
use nannou::{prelude::{ Point2, PI }, draw::primitive::ellipse::Ellipse };
use rustfft::num_complex::Complex32;
use super::{figure::random_vertices, ToMultiComplex32};
use super::epicycle::{ Epicycle, epicycles_from_cfds, compute_all_renders };
use crate::fourier::fft;

#[derive(Default)]
pub struct Model {
	pub vertices: Vec<Point2>,
	pub epicycles: Vec<Epicycle>,
	pub ellipses: Vec<Ellipse>,
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
	pub fn set_period_duration(&mut self, seconds: f32) {
		let dur = Duration::from_secs_f32(seconds);
    self.t_period = dur.as_millis();
    self.t_fq = 2_f32 * PI / (self.t_period as f32);
	}

  pub fn new_random_polygon(&mut self, left: f32, bottom: f32) {
		//! Debug
		self.t_elapsed = 0;
    self.vertices = random_vertices(Model::N_POINTS, left, bottom);
		// Conversion from similar structs Point2 -> Complex32 needed.
		let mut cfds: Vec<Complex32> = self.vertices.as_complex32_vec();
		// Create epicycles that describe the sequence of vertices.
		fft(&mut cfds);
		self.epicycles = epicycles_from_cfds(&cfds);
  }

	pub fn advance_time(&mut self, t_since_last_update: Duration) {
    crate::console::log(format!("t_since_last_update:\t{:?}\nt_elapsed:\t{:?}\nt_period:\t{:?}\nt_frac_of_period_elapsed:\t{:?}", t_since_last_update, self.t_elapsed, self.t_period, self.t_frac_of_period_elapsed));

    if self.is_repeat_period { self.is_repeat_period = false; }
		self.t_elapsed += t_since_last_update.as_millis();
    if self.t_elapsed >= self.t_period {
      self.t_elapsed %= self.t_period;
      self.is_repeat_period = true;
    }
    self.t_frac_of_period_elapsed = self.t_fq * (self.t_elapsed as f32);

    crate::console::log(format!("t_since_last_update:\t{:?}\nt_elapsed:\t{:?}\nt_period:\t{:?}\nt_frac_of_period_elapsed:\t{:?}", t_since_last_update, self.t_elapsed, self.t_period, self.t_frac_of_period_elapsed));
	}
}