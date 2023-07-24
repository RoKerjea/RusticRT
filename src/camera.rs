use crate::F;
use crate::matrix::Matrix;

pub struct Camera {
	pub transform: Matrix<4>,
	pub vsize: usize,
	pub hsize: usize,
	pub field_of_view: F,
}