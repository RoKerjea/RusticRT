use std::ops;
use num_traits::Float;

use crate::fuzzy_eq::*;

#[derive(Debug, Copy, Clone)]
pub struct Tuple<T>
where
	T: Float,
{
	pub x : T,
	pub y : T,
	pub z : T,
	pub w : T,
}

impl<T> Tuple<T>
where
	T: Float,
{
	pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
	pub fn point(x: T, y: T, z: T) -> Self {
        Self { x, y, z, w : T::one()}
	}
	pub fn vector(x: T, y: T, z: T) -> Self {
        Self { x, y, z, w : T::zero()}
	}
	pub fn is_point(&self) -> bool {
        self.w == T::one()
    }
    pub fn is_vector(&self) -> bool {
        self.w == T::zero()
    }
}

impl<T> PartialEq<Self> for Tuple<T>
where
	T: Float,
{
	fn eq(&self, other: &Self) -> bool {
		return self.x.fuzzy_eq(&other.x)
		&& self.y.fuzzy_eq(&other.y)
		&& self.z.fuzzy_eq(&other.z)
		&& self.w.fuzzy_eq(&other.w);
	}
}

impl<T> Tuple<T>
where
	T: Float,
{
	pub fn magnitude(&self) -> T{
		(self.x.powi(2) + self.y.powi(2)
		+ self.z.powi(2) + self.w.powi(2)).sqrt()
	}
	pub fn normalize(&self) -> Tuple<T>{
		*self / self.magnitude()
	}
	pub fn	dot(&self, other: &Self) -> T{
		self.x * other.x +
		self.y * other.y +
		self.z * other.z +
		self.w * other.w
	}
	pub fn	cross(&self, other: &Self) -> Self{
		Tuple::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
			T::zero(),
		)
	}
}

impl<T> ops::Add<Self> for Tuple<T>
where
	T: Float,
{
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
	}
}

impl<T> ops::Sub<Self> for Tuple<T>
where
	T: Float,
{
	type Output = Self;
	fn sub(self, other: Self) -> Self{
		Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
		)
	}
}
impl<T> ops::Mul<T> for Tuple<T>
where
	T: Float,
{
	type Output = Self;
	fn mul(self, other: T) -> Self{
		Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
		)
	}
}
impl<T> ops::Div<T> for Tuple<T>
where
	T: Float,
{
	type Output = Self;
	fn div(self, other: T) -> Self{
		Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
		)
	}
}

impl<T> ops::Neg for Tuple<T>
where
	T: Float,
{
	type Output = Self;
	fn neg(self) -> Self{
		Tuple::new(
            -self.x,
            -self.y,
            -self.z,
        	-self.w,
		)
	}
}

#[cfg(test)]
mod	tests {
	use super::*;

	#[test]
	fn create_tuple_point()
	{
		let point = Tuple::new(4.3, -4.2, 3.1, 1.0);
		assert!(point.x == 4.3);
		assert!(point.y == -4.2);
		assert!(point.z == 3.1);
		assert!(point.w == 1.0);
	}

	#[test]
	fn create_tuple_vector()
	{
		let point = Tuple::new(4.3, -4.2, 3.1, 0.0);
		assert!(point.x == 4.3);
		assert!(point.y == -4.2);
		assert!(point.z == 3.1);
		assert!(point.w == 0.0);
	}
	#[test]
    fn point_does_fill_properties() {
        let point = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
		assert_eq!(point.w, 1.0);
    }
	#[test]
    fn vector_does_fill_properties() {
        let point = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
		assert_eq!(point.w, 0.0);
    }
	// #[test]
    // fn sametup_work() {
    //     let point1 = Tuple::vector(4.3, -4.2, 3.1);
	// 	let point2 = Tuple::vector(4.3, -4.2, 3.1);
	// 	assert!(equal)
    // }
	#[test]
	fn	tuples_additions(){
		let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
		let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
		let tupleexpected = Tuple::new(1.0, 1.0, 6.0, 1.0);

		assert_eq!(tuple1 + tuple2, tupleexpected);
	}
	#[test]
	fn	points_substraction()
	{
		let tuple1 = Tuple::point(3.0, 2.0, 1.0);
		let	tuple2 = Tuple::point(5.0, 6.0, 7.0);
		let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

		assert_eq!(tuple1 - tuple2, expected_tuple);
	}
	#[test]
	fn	vector_substraction_from_point()
	{
		let tuple1 = Tuple::point(3.0, 2.0, 1.0);
		let	tuple2 = Tuple::vector(5.0, 6.0, 7.0);
		let expected_tuple = Tuple::point(-2.0, -4.0, -6.0);

		assert_eq!(tuple1 - tuple2, expected_tuple);
	}
	#[test]
	fn	vectors_substraction()
	{
		let tuple1 = Tuple::vector(3.0, 2.0, 1.0);
		let	tuple2 = Tuple::vector(5.0, 6.0, 7.0);
		let expected_tuple = Tuple::vector(-2.0, -4.0, -6.0);

		assert_eq!(tuple1 - tuple2, expected_tuple);
	}
	#[test]
	fn	vector_substraction_from_zero_vector()
	{
		let tuple1 = Tuple::vector(0.0, 0.0, 0.0);
		let	tuple2 = Tuple::vector(1.0, -2.0, 3.0);
		let expected_tuple = Tuple::vector(-1.0, 2.0, -3.0);

		assert_eq!(tuple1 - tuple2, expected_tuple);
	}
	#[test]
	fn	negating_tuple()
	{
		let tuple1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
		let expected_tuple = Tuple::new(-1.0, 2.0, -3.0, 4.0);

		assert_eq!(-tuple1, expected_tuple);
	}
	#[test]
	fn	scalar_tuple_multiplication()
	{
		let tuple1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
		let expected_tuple = Tuple::new(3.5, -7.0, 10.5, -14.0);

		assert_eq!(tuple1 * 3.5, expected_tuple);
		// assert_eq!(3.5 * tuple1, expected_tuple);
		//doesn't work because of order!
	}
	#[test]
	fn	fraction_scalar_tuple_multiplication()
	{
		let tuple1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
		let expected_tuple = Tuple::new(0.5, -1.0, 1.5, -2.0);

		assert_eq!(tuple1 * 0.5, expected_tuple);
	}
	#[test]
	fn	scalar_tuple_division()
	{
		let tuple1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
		let expected_tuple = Tuple::new(0.5, -1.0, 1.5, -2.0);

		assert_eq!(tuple1 / 2.0, expected_tuple);
	}
	#[test]
	fn	computing_magnitude()
	{
		let vec1 = Tuple::vector(1.0, 0.0, 0.0);
		assert_eq!(vec1.magnitude(), 1.0);
		let vec2 = Tuple::vector(0.0, 1.0, 0.0);
		assert_eq!(vec2.magnitude(), 1.0);
		let vec3 = Tuple::vector(0.0, 0.0, 1.0);
		assert_eq!(vec3.magnitude(), 1.0);
		let vec4 = Tuple::vector(1.0, 2.0, 3.0);
		let mag1:f64 = (14.0 as f64).sqrt();
		assert_eq!(vec4.magnitude(), mag1);
		let vec5 = Tuple::vector(-1.0, -2.0, -3.0);
		assert_eq!(vec5.magnitude(), mag1);
	}
	#[test]
	fn	normalizing_vector()
	{
		let vec1 = Tuple::vector(4.0, 0.0, 0.0);
		let expected = Tuple::vector(1.0, 0.0, 0.0);
		assert_eq!(vec1.normalize(), expected);
		let vec2 = Tuple::vector(1.0, 2.0, 3.0);
		let expected2 = Tuple::vector(0.26726, 0.53452, 0.80178);
		assert_eq!(vec2.normalize(), expected2);

		let vec3 = vec2.normalize();
		assert_eq!(vec3.magnitude(), 1.0);
	}
	#[test]
	fn	dot_product_tuples()
	{
		let vec1 = Tuple::vector(1.0, 2.0, 3.0);
		let vec2 = Tuple::vector(2.0, 3.0, 4.0);
		assert_eq!(vec1.dot(&vec2), 20.0);
	}
	#[test]
	fn	cross_product_tuples()
	{
		let vec1 = Tuple::vector(1.0, 2.0, 3.0);
		let vec2 = Tuple::vector(2.0, 3.0, 4.0);
		let expect1 = Tuple::vector(-1.0, 2.0, -1.0);
		let expect2 = Tuple::vector(1.0, -2.0, 1.0);
		assert_eq!(vec1.cross(&vec2), expect1);
		assert_eq!(vec2.cross(&vec1), expect2);
	}
}