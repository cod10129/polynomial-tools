use std::{fmt, ops};
use super::{signed_val, s_val_last, display_header};
use super::{Linear, Quadratic, Cubic, Polynomial};
/// A struct that contains the constants of an equation
/// in the form of ax^4 + bx^3 + cx^2 + dx + e.
/// Some useful functions are also implemented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quartic {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64,
  pub e: f64
}

impl fmt::Display for Quartic {
  /// Displays the Quartic.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let part1 = display_header(self.a, String::from("x^4"));
    let part2 = signed_val(self.b, String::from("x^3"));  
    let part3 = signed_val(self.c, String::from("x^2"));
    let part4 = signed_val(self.d, String::from("x"));
    let part5 = s_val_last(self.e);
    write!(f, "{part1}{part2}{part3}{part4}{part5}")
  }
}

impl ops::Add<Quartic> for Quartic {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.a + other.a,
      self.b + other.b,
      self.c + other.c,
      self.d + other.d,
      self.e + other.e
    )
  }
}

impl ops::Add<Cubic> for Quartic {
  type Output = Self;
  fn add(self, other: Cubic) -> Self::Output {
    Self {
      b: self.b + other.a,
      c: self.c + other.b,
      d: self.d + other.c,
      e: self.e + other.d,
      ..self
    }
  }
}

impl ops::Add<Quadratic> for Quartic {
  type Output = Self;
  fn add(self, other: Quadratic) -> Self::Output {
    Self {
      c: self.c + other.a,
      d: self.d + other.b,
      e: self.e + other.c,
      ..self
    }
  }
}

impl ops::Add<Linear> for Quartic {
  type Output = Self;
  fn add(self, other: Linear) -> Self::Output {
    Self {
      d: self.d + other.a,
      e: self.e + other.b,
      ..self
    }
  }
}

impl ops::Add<f64> for Quartic {
  type Output = Self;
  fn add(self, other: f64) -> Self::Output {
    Self {
      e: self.e + other,
      ..self
    }
  }
}

impl ops::Sub<Quartic> for Quartic {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.a - other.a,
      self.b - other.b,
      self.c - other.c,
      self.d - other.d,
      self.e - other.e
    )
  }
}

impl ops::Sub<Cubic> for Quartic {
  type Output = Self;
  fn sub(self, other: Cubic) -> Self::Output {
    Self {
      b: self.b - other.a,
      c: self.c - other.b,
      d: self.d - other.c,
      e: self.e - other.d,
      ..self
    }
  }
}

impl ops::Sub<Quadratic> for Quartic {
  type Output = Self;
  fn sub(self, other: Quadratic) -> Self::Output {
    Self {
      c: self.c - other.a,
      d: self.d - other.b,
      e: self.e - other.c,
      ..self
    }
  }
}

impl ops::Sub<Linear> for Quartic {
  type Output = Self;
  fn sub(self, other: Linear) -> Self::Output {
    Self {
      d: self.d - other.a,
      e: self.e - other.b,
      ..self
    }
  }
}

impl ops::Sub<f64> for Quartic {
  type Output = Self;
  fn sub(self, other: f64) -> Self::Output {
    Self {
      e: self.e - other,
      ..self
    }
  }
}

impl ops::Mul<f64> for Quartic {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Self::new(
      self.a * other,
      self.b * other,
      self.c * other,
      self.d * other,
      self.e * other
    )
  }
}
  
impl Polynomial for Quartic {
  /// Evaluates the Quartic for the given x.
  fn evaluate(&self, x: f64) -> f64 {
    (self.a * x * x * x * x) +
    (self.b * x * x * x) +
    (self.c * x * x) +
    (self.d * x) +
    (self.e)
  }
  fn is_zero(&self) -> bool {
    self.a == 0.0 &&
    self.b == 0.0 &&
    self.c == 0.0 &&
    self.d == 0.0 &&
    self.e == 0.0
  }
  fn degree(&self) -> u8 { 4 }

  type Derivative = Cubic;
  fn derivative(&self) -> Self::Derivative {
    Cubic::new(
      self.a * 4.0,
      self.b * 3.0,
      self.c * 2.0,
      self.d
    )
  }
}

impl Quartic {
  /// Creates a new Quartic from the values given.
  pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64) -> Self {
    Self { a, b, c, d, e }
  }

  /// Creates a new Quartic from the values given.
  pub fn new_i(a: i32, b: i32, c: i32, d: i32, e: i32) -> Self {
    Self {
      a: a.into(),
      b: b.into(),
      c: c.into(),
      d: d.into(),
      e: e.into()
    }
  }
}
