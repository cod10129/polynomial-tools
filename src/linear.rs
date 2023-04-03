use std::{fmt, ops};
use super::{s_val_last, display_header};
use super::{Quadratic, Polynomial};
/// A struct that contains the constants of an equation
/// in the form ax + b.
/// Some useful functions are also implemented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Linear {
  pub a: f64,
  pub b: f64
}

impl fmt::Display for Linear {
  /// Displays the Linear.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let part1 = display_header(self.a, String::from("x"));
    let part2 = s_val_last(self.b);
    write!(f, "{part1}{part2}")
  }
}

impl ops::Add<Linear> for Linear {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.a + other.a, 
      self.b + other.b
    )
  }
}

impl ops::Add<f64> for Linear {
  type Output = Self;
  fn add(self, other: f64) -> Self::Output {
    Self::new(self.a, self.b + other)
  }
}

impl ops::Sub<Linear> for Linear {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.a - other.a, 
      self.b - other.b
    )
  }
}

impl ops::Sub<f64> for Linear {
  type Output = Self;
  fn sub(self, other: f64) -> Self::Output {
    Self::new(self.a, self.b - other)
  }
}

impl ops::Mul<Linear> for Linear {
  type Output = Quadratic;
  fn mul(self, other: Linear) -> Self::Output {
    Self::Output::new(
      self.a * other.a,
      self.a * other.b + self.b * other.a,
      self.b * other.b
    )
  }
}
  
impl ops::Mul<f64> for Linear {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Self::new(self.a * other, self.b * other)
  }
}

impl Polynomial for Linear {
  /// Evaluates the Linear at the given x.
  fn evaluate(&self, x: f64) -> f64 {
    (self.a * x) +
    self.b
  }
  fn is_zero(&self) -> bool {
    self.a == 0.0 &&
    self.b == 0.0
  }
  fn degree(&self) -> u8 { 1 }

  type Derivative = f64;
  fn derivative(&self) -> Self::Derivative {
    self.a
  }
}

impl Linear {
  /// Creates a new Linear from the values given.
  pub fn new(a: f64, b: f64) -> Self {
    Self { a, b }
  }

  /// Creates a new Linear from the values given.
  pub fn new_i(a: i32, b: i32) -> Self {
    Self {
      a: a.into(),
      b: b.into()
    }
  }

  /// Finds the root of the Linear
  /// 
  /// # Errors:
  /// If no (or infinitely many) roots exist, an Err is returned.
  pub fn root(&self) -> Result<f64, &str> {
    if self.a == 0.0 {
      return Err("a must be non-zero");
    }
    Ok(-(self.b / self.a))
  }
}
