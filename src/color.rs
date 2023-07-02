use std::ops;

use super::util::*;

#[derive(Debug, Copy, Clone)]
pub struct Color{
	pub red : f64,
	pub green : f64,
	pub blue : f64,
}

impl	Color{
	pub fn new(red: f64, green: f64, blue: f64) -> Self {
		Self {red, green, blue}
	}
	pub	fn black() -> Self{
		Color::new(0.0, 0.0, 0.0)
	}
}

impl ops::Add<Self> for Color {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Color::new(
			self.red + other.red,
			self.green + other.green,
			self.blue + other.blue,
		)
	}
}

impl ops::Sub<Self> for Color {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Color::new(
			self.red - other.red,
			self.green - other.green,
			self.blue - other.blue,
		)
	}
}

impl	ops::Mul<f64> for Color{
	type Output = Self;
	fn mul(self, other: f64) -> Self{
		Color::new(
			self.red * other,
			self.green * other,
			self.blue * other,
		)
	}
}

impl	ops::Mul<Color> for Color{
	type Output = Color;
	fn mul(self, other: Color) -> Self{
		Color::new(
			self.red * other.red,
			self.green * other.green,
			self.blue * other.blue,
		)
	}
}

impl PartialEq<Color> for Color {
	fn eq(&self, other: &Self) -> bool {
		return epsil_compare(self.red, other.red)
		&& epsil_compare(self.blue, other.blue)
		&& epsil_compare(self.green, other.green);
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
}