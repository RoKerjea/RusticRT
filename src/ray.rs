use crate::matrix::Matrix;
use crate::tuple::*;
use crate::F;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
	pub origin: Tuple,
	pub direction: Tuple,
}

impl Ray{
	pub fn new(origin: Tuple, direction: Tuple) -> Self {
		Ray {origin, direction}
	}
	pub fn	position(&self, t: F) -> Tuple {
		self.origin + self.direction * t
	}
	pub fn	transform(&self, m: Matrix<4>) -> Self {
		Ray {
			origin: m * self.origin,
			direction: m * self.direction,
		}
	}
}

#[cfg(test)]
mod tests{
	use super::*;
    use crate::{tuple::Tuple, matrix::Matrix};

	#[test]
	fn	creating_and_querying_a_ray()
	{
		let origin = Tuple::point(1.0, 2.0, 3.0);
		let direction = Tuple::vector(4.0, 5.0, 6.0);
		let r = Ray::new(origin, direction);
		assert_eq!(r.origin, origin);
		assert_eq!(r.direction, direction);
	}
	#[test]
	fn	computing_point_from_distance()
	{
		let origin = Tuple::point(2.0, 3.0, 4.0);
		let direction = Tuple::vector(1.0, 0.0, 0.0);
		let r = Ray::new(origin, direction);
		assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
		assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
		assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
		assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
	}
	#[test]
	fn	translating_a_ray()
	{
		let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
		let m = Matrix::translation(3.0, 4.0, 5.0);
		let r2 = r.transform(m);
		assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
		assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
	}
	#[test]
	fn	scaling_a_ray()
	{
		let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
		let m = Matrix::scaling(2.0, 3.0, 4.0);
		let r2 = r.transform(m);
		assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
		assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
	}
}