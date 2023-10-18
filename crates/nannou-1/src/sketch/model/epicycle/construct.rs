use rustfft::num_complex::Complex;
use super::epicycle::*;

/* Takes complex numbers in the frequency domain (here named CFDs) that were produced by a Fourier transform,
 * and produces the epicycles that describe the sequence that was used in that Fourier transform.
 */
//TODO: this is parallelizable and (nannou::)wgpu provide improvement.
pub fn epicycles_from_cfds(data: &Vec<Complex<f32>>) -> Vec<Epicycle> {
  assert!(!data.is_empty());
  
  // Initialize.
  let nyquist = (data.len() / 2) as i64;
  let mut epicycles = Vec::<Epicycle>::with_capacity(data.len());
  let mut it = data.iter();

  // Construct the first epicycle; it always has frequency = 0.
  if let Some(e) = from_data(it.next().unwrap(), 0) {
    epicycles.push(e);
  }

  // Construct most other epicycles.
  for fq in 1..nyquist {
    if let Some(e) = from_data( it.next().unwrap(), fq) {
      epicycles.push(e);
    }
    
    if let Some(e) = from_data(it.next_back().unwrap(), -fq) {
      epicycles.push(e);
    }
  }

  // Construct the last epicycle(s).
  if let Some(e) = from_data(it.next().unwrap(), nyquist) {
    epicycles.push(e);
  }
  if data.len() % 2 != 0 {
    if let Some(e) = from_data(it.next().unwrap(), -nyquist) {
      epicycles.push(e);
    }
  }
  epicycles
}