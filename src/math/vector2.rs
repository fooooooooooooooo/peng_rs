use crate::bindings;

pub struct Vector2f {
  pub(crate) raw: bindings::Vector2f,
}

impl Vector2f {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      raw: bindings::Vector2f { x, y },
    }
  }
}

pub struct Vector2d {
  pub(crate) raw: bindings::Vector2d,
}

impl Vector2d {
  pub fn new(x: f64, y: f64) -> Self {
    Self {
      raw: bindings::Vector2d { x, y },
    }
  }
}

pub struct Vector2i {
  pub(crate) raw: bindings::Vector2i,
}

impl Vector2i {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      raw: bindings::Vector2i { x, y },
    }
  }
}

pub struct Vector2u {
  pub(crate) raw: bindings::Vector2u,
}

impl Vector2u {
  pub fn new(x: u32, y: u32) -> Self {
    Self {
      raw: bindings::Vector2u { x, y },
    }
  }
}

pub struct Vector2u8 {
  pub(crate) raw: bindings::Vector2u8,
}

impl Vector2u8 {
  pub fn new(x: u8, y: u8) -> Self {
    Self {
      raw: bindings::Vector2u8 { x, y },
    }
  }
}
