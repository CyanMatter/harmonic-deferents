use super::*;

#[test]
fn implement_constants() {
  struct A {}
  impl Constants for A {}
  assert_eq!(A::N_POINTS, Model::N_POINTS);
  assert_eq!(A::WEIGHT, Model::WEIGHT);
}