use super::*;
use self::epicycle::Epicycle;
use crate::{fourier::fft, util::zip_2_vecs};
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
  let exp: Vec<Complex32> = vec![
    Complex32 { re: 11_f32, im: 7_f32 },
    Complex32 { re: 11_f32, im: 3_f32 }
  ];

  let pl: Vec<Point2> = vec![
    pt2(9_f32, 2_f32),
    pt2(7_f32, 4_f32)
  ];
  let mut res: Vec<Complex32> = pl.as_complex32_vec();
  fft(&mut res);

  assert_eq!(exp, res);
}

#[test]
fn compute_epicycles_from_polyline() {
  let exp: Vec<Epicycle> = vec![
    Epicycle {
      radius: 13.0_f32,
      frequency: 0,
      phase: 0.567_f32,
      vector: pt2(11_f32, 7_f32)
    },
    Epicycle {
      radius: 11.4_f32,
      frequency: 1,
      phase: 0.266_f32,
      vector: pt2(11_f32, 3_f32)
    },
  ];
  let pl: Vec<Point2> = vec![
    pt2(9_f32, 2_f32),
    pt2(7_f32, 4_f32)
  ];

  let mut cfds: Vec<Complex32> = pl.as_complex32_vec();
  fft(&mut cfds);
  let res: Vec<Epicycle> = epicycles_from_cfds(&cfds);

  assert!(
    zip_2_vecs!(res, exp)
    .all(|(res, exp)| res.equals(exp))
  );
}