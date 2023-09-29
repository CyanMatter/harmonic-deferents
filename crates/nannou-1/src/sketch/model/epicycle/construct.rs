use rustfft::num_complex::Complex;
use super::epicycle::*;

impl Null for Epicycle {}

fn compute(pt: &Complex<f32>, fq: i64) -> Epicycle {
  let re: f32 = pt.re;
  let im: f32 = pt.im;

  let r: f32 = (re * re + im * im).sqrt();
  if r == 0_f32 { return Epicycle::NULL; }

  let ph: f32 = im.atan2(re);
  let v: Position = position_at_root(r, fq, ph);
  Epicycle {
    radius: r,
    frequency: fq,
    phase: ph,
    vector: v
  }
}

pub fn construct(data: &Vec<Complex<f32>>) -> Vec<Epicycle> {
  // Initialize
  let nyquist: usize = data.len() / 2;
  let epicycles = &Vec::<Epicycle>::with_capacity(data.len());
  let mut it = data.iter();

  // Compute the first epicycle; it always has frequency = 0.
  let e: Epicycle = compute(
    it.next().expect("There should be at least one item in `data.`"),
    0);

  //continue
  return Vec::<Epicycle>::new();
}