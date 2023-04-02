use std::{fmt, ops};
use super::{signed_val, s_val_last, display_header};
use super::{Linear, Cubic, Quartic, Polynomial};
/// A struct that contains the constants of an equation
/// in the form of ax^2 + bx + c.
/// Several useful functions are also implemented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quadratic {
  pub a: f64,
  pub b: f64,
  pub c: f64
}

impl fmt::Display for Quadratic {
  /// Displays the Quadratic.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let part1 = display_header(self.a, String::from("x^2"));
    let part2 = signed_val(self.b, String::from("x"));
    let part3 = s_val_last(self.c);
    write!(f, "{part1}{part2}{part3}")
  }
}

impl ops::Add<Quadratic> for Quadratic {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.a + other.a, 
      self.b + other.b,
      self.c + other.c
    )
  }
}

impl ops::Add<Linear> for Quadratic {
  type Output = Self;
  fn add(self, other: Linear) -> Self::Output {
    Self {
      b: self.b + other.a,
      c: self.c + other.b,
      ..self
    }
  }
}

impl ops::Add<f64> for Quadratic {
  type Output = Self;
  fn add(self, other: f64) -> Self::Output {
    Self {
      c: self.c + other,
      ..self
    }
  }
}
  
impl ops::Sub<Quadratic> for Quadratic {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.a - other.a, 
      self.b - other.b,
      self.c - other.c
    )
  }
}

impl ops::Sub<Linear> for Quadratic {
  type Output = Self;
  fn sub(self, other: Linear) -> Self::Output {
    Self {
      b: self.b - other.a,
      c: self.c - other.b,
      ..self
    }
  }
}

impl ops::Sub<f64> for Quadratic {
  type Output = Self;
  fn sub(self, other: f64) -> Self::Output {
    Self {
      c: self.c - other,
      ..self
    }
  }
}

impl ops::Mul<Quadratic> for Quadratic {
  type Output = Quartic;
  fn mul(self, other: Quadratic) -> Self::Output {
    Self::Output::new(
      self.a * other.a,
      self.a * other.b + self.b * other.a,
      self.a * other.c + self.b * other.b + self.c * other.a,
      self.b * other.c + self.c * other.b,
      self.c * other.c
    )
  }
}

impl ops::Mul<Linear> for Quadratic {
  type Output = Cubic;
  fn mul(self, other: Linear) -> Self::Output {
    Self::Output::new(
      self.a * other.a,
      self.a * other.b + self.b * other.a,
      self.b * other.b + self.c * other.a,
      self.c * other.b
    )
  }
}
  
impl ops::Mul<f64> for Quadratic {
  type Output = Self;
  fn mul(self, other: f64) -> Self::Output {
    Self::new(
      self.a * other,
      self.b * other,
      self.c * other
    )
  }
}

impl Polynomial for Quadratic {
  /// Evaluates the Quadratic with the given x.
  fn evaluate(&self, x: f64) -> f64 {
    (self.a * x * x) + 
    (self.b * x) + 
    self.c
  }
  fn is_zero(&self) -> bool {
    self.a == 0.0 &&
    self.b == 0.0 &&
    self.c == 0.0
  }
  fn degree(&self) -> u8 { 2 }
}

impl Quadratic {
  /// Creates a new Quadratic from the values given.
  pub fn new(a: f64, b: f64, c: f64) -> Self {
    Self { a, b, c }
  }

  /// Creates a new Quadratic from the values given.
  pub fn new_i(a: i32, b: i32, c: i32) -> Self {
    Self {
      a: a.into(),
      b: b.into(),
      c: c.into()
    }
  }
  
  /// Returns the vertex of the parabola described by the Quadratic
  /// 
  /// # Errors:
  /// An Err is returned if self.a is equal to 0.
  pub fn vertex(&self) -> Result<[f64; 2], &str> {
    if self.a == 0.0 {
      Err("a must be non-zero")
    } else {
      let x_pos = (-self.b) / (2.0 * self.a);
      Ok([
        x_pos,
        self.evaluate(x_pos)
      ])
    }
  }
  
  fn discriminant(&self) -> f64 {
    self.b * self.b - (4.0 * self.a * self.c)
  }

  /// Returns the roots (zeros) of the Quadratic.
  ///
  /// # Errors:
  /// An Err is returned if the Quadratic has no real roots
  pub fn solver(&self) -> Result<[f64; 2], &str> {
    let discriminant = self.discriminant();
    // Check if the roots are complex
    // If so, return an Err
    if discriminant < 0.0 {
      Err("Complex numbers are not supported")
    } else {
      // This is a different formula than usual
      // This is because b^2 - 4ac >= 0 is ensured
      let b2a = self.b / (2.0 * self.a);
      // ((b/2a)^2 - c/a).sqrt()
      let b2a2_ca_rt = (b2a.powi(2) - self.c/self.a).sqrt();
      Ok([
        -(b2a) + b2a2_ca_rt,
        -(b2a) - b2a2_ca_rt
      ])
    }
  }
}
