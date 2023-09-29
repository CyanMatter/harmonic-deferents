// src/sketch/model/figure
use crate::util::zip_2_vecs;
use nannou::{
	rand::{
		{distributions::Uniform, Rng},
		thread_rng
	},
	prelude::{Point2, pt2}
};

fn bound_rand_vec(n: usize, low: f32, high: f32) -> Vec<f32> {
	let bounds = Uniform::from(low..high);
	thread_rng()
		.sample_iter(&bounds)
		.take(n)
		.collect()		
}

pub fn random_vertices(n: usize, left: f32, bottom: f32) -> Vec<Point2> {
	let xs: Vec<f32> = bound_rand_vec(n, left, -left);
	let ys: Vec<f32> = bound_rand_vec(n, bottom, -bottom);
	zip_2_vecs!(xs, ys)
		.map(|(&x, &y)| pt2(x, y))
		.collect()
}