
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
}