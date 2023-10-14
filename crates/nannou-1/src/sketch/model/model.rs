use std::ops::Deref;
use std::time::Duration;
use itertools::Itertools;
use nannou::prelude::{ Point2, PI };
use rustfft::num_complex::Complex32;
use super::{figure::random_vertices, ToMultiComplex32};
use super::epicycle::{ Epicycle, epicycles_from_cfds, sort_by_radius };
use crate::fourier::fft;

#[derive(Default)]
pub struct Model {
	pub vertices: Vec<Point2>,
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
    self.vertices = random_vertices(Model::N_POINTS, left, bottom);
		// Conversion from similar structs Point2 -> Complex32 needed.
		let mut cfds: Vec<Complex32> = self.vertices.as_complex32_vec();
		// Create epicycles that describe the sequence of vertices.
		fft(&mut cfds);
		self.epicycles = epicycles_from_cfds(&cfds);
		sort_by_radius(&mut self.epicycles);
  }

	fn polygon_distance(&self) -> f32 {
		assert!(!self.vertices.is_empty());

		let mut sum = 0_f32;
		let iter = self.vertices.iter().circular_tuple_windows();
		for (prev, next) in iter {
			sum += prev.distance(*next);
		}
		sum
	}

	pub fn interpolate(&mut self, amount: u64) {
		let dis = self.polygon_distance();
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