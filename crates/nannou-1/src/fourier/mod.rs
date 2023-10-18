// src/fourier
#[cfg(test)]
mod tests;

use rustfft::{FftPlanner, num_complex::Complex};

fn scale(data: &mut Vec<Complex<f32>>) {
  let n = 1_f32 / (data.len() as f32);
  for c in data.iter_mut() {
    *c *= n;
  }
}

pub fn fft(data: &mut Vec<Complex<f32>>) {
  let mut planner = FftPlanner::new();
  let fft = planner.plan_fft_forward(data.len());
  fft.process(data);
  scale(data);
}

pub fn ifft(data: &mut Vec<Complex<f32>>) {
  let mut planner = FftPlanner::new();
  let fft = planner.plan_fft_inverse(data.len());
  fft.process(data);
  scale(data);
}