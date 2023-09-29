use rustfft::{FftPlanner, num_complex::Complex};

pub fn fft(data: &mut Vec<Complex<f32>>) {
  let mut planner = FftPlanner::new();
  let fft = planner.plan_fft_forward(data.len());
  fft.process(data);
}

pub fn ifft(data: &mut Vec<Complex<f32>>) {
  let mut planner = FftPlanner::new();
  let fft = planner.plan_fft_inverse(data.len());
  fft.process(data);
}