use std::ops::{Add, Mul, Sub};
use num_traits::Num;
use num_traits::Float;

use super::fuzzy_eq::*;

#[derive(Debug, Copy, Clone)]
pub struct Color<T = f64>
where
	T: Float,
{
	pub red : T,
	pub green : T,
	pub blue : T,
}

impl<T: Float>	Color<T>{
	pub fn new(red: T, green: T, blue: T) -> Self {
		Self {red, green, blue}
	}
	pub	fn black() -> Self{
		Color::new(T::zero(), T::zero(), T::zero())
	}
	pub fn	clamp(&self, lower_bound: T, upper_bound: T) -> Color<T>{
		Color::new(
			self.red.min(upper_bound).max(lower_bound),
			self.green.min(upper_bound).max(lower_bound),
			self.blue.min(upper_bound).max(lower_bound),
		)
	}
}

impl<T: Float> Add for Color<T> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Color::new(
			self.red + other.red,
			self.green + other.green,
			self.blue + other.blue,
		)
	}
}

impl<T: Float> Sub for Color<T> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Color::new(
			self.red - other.red,
			self.green - other.green,
			self.blue - other.blue,
		)
	}
}

impl<T, U> Mul<U> for Color<T>
where
  T: Float,
  T: From<U>,
  U: Num,
{
  type Output = Color<T>;

  fn mul(self, other: U) -> Self::Output {
    let multiplicator: T = other.into();
    Color::new(
      self.red * multiplicator,
      self.green * multiplicator,
      self.blue * multiplicator,
    )
  }
}

impl<T>	Mul<Color<T>> for Color<T>
where
	T:Float
{
	type Output = Color<T>;
	fn mul(self, other: Self) -> Self::Output{
		Color::new(
			self.red * other.red,
			self.green * other.green,
			self.blue * other.blue,
		)
	}
}

impl<T> PartialEq<Color<T>> for Color<T>
where
	T: Float,
	T: FuzzyEq<T>,
{
	fn eq(&self, other: &Self) -> bool {
		return self.red.fuzzy_eq(other.red)
		&& self.blue.fuzzy_eq(other.blue)
		&& self.green.fuzzy_eq(other.green)
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