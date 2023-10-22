use itertools::Itertools;
use nannou::prelude::Point2;
#[cfg(test)]
mod tests;

fn distances_between_vertices(vertices: &Vec<Point2>) -> Vec<f32> {
	let mut distances: Vec<f32> = Vec::with_capacity(vertices.len());
	let iter = vertices.iter().circular_tuple_windows();
	for (prev, next) in iter {
		distances.push(prev.distance(*next));
	}
	distances
}

pub fn resample_polygon(vertices: &Vec<Point2>, num_pts: usize) -> Vec<Point2> {
	assert!(!vertices.is_empty());
	let mut resample = Vec::with_capacity(num_pts);
	resample.push(*vertices.first().unwrap());
	
	let vertex_pair_iter = vertices.iter().circular_tuple_windows();
	let distances = distances_between_vertices(vertices);
	let distance_iter = distances.iter();
	let tot_dist: f32 = distances.iter().sum();
	let arc_length: f32 = tot_dist / (num_pts as f32);
	
	let mut t = 0_f32;
	for ((prev, next), distance) in vertex_pair_iter.zip(distance_iter) {
		let d_t = *distance / arc_length;
		while ((t + d_t) >= (resample.len() as f32)) && (resample.len() < num_pts) {
			let s = (resample.len() as f32 - t) / d_t;
			let v = prev.lerp(*next, s);
			resample.push(v);
		}
		t += d_t
	}
	resample
}