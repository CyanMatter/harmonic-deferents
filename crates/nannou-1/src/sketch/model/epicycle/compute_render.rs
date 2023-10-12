use nannou::{ prelude::Point2, draw::{ primitive::{ellipse::Ellipse, polygon::SetPolygon}, properties::{SetPosition, SetStroke} }};
use crate::sketch::{Model, model::Constants};

/* Computes the geometric shapes that will be rendered according to the data in 
 * `model.epicycles` at `model.t_elapsed`:
 * Updates the model with up-to-date circles and path. These are then available to be 
 * drawn using the app API.
 * Updates the `vector` field of each epicycle in `model.epicycles`.
 */
pub fn compute_all_renders(model: &mut Model) {
  create_path(model);
  create_ellipses(model);
}

// Computes the position of each epicycle according to the vectors in each Epicycle.
fn create_path(model: &mut Model) {
  let mut path: Vec<Point2> = Vec::with_capacity(1 + model.epicycles.len());
  path.push(Point2::ZERO);
  let mut pos = Point2::ZERO;

  for epicycle in model.epicycles.iter_mut() {
    pos += epicycle.vector_at(model.t_frac_of_period_elapsed);
    path.push(pos);
  }

  model.epicycle_path = path;
}

fn create_ellipses(model: &mut Model) {
  for (epicycle, pos) in model.epicycles.iter_mut().zip(model.epicycle_path.iter()) {
    epicycle.ellipse = Some(
      Ellipse::radius(Ellipse::default(), epicycle.radius)
        .xy(*pos)
        .stroke(nannou::color::CRIMSON)
        .stroke_weight(Model::WEIGHT)
        .no_fill()
    );
  }
}