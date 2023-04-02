use std::{fmt, ops};
use super::{signed_val, s_val_last, display_header};
use super::{Linear, Quadratic, Quartic, Polynomial};
/// A struct that contains the constants of an equation
/// in the form of ax^3 + bx^2 + cx + d.
/// Some useful functions are also implemented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cubic {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64
}

impl fmt::Display for Cubic {
  /// Displays the Cubic.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let part1 = display_header(self.a, String::from("x^3"));
    let part2 = signed_val(self.b, String::from("x^2"));  
    let part3 = signed_val(self.c, String::from("x"));
    let part4 = s_val_last(self.d);
    write!(f, "{part1}{part2}{part3}{part4}")
  }
}

impl ops::Add<Cubic> for Cubic {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.a + other.a, 
      self.b + other.b,
      self.c + other.c,
      self.d + other.d
    )
  }
}

impl ops::Add<Quadratic> for Cubic {
  type Output = Self;
  fn add(self, other: Quadratic) -> Self::Output {
    Self {
      b: self.b + other.a,
      c: self.c + other.b,
      d: self.d + other.c,
      ..self
    }
  }
}

impl ops::Add<Linear> for Cubic {
  type Output = Self;
  fn add(self, other: Linear) -> Self::Output {
    Self {
      c: self.c + other.a,
      d: self.d + other.b,
      ..self
    }
  }
}

impl ops::Add<f64> for Cubic {
  type Output = Self;
  fn add(self, other: f64) -> Self::Output {
    Self {
      d: self.d + other,
      ..self
    }
  }
}
  
impl ops::Sub<Cubic> for Cubic {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.a - other.a,
      self.b - other.b,
      self.c - other.c,
      self.d - other.d
    )
  }
}

impl ops::Sub<Quadratic> for Cubic {
  type Output = Self;
  fn sub(self, other: Quadratic) -> Self::Output {
    Self {
      b: self.b - other.a,
      c: self.c - other.b,
      d: self.d - other.c,
      ..self
    }
  }
}

impl ops::Sub<Linear> for Cubic {
  type Output = Self;
  fn sub(self, other: Linear) -> Self::Output {
    Self {
      c: self.c - other.a,
      d: self.d - other.b,
      ..self
    }
  }
}

impl ops::Sub<f64> for Cubic {
  type Output = Self;
  fn sub(self, other: f64) -> Self::Output {
    Self {
      d: self.d - other,
      ..self
    }
  }
}

impl ops::Mul<Linear> for Cubic {
  type Output = Quartic;
  fn mul(self, other: Linear) -> Self::Output {
    Self::Output::new(
      self.a * other.a,
      self.a * other.b + self.b * other.a,
      self.b * other.b + self.c * other.a,
      self.c * other.b + self.d * other.a,
      self.d * other.b
    )
  }
}

impl ops::Mul<f64> for Cubic {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Self::new(
      self.a * other,
      self.b * other,
      self.c * other,
      self.d * other
    )
  }
}
  
impl Polynomial for Cubic {
  /// Evaluates the Cubic for the given x.
  fn evaluate(&self, x: f64) -> f64 {
    (self.a * x * x * x) +
    (self.b * x * x) +
    (self.c * x) +
    self.d
  }

  fn is_zero(&self) -> bool {
    self.a == 0.0 &&
    self.b == 0.0 &&
    self.c == 0.0 &&
    self.d == 0.0
  }

  fn degree(&self) -> u8 { 3 }
}
  
impl Cubic {
  /// Creates a new Cubic struct from the values given.
  pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
    Self { a, b, c, d }
  }

  /// Creates a new Cubic struct from the values given.
  pub fn new_i(a: i32, b: i32, c: i32, d: i32) -> Self {
    Self {
      a: a.into(),
      b: b.into(),
      c: c.into(),
      d: d.into()
    }
  }
}
