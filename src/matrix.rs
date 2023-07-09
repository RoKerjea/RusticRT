//external crates
use num_traits::Float;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};
//my crates
use crate::fuzzy_eq::*;
use crate::tuple::*;

type Matrix3f = Matrix<f64, 3>;

type Matrix4fArray= [Matrix4fArrayRow; 4];
type Matrix4fArrayRow = [f64; 4];

// @TODO refactor matrix to have one universal type

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const D: usize>
where
	T: Float,
{
	data: [[T; D]; D],
}

impl<T, const D: usize> From<[[T; D]; D]> for Matrix<T, D>
where
	T: Float,
{
	fn from(data: [[T; D]; D]) -> Self {
		Matrix { data }
	}
}

impl<T, const D: usize> Matrix<T, D>
where
	T: Float,
{
	pub fn new() -> Matrix<T, D> {
		Matrix::from([[T::zero(); D]; D])
	}
}

impl<T, const D: usize> Index<usize> for Matrix<T, D>
where
	T: Float,
{
	type Output = [T; D];

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl<T, const D: usize> IndexMut<usize> for Matrix<T, D>
where
	T: Float,
{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

impl<T, const D: usize> FuzzyEq<Self> for Matrix<T, D>
where
	T: Float,
	T: FuzzyEq<T>,
{
	fn fuzzy_eq(&self, other: &Self) -> bool {
		for row in 0..D {
			for col in 0..D{
				if !self[row][col].fuzzy_eq(&other[row][col])
				{
					return false;
				}
			}
		}
		return true;
	}
}

impl<T> Matrix<T, 2>
where
	T: Float,
{
	pub	fn determinant(&self) -> T {
		self[0][0] * self[1][1] - self[0][1] * self[1][0]
	}
}

impl<T> Matrix<T, 3>
where
	T: Float,
{
	//@TODO Big refacto here, it's actually worse than what I improvised in C
	//for MiniRT, and that was already not very pretty
	// pub fn	submatrix(&self, row: usize, col: usize) -> Matrix{
	// 	let mut res = Matrix::new();
	// 	let [mut self_row, mut self_col, mut sub_row, mut sub_col] = [0; 4];
	// 	while sub_row < 2
	// 	{
	// 		while sub_col < 2
	// 		{
	// 			if self_col != col && self_row != row
	// 			{
	// 				res[sub_row][sub_col]= self[self_row][self_col];
	// 				sub_col += 1;
	// 			}
	// 			self_col += 1;
	// 		}
	// 		self_row += 1;
	// 		self_col = 0;
	// 		if sub_row != row {
	// 			sub_row += 1;
	// 		}
	// 		sub_col = 0;
	// 	}
	// 	res
	// }
	//Absolutly dreadful, very excited to be able to refacto that
	//still can't deviate too much from example
	pub fn submatrix(&self, row: usize, column: usize) -> Matrix<T, 2>
	{
		let mut m: Matrix<T, 2> = Matrix::new();
		let [mut self_row, mut self_col, mut sub_row, mut sub_col] = [0; 4];
		while sub_row < 2 {
			if self_row == row {
			// Skip row to be removed
				self_row += 1;
			}
			while sub_col < 2 {
				if self_col == column {
				// Skip column to be removed
					self_col += 1;
				}
				m[sub_row][sub_col] = self[self_row][self_col];

				self_col += 1;
				sub_col += 1;
			}
			self_row += 1;
			self_col = 0;
			sub_row += 1;
			sub_col = 0;
		}
		return m;
	}
	pub fn minor(&self, row: usize, column: usize) -> T {
		self.submatrix(row, column).determinant()
	}
	pub	fn cofactor(&self, row: usize, column: usize) -> T {
		let res = self.minor(row, column);
		if (row + column) % 2 != 0{
			return -res;
		}
		return res;
	}
	pub fn determinant(&self) -> T {
		return 	self[0][0] * self.cofactor(0, 0)
				+ self[0][1] * self.cofactor(0, 1)
				+ self[0][2] * self.cofactor(0, 2);
	}
}
//////////////////////////////////////////////////////////

// #[derive(Debug, Copy, Clone)]
// pub struct Matrix3f{
// 	data: Matrix3fArray,
// }

// impl From<Matrix3fArray> for Matrix3f {
// 	fn from(data: Matrix3fArray) -> Self {
// 		Matrix3f { data }
// 	}
// }

// impl Index<usize> for Matrix3f{
// 	type Output = Matrix3fArrayRow;

// 	fn index(&self, index: usize) -> &Self::Output {
// 		&self.data[index]
// 	}
// }

// impl IndexMut<usize> for Matrix3f{
// 	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
// 		&mut self.data[index]
// 	}
// }

impl Matrix3f {
	// pub fn new() -> Matrix3f {
	// 	Matrix3f::from([
	// 		[0.0, 0.0, 0.0],
	// 		[0.0, 0.0, 0.0],
	// 		[0.0, 0.0, 0.0],])
	// }
	
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4f{
	data: Matrix4fArray,
}

impl From<Matrix4fArray> for Matrix4f {
	fn from(data: Matrix4fArray) -> Self {
		Matrix4f { data }
	}
}

impl Index<usize> for Matrix4f{
	type Output = Matrix4fArrayRow;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl IndexMut<usize> for Matrix4f{
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

impl Matrix4f {
	pub fn new() -> Matrix4f {
		Matrix4f::from([
			[0.0, 0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0, 0.0],])
	}
	pub fn identity() -> Matrix4f {
		Matrix4f::from([
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],])
	}
	pub fn transpose(&self) -> Matrix4f {
		let mut m = Matrix4f::new();
		for row in 0..4 {
			for col in 0..4{
				m[col][row] = self[row][col];
			}
		}
		m
	}
	pub fn submatrix(&self, row: usize, column: usize) -> Matrix3f {
		let mut m = Matrix3f::new();
		let [mut self_row, mut self_col, mut sub_row, mut sub_col] = [0; 4];
		while sub_row < 3 {
			if self_row == row {
			// Skip row to be removed
				self_row += 1;
			}
			while sub_col < 3 {
				if self_col == column {
				// Skip column to be removed
					self_col += 1;
				}
				m[sub_row][sub_col] = self[self_row][self_col];

				self_col += 1;
				sub_col += 1;
			}
			self_row += 1;
			self_col = 0;
			sub_row += 1;
			sub_col = 0;
		}
		return m;
	}
	pub fn minor(&self, row: usize, column: usize) -> f64 {
		self.submatrix(row, column).determinant()
	}
	pub	fn cofactor(&self, row: usize, column: usize) -> f64 {
		let mut res = self.minor(row, column);
		if (row + column) % 2 == 1{
			res *= -1.0;
		}
		res
	}
	pub fn determinant(&self) -> f64 {
		return 	self[0][0] * self.cofactor(0, 0)
				+ self[0][1] * self.cofactor(0, 1)
				+ self[0][2] * self.cofactor(0, 2)
				+ self[0][3] * self.cofactor(0, 3);

	}
	pub	fn	invertible(&self) -> bool {
		if self.determinant() == 0.0{
			return false;
		}
		return true;
	}
	pub fn	inverse(&self) -> Self {
		let mut resmatrix = Matrix4f::new();
		for row in 0..4 {
			for col in 0..4 {
				resmatrix[col][row] = self.cofactor(row, col) / self.determinant();
			}
		}
		resmatrix
	}
}

//TODO: implement this operation as a '==' oveload
//it's totally possible to do that and would be much better to use
//not doing that for now because deviating too far from example is risky
impl FuzzyEq<Matrix4f> for Matrix4f {
	fn fuzzy_eq(&self, other: &Matrix4f) -> bool {
		self[0][0].fuzzy_eq(&other[0][0])
		&& self[0][1].fuzzy_eq(&other[0][1])
		&& self[0][2].fuzzy_eq(&other[0][2])
		&& self[0][3].fuzzy_eq(&other[0][3])
		&& self[1][0].fuzzy_eq(&other[1][0])
		&& self[1][1].fuzzy_eq(&other[1][1])
		&& self[1][2].fuzzy_eq(&other[1][2])
		&& self[1][3].fuzzy_eq(&other[1][3])
		&& self[2][0].fuzzy_eq(&other[2][0])
		&& self[2][1].fuzzy_eq(&other[2][1])
		&& self[2][2].fuzzy_eq(&other[2][2])
		&& self[2][3].fuzzy_eq(&other[2][3])
		&& self[3][0].fuzzy_eq(&other[3][0])
		&& self[3][1].fuzzy_eq(&other[3][1])
		&& self[3][2].fuzzy_eq(&other[3][2])
		&& self[3][3].fuzzy_eq(&other[3][3])
	}
}

impl Mul<Matrix4f> for Matrix4f{
	type Output = Matrix4f;
	fn mul(self, other: Matrix4f) -> Self::Output{
		let mut m = Matrix4f::new();
		for row in 0..4 {
			for column in 0..4  {
				m[row][column]	= self[row][0] * other[0][column]
								+ self[row][1] * other[1][column]
								+ self[row][2] * other[2][column]
								+ self[row][3] * other[3][column];
			}
		}
		m
	}
}

impl Mul<Tuple> for Matrix4f{
	type Output = Tuple;
	fn mul(self, other: Tuple) -> Tuple {
		Tuple::new(
			self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3] * other.w,
			self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3] * other.w,
			self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3] * other.w,
			self[3][0] * other.x + self[3][1] * other.y + self[3][2] * other.z + self[3][3] * other.w,
		)
	}
}

// impl FuzzyEq<Matrix3f> for Matrix3f {
// 	fn fuzzy_eq(&self, other: &Matrix3f) -> bool {
// 	 	self[0][0].fuzzy_eq(&other[0][0])
// 		&& self[0][1].fuzzy_eq(&other[0][1])
// 		&& self[0][2].fuzzy_eq(&other[0][2])
// 		&& self[1][0].fuzzy_eq(&other[1][0])
// 		&& self[1][1].fuzzy_eq(&other[1][1])
// 		&& self[1][2].fuzzy_eq(&other[1][2])
// 		&& self[2][0].fuzzy_eq(&other[2][0])
// 		&& self[2][1].fuzzy_eq(&other[2][1])
// 		&& self[2][2].fuzzy_eq(&other[2][2])
// 	}
// }


#[cfg(test)]
mod tests{
	use crate::tuple::Tuple;

// use std::result;
	use super::*;
	#[test]
	fn	constructing_a_4fmatrix()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		assert_eq!(matrix1[0][0], 1.0);
		assert_eq!(matrix1[0][3], 4.0);
		assert_eq!(matrix1[1][0], 5.5);
		assert_eq!(matrix1[1][2], 7.5);
		assert_eq!(matrix1[2][2], 11.0);
		assert_eq!(matrix1[3][0], 13.5);
		assert_eq!(matrix1[3][2], 15.5);
	}
	#[test]
	fn	constructing_a_2fmatrix()
	{
		let matrix1 = Matrix::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		assert_eq!(matrix1[0][0], -3.0);
		assert_eq!(matrix1[0][1], 5.0);
		assert_eq!(matrix1[1][0], 1.0);
		assert_eq!(matrix1[1][1], -2.0);
	}
	#[test]
	fn	constructing_a_3fmatrix()
	{
		let matrix1 = Matrix3f::from([
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],]);
		assert_eq!(matrix1[0][0], -3.0);
		assert_eq!(matrix1[1][1], -2.0);
		assert_eq!(matrix1[2][2], 1.0);
	}
	#[test]
	fn	comparing_4fmatrixes_true()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		let matrix2 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_4fmatrixes_false()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		let matrix2 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 1.5],]);
		assert!(!matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_3fmatrixes_true(){
		let matrix1 = Matrix3f::from([
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],]);
		let matrix2 = Matrix3f::from([
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],]);
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_2fmatrixes_true(){
		let matrix1 = Matrix::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		let matrix2 = Matrix::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	multiplying_4f_matrixes()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 8.0, 7.0, 6.0],
			[5.0, 4.0, 3.0, 2.0],]);
		let matrix2 = Matrix4f::from([
			[-2.0, 1.0, 2.0, 3.0],
			[3.0, 2.0, 1.0, -1.0],
			[4.0, 3.0, 6.0, 5.0],
			[1.0, 2.0, 7.0, 8.0],]);
		let expected = Matrix4f::from([
			[20.0, 22.0, 50.0, 48.0],
			[44.0, 54.0, 114.0, 108.0],
			[40.0, 58.0, 110.0, 102.0],
			[16.0, 26.0, 46.0, 42.0],]);
		let actual_result: Matrix4f = matrix1 * matrix2;
		assert!(actual_result.fuzzy_eq(&expected));
	}
	#[test]
	fn	multiplying_4f_matrixes_by_identity_matrix()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 8.0, 7.0, 6.0],
			[5.0, 4.0, 3.0, 2.0],]);
		let identitym = Matrix4f::identity();

		let actual_res = matrix1 * identitym;
		assert!(actual_res.fuzzy_eq(&matrix1));
	}
	#[test]
	fn	multiplying_4f_matrixes_by_tuple()
	{
		let matrix1 = Matrix4f::from([
			[1.0, 2.0, 3.0, 4.0],
			[2.0, 4.0, 4.0, 2.0],
			[8.0, 6.0, 4.0, 1.0],
			[0.0, 0.0, 0.0, 1.0],]);
		let tuplearg = Tuple::point(1.0, 2.0, 3.0);

		let actual_res:Tuple = matrix1 * tuplearg;
		let expected = Tuple::point(18.0, 24.0, 33.0);
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn	transposing_4f_matrix()
	{
		let matrix1 = Matrix4f::from([
			[0.0, 9.0, 3.0, 0.0],
			[9.0, 8.0, 0.0, 8.0],
			[1.0, 8.0, 5.0, 3.0],
			[0.0, 0.0, 5.0, 8.0],]);
		let expected = Matrix4f::from([
			[0.0, 9.0, 1.0, 0.0],
			[9.0, 8.0, 8.0, 0.0],
			[3.0, 0.0, 5.0, 5.0],
			[0.0, 8.0, 3.0, 8.0],]);
		let actual_res = matrix1.transpose();
		assert!(actual_res.fuzzy_eq(&expected));
	}
	#[test]
	fn	determinant_of_a_2f_matrix()
	{
		let mat1 = Matrix::from([
			[1.0, 5.0],
			[-3.0, 2.0],]);
		let expected_res = 17.0;
		assert_eq!(mat1.determinant(), expected_res);
	}
	#[test]
	fn	submatrix_of_matrix3f()
	{
		let matrix1 = Matrix3f::from([
			[1.0, 5.0, 0.0],
			[-3.0, 2.0, 7.0],
			[0.0, 6.0, -3.0],]);
		let expected_res = Matrix::from([
			[-3.0, 2.0],
			[0.0, 6.0],]);
		let	actual_res = matrix1.submatrix(0, 2);
		assert!(actual_res.fuzzy_eq(&expected_res));
	}
	#[test]
	fn	submatrix_of_matrix4f()
	{
		let matrix1 = Matrix4f::from([
			[-6.0, 1.0, 1.0, 6.0],
			[-8.0, 5.0, 8.0, 6.0],
			[-1.0, 0.0, 8.0, 2.0],
			[-7.0, 1.0, -1.0, 1.0],]);
		let expected_res = Matrix3f::from([
			[-6.0, 1.0, 6.0],
			[-8.0, 8.0, 6.0],
			[-7.0, -1.0, 1.0],]);
		let	actual_res = matrix1.submatrix(2, 1);
		assert!(actual_res.fuzzy_eq(&expected_res));
	}
	#[test]
	fn	minor_of_matrix3f()
	{
		let matrix1 = Matrix3f::from([
			[3.0, 5.0, 0.0],
			[2.0, -1.0, -7.0],
			[6.0, -1.0, 5.0],]);
		let expected_res = 25.0;
		let actual_res = matrix1.minor(1, 0);
		assert_eq!(expected_res, actual_res);
	}
	#[test]
	fn	cofactor_of_matrix3f(){
		let matrix1 = Matrix3f::from([
			[3.0, 5.0, 0.0],
			[2.0, -1.0, -7.0],
			[6.0, -1.0, 5.0],]);
		assert_eq!(matrix1.minor(0, 0), -12.0);
		assert_eq!(matrix1.cofactor(0, 0), -12.0);
		assert_eq!(matrix1.minor(1,0), 25.0);
		assert_eq!(matrix1.cofactor(1,0), -25.0);
	}
	#[test]
	fn	determinant_of_matrix3f(){
		let matrix1 = Matrix3f::from([
			[1.0, 2.0, 6.0],
			[-5.0, 8.0, -4.0],
			[2.0, 6.0, 4.0],]);
		assert_eq!(matrix1.cofactor(0, 0), 56.0);
		assert_eq!(matrix1.cofactor(0, 1), 12.0);
		assert_eq!(matrix1.cofactor(0, 2), -46.0);
		assert_eq!(matrix1.determinant(), -196.0);
	}
	#[test]
	fn	determinant_of_matrix4f(){
		let matrix1 = Matrix4f::from([
			[-2.0, -8.0, 3.0, 5.0],
			[-3.0, 1.0, 7.0, 3.0],
			[1.0, 2.0, -9.0, 6.0],
			[-6.0, 7.0, 7.0, -9.0],]);
		assert_eq!(matrix1.cofactor(0, 0), 690.0);
		assert_eq!(matrix1.cofactor(0, 1), 447.0);
		assert_eq!(matrix1.cofactor(0, 2), 210.0);
		assert_eq!(matrix1.cofactor(0, 3), 51.0);
		// assert_eq!(matrix1.determinant(), -196.0);
	}
	#[test]
	fn	matrix4f_invertibility(){
		let matrix1 = Matrix4f::from([
			[6.0, 4.0, 4.0, 4.0],
			[5.0, 5.0, 7.0, 6.0],
			[4.0, -9.0, 3.0, -7.0],
			[9.0, 1.0, 7.0, -6.0],]);
		let matrix2 = Matrix4f::from([
			[-4.0, 2.0, -2.0, -3.0],
			[9.0, 6.0, 2.0, 6.0],
			[0.0, -5.0, 1.0, -5.0],
			[0.0, 0.0, 0.0, 0.0],]);
		assert!(matrix1.invertible());
		assert!(!matrix2.invertible());
	}
	#[test]
	fn	matrix4f_inversion() {
		let matrix1 = Matrix4f::from([
			[-5.0, 2.0, 6.0, -8.0],
			[1.0, -5.0, 1.0, 8.0],
			[7.0, 7.0, -6.0, -7.0],
			[1.0, -3.0, 7.0, 4.0],]);
		let expected_res = Matrix4f::from([
			[0.21805, 0.45113, 0.24060, -0.04511],
			[-0.80827, -1.45677, -0.44361, 0.52068],
			[-0.07895, -0.22368, -0.05263, 0.19737],
			[-0.52256, -0.81391, -0.30075, 0.30639],]);
		let	actual_res = matrix1.inverse();
		assert_eq!(matrix1.cofactor(2, 3), -160.0);
		assert_eq!(matrix1.cofactor(3, 2), 105.0);
		assert_eq!(matrix1.determinant(), 532.0);
		assert!(actual_res.fuzzy_eq(&expected_res));
	}
	#[test]
	fn	matrix4f_inversion2() {
		let matrix1 = Matrix4f::from([
			[8.0, -5.0, 9.0, 2.0],
			[7.0, 5.0, 6.0, 1.0],
			[-6.0, 0.0, 9.0, 6.0],
			[-3.0, 0.0, -9.0, -4.0],]);
		let expected_res = Matrix4f::from([
			[-0.15385, -0.15385, -0.28205, -0.53846],
			[-0.07692, 0.12308, 0.02564, 0.03077],
			[0.35897, 0.35897, 0.43590, 0.92308],
			[-0.69231, -0.69231, -0.76923, -1.92308],]);
		let	actual_res = matrix1.inverse();
		assert!(actual_res.fuzzy_eq(&expected_res));
	}
	#[test]
	fn	matrix4f_inversion3() {
		let matrix1 = Matrix4f::from([
			[9.0, 3.0, 0.0, 9.0],
			[-5.0, -2.0, -6.0, -3.0],
			[-4.0, 9.0, 6.0, 4.0],
			[-7.0, 6.0, 6.0, 2.0],]);
		let expected_res = Matrix4f::from([
			[-0.04074, -0.07778, 0.14444, -0.22222],
			[-0.07778, 0.03333, 0.36667, -0.33333],
			[-0.02901, -0.14630, -0.10926, 0.12963],
			[0.17778, 0.06667, -0.26667, 0.33333],]);
		let	actual_res = matrix1.inverse();
		assert!(actual_res.fuzzy_eq(&expected_res));
	}
	#[test]
	fn	matrix4f_inversion4() {
		let matrix1 = Matrix4f::from([
			[3.0, -9.0, 7.0, 3.0],
			[3.0, -8.0, 2.0, -9.0],
			[-4.0, 4.0, 4.0, 1.0],
			[-6.0, 5.0, -1.0, 1.0],]);
		let matrix2 = Matrix4f::from([
			[8.0, 2.0, 2.0, 2.0],
			[3.0, -1.0, 7.0, 0.0],
			[7.0, 0.0, 5.0, 4.0],
			[6.0, -2.0, 0.0, 5.0],]);
		let	actual_res = matrix1 * matrix2;
		let expected_res = actual_res * matrix2.inverse();
		assert!(expected_res.fuzzy_eq(&matrix1));
	}
	// #[test]
	// fn	applying_translation_matrix_to_point()
	// {
	// 	let matrix_translate = Matrix4f::translation(5.0, -3.0, 2.0);
	// 	let p = Tuple::point(-3.0, 4.0, 5.0);
	// 	let expected = Tuple::point(2.0, 1.0, 7.0);
	// }
}