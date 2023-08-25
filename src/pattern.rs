use crate::body::{Body, Intersectable};
use crate::color::Color;
use crate::fuzzy_eq::FuzzyEq;
use crate::matrix::Matrix;
use crate::tuple::Tuple;


pub trait Stencil {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color;
    fn transform(&self) -> Matrix<4>;
    fn color_at(&self, position: Tuple, body: &Body) -> Color{
      let object_position = body.transform().inverse() * position;
      //apply pattern transform to color at position
      let pattern_position = self.transform().inverse() * object_position;
      self.color_at_in_pattern_space(pattern_position)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pattern {
    Striped(Striped),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
}

impl FuzzyEq<Pattern> for Pattern {
    fn	fuzzy_eq(&self, other: Pattern) -> bool {
    match (self, other) {
        (Pattern::Striped(ref striped), Pattern::Striped(other)) => striped.fuzzy_eq(other),
        (Pattern::Gradient(ref gradient), Pattern::Gradient(other)) => gradient.fuzzy_eq(other),
        (Pattern::Ring(ref ring), Pattern::Ring(other)) => ring.fuzzy_eq(other),
        (Pattern::Checker(ref checker), Pattern::Checker(other)) => checker.fuzzy_eq(other),
        _ => false,
      }
    }
}

impl Stencil for Pattern {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        match *self {
            Pattern::Striped(ref striped) => striped.color_at_in_pattern_space(position),
            Pattern::Gradient(ref gradient) => gradient.color_at_in_pattern_space(position),
            Pattern::Ring(ref ring) => ring.color_at_in_pattern_space(position),
            Pattern::Checker(ref checker) => checker.color_at_in_pattern_space(position),
        }
    }
    fn transform(&self) -> Matrix<4> {
      match *self {
        Pattern::Striped(ref striped) => striped.transform(),
        Pattern::Gradient(ref gradient) => gradient.transform(),
        Pattern::Ring(ref ring) => ring.transform(),
        Pattern::Checker(ref checker) => checker.transform(),
      }
    }
}

impl From<Gradient> for Pattern {
  fn from(striped: Gradient) -> Self {
      Pattern::Gradient(striped)
  }
}

impl From<Ring> for Pattern {
  fn from(ring: Ring) -> Self {
      Pattern::Ring(ring)
  }
}
impl From<Striped> for Pattern {
    fn from(striped: Striped) -> Self {
        Pattern::Striped(striped)
    }
}

impl From<Checker> for Pattern {
  fn from(checker: Checker) -> Self {
      Pattern::Checker(checker)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Striped {
    color_a: Color,
    color_b: Color,
    transform: Matrix<4>,
}

impl Striped {
    pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
        self.color_a = color_a;
        self.color_b = color_b;
        self
    }

    pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self
    }
}

impl Default for Striped {
    fn default() -> Self {
        Self {
            color_a: Color::black(),
            color_b: Color::white(),
            transform: Matrix::identity(),
        }
    }
}

impl Stencil for Striped {
    fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
        let x = position.x;
        if x.floor() as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
    fn transform(&self) -> Matrix<4> {
      self.transform
    }
}

impl FuzzyEq<Striped> for Striped {
    fn fuzzy_eq(&self, other: Striped) -> bool {
      self.color_b.fuzzy_eq(other.color_b) && self.color_b.fuzzy_eq(other.color_b)
    }
  }

  #[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Gradient {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl  Default for Gradient {
  fn default() -> Self {
    Self {
      color_a: Color::green(),
      color_b: Color::red(),
      transform: Matrix::identity(),
    }
  }
}

impl Stencil for Gradient {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let distance = self.color_b - self.color_a;
    let fraction = position.x - position.x.floor();
    self.color_a + distance * fraction
  }
  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

impl FuzzyEq<Gradient> for Gradient {
  fn fuzzy_eq(&self, other: Gradient) -> bool {
    self.color_b.fuzzy_eq(other.color_b) && self.color_b.fuzzy_eq(other.color_b)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Ring {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl  Default for Ring {
  fn default() -> Self {
    Self {
      color_a: Color::green(),
      color_b: Color::red(),
      transform: Matrix::identity(),
    }
  }
}

impl Stencil for Ring {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    if (position.x.powi(2) + position.z.powi(2)).sqrt().floor() as isize % 2 == 0 {
      self.color_a
    } else {
      self.color_b
    }
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

impl FuzzyEq<Ring> for Ring {
  fn fuzzy_eq(&self, other: Ring) -> bool {
    self.color_b.fuzzy_eq(other.color_b) && self.color_b.fuzzy_eq(other.color_b)
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checker {
  color_a: Color,
  color_b: Color,
  transform: Matrix<4>,
}

impl Checker {
  pub fn with_colors(mut self, color_a: Color, color_b: Color) -> Self {
    self.color_a = color_a;
    self.color_b = color_b;
    self
  }

  pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
    self.transform = transform;
    self
  }
}

impl  Default for Checker {
  fn default() -> Self {
    Self {
      color_a: Color::white(),
      color_b: Color::black(),
      transform: Matrix::identity(),
    }
  }
}

impl Stencil for Checker {
  fn color_at_in_pattern_space(&self, position: Tuple) -> Color {
    let x = position.x;
    let y = position.y;
    let z = position.z;
        if (x.floor() + y.floor() + z.floor())  as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
  }

  fn transform(&self) -> Matrix<4> {
    self.transform
  }
}

impl FuzzyEq<Checker> for Checker {
  fn fuzzy_eq(&self, other: Checker) -> bool {
    self.color_b.fuzzy_eq(other.color_b) && self.color_b.fuzzy_eq(other.color_b)
  }
}

  #[cfg(test)]
mod tests {
  use crate::{assert_fuzzy_eq, sphere::Sphere};

use super::*;

  #[test]
  fn a_stripe_pattern_is_constant_in_y() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 1.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 2.0, 0.0), &body)
    );
  }

  #[test]
  fn a_stripe_pattern_is_constant_in_z() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 1.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 2.0), &body)
    );
  }

  #[test]
  fn a_stripe_pattern_alternates_in_x() {
    let pattern = Striped::default();
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.9, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(1.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.1, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(-0.1, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(-1.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(-1.1, 0.0, 0.0), &body)
    );
  }
  #[test]
  fn striped_pattern_adheres_to_object_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(Striped::default().with_colors(Color::black(), Color::white()));
    let body = Body::from(Sphere::default().with_transform(transform));

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.5, 0.0, 0.0), &body)
    );
  }

  #[test]
  fn striped_pattern_adheres_to_pattern_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(Color::black(), Color::white())
        .with_transform(transform),
    );
    let body = Body::from(Sphere::default());

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.5, 0.0, 0.0), &body)
    );
  }

  #[test]
  fn striped_pattern_adheres_to_object_and_pattern_transform() {
    let transform = Matrix::scaling(2.0, 2.0, 2.0);
    let pattern = Pattern::from(
      Striped::default()
        .with_colors(Color::black(), Color::white())
        .with_transform(transform),
    );
    let body = Body::from(Sphere::default().with_transform(transform));

    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(3.5, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(4.0, 0.0, 0.0), &body)
    );
  }
  #[test]
  fn a_gradient_linearly_interpolates_between_colors() {
    // let pattern = Gradient::with_colors(white, black);
    let body = Body::from(Sphere::default());
    let pattern = Pattern::from(
      Gradient::default()
        .with_colors(Color::white(), Color::black()),
    );
    assert_fuzzy_eq!(
      Color::new(1.0, 1.0, 1.0),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::new(0.75, 0.75, 0.75),
      pattern.color_at(Tuple::point(0.25, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::new(0.5, 0.5, 0.5),
      pattern.color_at(Tuple::point(0.5, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::new(0.25, 0.25, 0.25),
      pattern.color_at(Tuple::point(0.75, 0.0, 0.0), &body)
    );
  }
  #[test]
  fn a_ring_should_extend_in_both_x_and_z()
  {
    let pattern = Pattern::from(
      Ring::default()
        .with_colors(Color::white(), Color::black()),
    );
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 0.0, 1.0), &body)
    );
    // 0.708 = just slightly more than √2/2
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.708, 0.0, 0.708), &body)
    );
  }
  #[test]
  fn checker_should_repeat_in_x(){
    let pattern = Pattern::from(
      Checker::default()
        .with_colors(Color::white(), Color::black()),
    );
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.99, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(1.01, 0.0, 0.0), &body)
    );
  }
  #[test]
  fn checker_should_repeat_in_y(){
    let pattern = Pattern::from(
      Checker::default()
        .with_colors(Color::white(), Color::black()),
    );
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.99, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point(0.0, 1.01, 0.0), &body)
    );
  }
  #[test]
  fn checker_should_repeat_in_z(){
    let pattern = Pattern::from(
      Checker::default()
        .with_colors(Color::white(), Color::black()),
    );
    let body = Body::from(Sphere::default());
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.0), &body)
    );
    assert_fuzzy_eq!(
      Color::white(),
      pattern.color_at(Tuple::point(0.0, 0.0, 0.99), &body)
    );
    assert_fuzzy_eq!(
      Color::black(),
      pattern.color_at(Tuple::point( 0.0, 0.0, 1.01), &body)
    );
  }
//   Scenario: Checkers should repeat in x
// Given pattern ← checkers_pattern(white, black)
// Then pattern_at(pattern, point(0, 0, 0)) = white
// And pattern_at(pattern, point(0.99, 0, 0)) = white
// And pattern_at(pattern, point(1.01, 0, 0)) = black
// Scenario: Checkers should repeat in y
// Given pattern ← checkers_pattern(white, black)
// Then pattern_at(pattern, point(0, 0, 0)) = white
// And pattern_at(pattern, point(0, 0.99, 0)) = white
// And pattern_at(pattern, point(0, 1.01, 0)) = black
// Scenario: Checkers should repeat in z
// Given pattern ← checkers_pattern(white, black)
// Then pattern_at(pattern, point(0, 0, 0)) = white
// And pattern_at(pattern, point(0, 0, 0.99)) = white
// And pattern_at(pattern, point(0, 0, 1.01)) = black
}