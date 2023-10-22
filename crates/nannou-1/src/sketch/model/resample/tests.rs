use std::collections::VecDeque;

use super::*;
use crate::{sketch::model::figure::random_vertices, util::zip2};
use itertools::Itertools;

#[test]
fn all_resampled_points_are_at_same_distance() {
  const N0: usize = 128;
  const N1: usize = 1234;
  
  // The intended use of these functions is that the resampled vector size is greater than the vector size of the original data.
  // So N0 is expected to be less than N1.

  let data = random_vertices(N0, -500_f32, -500_f32);
  let mut data_shifted: VecDeque<Point2> = data.clone().into();
  let front = data_shifted.pop_front().unwrap();
  data_shifted.push_back(front);
  let polygon = data.iter().circular_tuple_windows();
  let pg_dists: Vec<f32> = polygon.map(|(&prev, &next)| { prev.distance(next) }).collect();

  let _rs_data = resample_polygon(&data, N1);
  let mut resampled = _rs_data.iter().circular_tuple_windows();

  let mut rs_dists: Vec<f32> = Vec::with_capacity(N1);
  let mut rem_dist = 0_f32;
  'outer: for (pg_dist_ref, &pg) in zip2!(pg_dists, data_shifted) {
    let mut pg_dist = pg_dist_ref.clone();
    pg_dist -= rem_dist;
    
    'inner: loop {
      let option = resampled.next();
      if option.is_none() { break 'outer; }

      let (&prev_rs, &next_rs) = option.unwrap();
      let rs_dist = prev_rs.distance(next_rs);
      if pg_dist < rs_dist {
        rem_dist = rs_dist - pg_dist;
        rs_dists.push(prev_rs.distance(pg) + rem_dist);
        break 'inner;
      } else {
        rs_dists.push(rs_dist);
      }
    }
  }

  let tot_dist: f32 = Iterator::sum(rs_dists.iter());
  let avg_dist: f32 = tot_dist / (rs_dists.len() as f32);
  const MARGIN: f32 = 0.0001;
  println!("avg_dist:\t{avg_dist}\nrs_dists:\t{:#?}", rs_dists);
  assert!(rs_dists.iter().all(|x| { (x - avg_dist).abs() <= MARGIN }));
}