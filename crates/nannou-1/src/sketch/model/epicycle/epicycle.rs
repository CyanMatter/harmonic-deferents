pub struct Epicycle {
  pub radius: f32,
  pub frequency: i64,
  pub phase: f32,
  pub vector: Position,
}

pub struct Position {
  pub x: f32,
  pub y: f32,
  pub t: f32
}

pub fn position_at_absolute_offset(time: f32, radius: f32, frequency: i64, phase: f32) -> Position {
  let c: f32 = (frequency as f32) * time + phase;
  Position {
    x: radius * c.cos(),
    y: radius * c.sin(),
    t: time
  }
}

pub fn position_at_root(radius: f32, frequency: i64, phase: f32) -> Position {
  position_at_absolute_offset(1_f32, radius, frequency, phase)
}


impl Epicycle {
  fn update_position_to_absolute_offset(&mut self, time: f32) {
    self.vector = position_at_absolute_offset(time, self.radius, self.frequency, self.phase);
  }

  fn update_position_to_relative_offset(&mut self, time: f32) {
    self.vector = position_at_absolute_offset(self.vector.t + time, self.radius, self.frequency, self.phase);
  }

  fn update_position_to_root(&mut self) {
    self.vector = position_at_absolute_offset(1_f32, self.radius, self.frequency, self.phase);
  }
}

// A special epicycle that has a radius of 0.
pub trait Null {
  const NULL: Epicycle = Epicycle {
    radius: 0_f32,
    frequency: 0,
    phase: 0_f32,
    vector: Position {
      x: 0_f32,
      y: 0_f32,
      t: 1_f32
    }
  };
}