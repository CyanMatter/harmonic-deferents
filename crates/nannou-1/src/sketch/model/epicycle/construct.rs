use rustfft::num_complex::Complex;
use super::epicycle::*;

impl Null for Epicycle {}

fn new_epicycle(cfd: &Complex<f32>, fq: i64) -> Epicycle {
  let r: f32 = (cfd.re * cfd.re + cfd.im * cfd.im).sqrt();
  if r == 0_f32 { return Epicycle::NULL; }

  let ph: f32 = cfd.im.atan2(cfd.re);
  let v: Position = position_at_root(r, fq, ph);
  Epicycle {
    radius: r,
    frequency: fq,
    phase: ph,
    vector: v
  }
}

/* Takes complex numbers in the frequency domain (here named CFDs) that were produced by a Fourier transform,
 * and produces the epicycles that describe the sequence that was used in that Fourier transform.
 */
//TODO: this is parallelizable and (nannou::)wgpu provide improvement.
pub fn epicycles_from_cfds(data: &Vec<Complex<f32>>) -> Vec<Epicycle> {
  assert!(data.len() > 0);
  
  // Initialize.
  let nyquist = (data.len() / 2) as i64;
  let mut epicycles = Vec::<Epicycle>::with_capacity(data.len());
  let mut it = data.iter();

  // Construct the first epicycle; it always has frequency = 0.
  let e = new_epicycle(it.next().unwrap(), 0);
  epicycles.push(e);

  // Construct most other epicycles.
  for fq in 1..nyquist {
    epicycles.push(new_epicycle(it.next().unwrap(), fq));
    epicycles.push(new_epicycle(it.next().unwrap(), -fq));
  }

  // Construct the last epicycle(s).
  epicycles.push(new_epicycle(it.next().unwrap(), nyquist));
  if data.len() % 2 != 0 {
    epicycles.push(new_epicycle(it.next().unwrap(), -nyquist));
  }

  // Filter out all epicycles with 0 radius.
  return epicycles.into_iter()
    .filter(|e| e.radius > 0_f32)
    .collect();
}