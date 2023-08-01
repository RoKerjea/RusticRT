use std::ops::{Add, Mul, Sub};

use crate::F;

use super::fuzzy_eq::*;

#[derive(Debug, Copy, Clone)]
pub struct Color{
	pub red : F,
	pub green : F,
	pub blue : F,
}

impl Color{
	pub fn new(red: F, green: F, blue: F) -> Self {
		Self {red, green, blue}
	}
	pub	fn black() -> Self{
		Color::new(0.0, 0.0, 0.0)
	}
	pub	fn white() -> Self{
		Color::new(1.0, 1.0, 1.0)
	}
	pub fn	clamp(&self, lower_bound: F, upper_bound: F) -> Color{
		Color::new(
			self.red.min(upper_bound).max(lower_bound),
			self.green.min(upper_bound).max(lower_bound),
			self.blue.min(upper_bound).max(lower_bound),
		)
	}
}

impl Add for Color {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Color::new(
			self.red + other.red,
			self.green + other.green,
			self.blue + other.blue,
		)
	}
}

impl Sub for Color {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Color::new(
			self.red - other.red,
			self.green - other.green,
			self.blue - other.blue,
		)
	}
}

impl Mul<F> for Color
{
  type Output = Color;

  fn mul(self, other: F) -> Self::Output {
    let multiplicator: F = other.into();
    Color::new(
      self.red * multiplicator,
      self.green * multiplicator,
      self.blue * multiplicator,
    )
  }
}

impl	Mul<Color> for Color {
	type Output = Color;
	fn mul(self, other: Self) -> Self::Output{
		Color::new(
			self.red * other.red,
			self.green * other.green,
			self.blue * other.blue,
		)
	}
}

impl PartialEq<Color> for Color
{
	fn eq(&self, other: &Self) -> bool {
		return self.red.fuzzy_eq(other.red)
		&& self.blue.fuzzy_eq(other.blue)
		&& self.green.fuzzy_eq(other.green)
	}
}

impl FuzzyEq<Color> for Color {
	fn fuzzy_eq(&self, other: Self) -> bool {
	  self.red.fuzzy_eq(other.red)
		&& self.green.fuzzy_eq(other.green)
		&& self.blue.fuzzy_eq(other.blue)
	}
  }

#[cfg(test)]
mod	tests {
	use super::*;
	#[test]
	fn	color_new()
	{
		let col = Color::new(-0.5, 0.4, 1.7);
		assert_eq!(col.red, -0.5);
		assert_eq!(col.green, 0.4);
		assert_eq!(col.blue, 1.7);
	}

	#[test]
	fn	color_addition()
	{
		let col1 = Color::new(0.9, 0.6, 0.75);
		let col2 = Color::new(0.7, 0.1, 0.25);
		let expected_col = Color::new(1.6, 0.7, 1.0);
		assert_eq!(col1 + col2, expected_col);
	}
	#[test]
	fn	color_substraction()
	{
		let col1 = Color::new(0.9, 0.6, 0.75);
		let col2 = Color::new(0.7, 0.1, 0.25);
		let expected_col = Color::new(0.2, 0.5, 0.5);
		assert_eq!(col1 - col2, expected_col);
	}
	#[test]
	fn	color_scalar_multiplication()
	{
		let col1 = Color::new(0.2, 0.3, 0.4);
		let expected_col = Color::new(0.4, 0.6, 0.8);
		assert_eq!(col1 * 2.0, expected_col);
	}
	#[test]
	fn	color_multiplication()
	{
		let col1 = Color::new(1.0, 0.2, 0.4);
		let col2 = Color::new(0.9, 1.0, 0.1);
		let expected_col = Color::new(0.9, 0.2, 0.04);
		assert_eq!(col1 * col2, expected_col);
	}
	#[test]
	fn	color_clamping()
	{
		let col1 = Color::new(1.1, 2.3, -6.4);
		let expected_col = Color::new(1.0, 1.0, 0.0);
		assert_eq!(col1.clamp(0.0, 1.0), expected_col);
	}

}