
use std::ops;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
	pub x : f64,
	pub y : f64,
	pub z : f64,
	pub w : f64,
}

impl Tuple {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
	pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w : 1.0 }
	}
	pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w : 0.0 }
	}
	pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

pub fn epsil_compare(left : f64, right: f64) -> bool {
	let epsilon = 0.00001;
	(left- right).abs() < epsilon
}

impl PartialEq<Tuple> for Tuple {
	fn eq(&self, other: &Self) -> bool {
		return epsil_compare(self.x, other.x)
		&& epsil_compare(self.y, other.y)
		&& epsil_compare(self.z, other.z)
		&& epsil_compare(self.w, other.w);
	}
}

impl ops::Add<Self> for Tuple {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
	}
}

impl ops::Sub<Self> for Tuple {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self{
		Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
		)
	}
}
impl ops::Mul<f64> for Tuple {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self{
		Tuple::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
		)
	}
}
impl ops::Div<f64> for Tuple {
	type Output = Self;
	fn div(self, rhs: f64) -> Self{
		self * (1.0 / rhs)
	}
}

impl ops::Neg for Tuple {
	type Output = Self;
	fn neg(self) -> Self{
		Tuple::new(
            -self.x,
            -self.y,
            -self.z,
           - self.w,
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
}