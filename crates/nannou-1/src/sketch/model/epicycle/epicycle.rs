use nannou::prelude::{ Point2, pt2 };
use rustfft::num_complex::Complex;

pub struct Epicycle {
  pub radius: f32,
  pub frequency: i64,
  pub phase: f32,
  pub vector: Point2,
}

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

  pub fn equals(&self, other: &Epicycle) -> bool {
    self.radius == other.radius
      && self.frequency == other.frequency
      && self.phase == other.phase
      && self.vector == other.vector
  }

  // why is this implemented in the struct instead of just in the module?
  pub fn from_data(cfd: &Complex<f32>, fq: i64) -> Epicycle {
    let r: f32 = (cfd.re * cfd.re + cfd.im * cfd.im).sqrt();
    if r == 0_f32 { return Epicycle::NULL; }

    let ph: f32 = cfd.im.atan2(cfd.re);
    let v = vector(0_f32, r, fq, ph);
    Epicycle {
      radius: r,
      frequency: fq,
      phase: ph,
      vector: v
    }
  }
}

// A special epicycle that has a radius of 0.
pub trait Null {
  const NULL: Epicycle = Epicycle {
    radius: 0_f32,
    frequency: 0,
    phase: 0_f32,
    vector: Point2::ZERO,
  };
}