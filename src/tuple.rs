use std::ops;

use super::util::*;

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
	// pub fn is_point(&self) -> bool {
    //     self.w == 1.0
    // }
    // pub fn is_vector(&self) -> bool {
    //     self.w == 0.0
    // }
}


impl PartialEq<Tuple> for Tuple {
	fn eq(&self, other: &Self) -> bool {
		return epsil_compare(self.x, other.x)
		&& epsil_compare(self.y, other.y)
		&& epsil_compare(self.z, other.z)
		&& epsil_compare(self.w, other.w);
	}
}

impl Tuple {
	fn magnitude(&self) -> f64{
		(self.x.powi(2) + self.y.powi(2)
		+ self.z.powi(2) + self.w.powi(2)).sqrt()
	}
	fn normalize(&self) -> Tuple{
		*self / self.magnitude()
	}
	fn	dot(&self, other: &Self) -> f64{
		self.x * other.x +
		self.y * other.y +
		self.z * other.z +
		self.w * other.w
	}
	fn	cross(&self, other: &Self) -> Tuple{
		Tuple::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
			0.0,
		)
	}
}

impl ops::Add<Self> for Tuple {
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

impl ops::Sub<Self> for Tuple {
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
impl ops::Mul<f64> for Tuple {
	type Output = Self;
	fn mul(self, other: f64) -> Self{
		Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
		)
	}
}
impl ops::Div<f64> for Tuple {
	type Output = Self;
	fn div(self, other: f64) -> Self{
		self * (1.0 / other)
	}
}

impl ops::Neg for Tuple {
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