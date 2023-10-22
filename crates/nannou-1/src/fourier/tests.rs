use super::*;
use rustfft::num_complex::Complex;

#[test]
fn ones() {
  let result = vec![
    Complex{ re: 1.0f32, im: 1.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 }
  ];
  let mut buffer = vec![Complex{ re: 1.0f32, im: 1.0f32 }; 4];
  fft(&mut buffer);
  assert_eq!(result, buffer);
}
