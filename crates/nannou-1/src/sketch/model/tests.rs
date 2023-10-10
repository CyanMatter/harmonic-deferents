use super::*;
use self::epicycle::Epicycle;
use crate::{fourier::fft, util::zip2};
use nannou::prelude::{ Point2, pt2 };
use rustfft::num_complex::Complex32;

#[test]
fn implement_constants() {
  struct A {}
  impl Constants for A {}
  assert_eq!(A::N_POINTS, Model::N_POINTS);
  assert_eq!(A::WEIGHT, Model::WEIGHT);
}

#[test]
fn compute_fft_from_polyline() {
  let expect: Vec<Complex32> = vec![
    Complex32 { re: 16_f32, im: 6_f32 },
    Complex32 { re: 2_f32, im: -2_f32 }
  ];

  let pl: Vec<Point2> = vec![
    pt2(9_f32, 2_f32),
    pt2(7_f32, 4_f32)
  ];
  let mut result: Vec<Complex32> = pl.as_complex32_vec();
  fft(&mut result);

  assert_eq!(expect, result);
}

#[test]
fn compute_epicycles_from_polyline() {
  let expect: Vec<Epicycle> = vec![
    Epicycle {
      radius: 17.09_f32,
      frequency: 0,
      phase: 0.3588_f32,
      vector: pt2(16_f32, 6_f32)
    },
    Epicycle {
      radius: 2.83_f32,
      frequency: 1,
      phase: -std::f32::consts::FRAC_PI_4,
      vector: pt2(2_f32, -2_f32)
    },
  ];
  let pl: Vec<Point2> = vec![
    pt2(9_f32, 2_f32),
    pt2(7_f32, 4_f32)
  ];

  let mut cfds: Vec<Complex32> = pl.as_complex32_vec();
  fft(&mut cfds);
  let result: Vec<Epicycle> = epicycles_from_cfds(&cfds);

  assert!(
    zip2!(result, expect)
    .all(|(r, e)| r.approx_eq(e, 0.01, 0.0001, pt2(0_f32, 0_f32)))
  );
}