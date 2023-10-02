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
  assert!(data.len() > 0);
  
  // Initialize.
  let nyquist = (data.len() / 2) as i64;
  let mut epicycles = Vec::<Epicycle>::with_capacity(data.len());
  let mut it = data.iter();

  // Construct the first epicycle; it always has frequency = 0.
  let e = compute(it.next().unwrap(), 0);
  epicycles.push(e);

  // Construct most other epicycles.
  for fq in 1..nyquist {
    epicycles.push(compute(it.next().unwrap(), fq));
    epicycles.push(compute(it.next().unwrap(), -fq));
  }

  // Construct the last epicycle(s).
  epicycles.push(compute(it.next().unwrap(), nyquist));
  if data.len() % 2 != 0 {
    epicycles.push(compute(it.next().unwrap(), -nyquist));
  }

  // Filter out all epicycles with 0 radius.
  return epicycles.into_iter()
    .filter(|e| e.radius > 0_f32)
    .collect();
}