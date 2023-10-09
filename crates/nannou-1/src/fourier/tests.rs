use super::*;
use crate::util::zip_2_vecs;
use nannou::rand::{distributions::Uniform, Rng};
use nannou::rand::thread_rng;

#[test]
fn ones() {
  let result = vec![
    Complex{ re: 4.0f32, im: 4.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 },
    Complex{ re: 0.0f32, im: 0.0f32 }
  ];
  let mut buffer = vec![Complex{ re: 1.0f32, im: 1.0f32 }; 4];
  fft(&mut buffer);
  assert_eq!(result, buffer);
}

#[test]
fn randomized() {
  // This test tests whether any errors pop up, so there is no checking against a result in this test.
  let dist = Uniform::from(f32::MIN..f32::MAX);
  let random_numbers = thread_rng().sample_iter(&dist);
  let real_vec: Vec<Complex32> = random_numbers.take(64).collect();
  let imag_vec: Vec<Complex32> = random_numbers.take(64).collect();
  let mut complex_numbers = zip_2_vecs!(real_vec, imag_vec)
    .map(|&real, &imag| Complex { re: real, im: imag })
    .collect();

  let randomized_test = || -> Result<(), MyError> {
    fft(&mut complex_numbers);
    Ok(())
  };
  
  if let Err(_err) = randomized_test() {
    panic!("While attempting to apply an FFT to the following series, an error occurred: {}.\n{:#?}", _err, complex_numbers);
  }
}