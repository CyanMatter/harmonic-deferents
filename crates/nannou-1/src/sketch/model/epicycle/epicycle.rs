use nannou::{ prelude::{ Point2, pt2 }, draw::primitive::{ellipse::Ellipse, polygon::SetPolygon}, lyon::geom::euclid::default, color::ConvertInto };
use rustfft::num_complex::Complex;

pub struct Epicycle {
  pub radius: f32,
  pub frequency: i64,
  pub phase: f32,
  pub vector: Point2,
  pub ellipse: Option<Ellipse>,
}

// A special epicycle that has a radius of 0.
trait Null {
  const NULL: Epicycle = Epicycle {
    radius: 0_f32,
    frequency: 0,
    phase: 0_f32,
    vector: Point2::ZERO,
    ellipse: None,
  };
}

impl Null for Epicycle {}

impl PartialEq for Epicycle {
  fn eq(&self, other: &Self) -> bool {
    self.radius == other.radius
      && self.frequency == other.frequency
      && self.phase == other.phase
      && self.vector == other.vector
  }
}

impl Eq for Epicycle {}

fn vector(time: f32, radius: f32, frequency: i64, phase: f32) -> Point2 {
  let c: f32 = (frequency as f32) * time + phase;
  let x: f32 = radius * c.cos();
  let y: f32 = radius * c.sin();
  pt2(x, y)
}

impl Epicycle {
  pub fn vector_at(&mut self, time: f32) -> Point2 {
    self.vector = vector(time, self.radius, self.frequency, self.phase);
    self.vector
  }

  pub fn approx_eq(&self, other: &Epicycle, radius_err: f32, phase_err: f32, vector_err: Point2) -> bool {
    ((self.radius - other.radius).abs() <= radius_err + std::f32::EPSILON)
      && ((self.phase - other.phase).abs() <= phase_err + std::f32::EPSILON)
      && (self.vector - other.vector).abs().cmple(vector_err + pt2(std::f32::EPSILON, std::f32::EPSILON)).all()
      && (self.frequency == other.frequency)
  }
}

pub fn from_data(cfd: &Complex<f32>, fq: i64) -> Epicycle {
  let temp: f32 = cfd.re * cfd.re + cfd.im * cfd.im;
  if temp == 0_f32 { return Epicycle::NULL; }
  let r: f32 = temp.sqrt();

  let ph: f32 = cfd.im.atan2(cfd.re);
  let v = vector(0_f32, r, fq, ph);
  let el = Ellipse::radius(Ellipse::default(), r);

  Epicycle {
    radius: r,
    frequency: fq,
    phase: ph,
    vector: v,
    ellipse: Some(el),
  }
}

// Sorts a vector of epicycles in-place in order of largest to smallest radius
pub fn sort_by_radius(vec: &mut [Epicycle]) {
  vec.sort_by(|a, b| b.radius.partial_cmp(&a.radius).unwrap());
}