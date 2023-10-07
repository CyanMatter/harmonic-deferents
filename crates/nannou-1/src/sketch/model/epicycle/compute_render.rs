use nannou::{ prelude::{ Point2, pt2 }, draw::primitive::ellipse::Ellipse };
use crate::sketch::Model;

/* Computes the geometric shapes that will be rendered according to the data in 
 * `model.epicycles` at `model.t_elapsed`:
 * Updates the model with up-to-date circles and path. These are then available to be 
 * drawn using the app API.
 * Updates the `vector` field of each epicycle in `model.epicycles`.
 */
pub fn compute_all_renders(model: &mut Model) {
  let pos = pt2(0_f32, 0_f32);
  let n = model.epicycles.len();
  let mut ellipses: Vec<Ellipse> = Vec::with_capacity(n);
  let mut path: Vec<Point2> = Vec::with_capacity(n + 1);
  path.push(pos);

  for epicycle in &mut model.epicycles {
    let ellipse = Ellipse::radius(Ellipse::default(), epicycle.radius);
    ellipses.push(ellipse);

    let next_pos = epicycle.vector_at(model.t_elapsed);
    path.push(next_pos);
  }
  
  model.ellipses = ellipses;
  model.epicycle_path = path;
}