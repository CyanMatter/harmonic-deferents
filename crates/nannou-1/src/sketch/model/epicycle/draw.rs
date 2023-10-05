use nannou::{geom::ellipse::Ellipse, prelude::Point2};

pub fn draw_epicycles(seq: &Vec<Epicycle>) {
  let pos: Point2 = [0, 0];
  for epicycle in seq {
    pos = draw_epicycle(&epicycle);
  }
}

fn draw_epicycle(epicycle: &Epicycle, root: Point2) -> Point2 {
  let circle = Ellipse::radius(epicycle.radius);
  let vector: Point2 = epicycle.vector.to_point2;
  let arrow = Arrow::points(root, root + vector);
  return 
}