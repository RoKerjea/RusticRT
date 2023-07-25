//external crates

use crate::F;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};
//my crates
use crate::fuzzy_eq::*;
use crate::tuple::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix< const D: usize>
{
	data: [[F; D]; D],
}

impl<const D: usize> From<[[F; D]; D]> for Matrix<D>
{
	fn from(data: [[F; D]; D]) -> Self {
		Matrix { data }
	}
}

//Generic universal implementation for all matrixes

impl<const D: usize> Matrix<D> {
	pub fn new() -> Matrix<D> {
	  Matrix::from([[0.0; D]; D])
	}
  
	pub fn diagonal(value: F) -> Matrix<D> {
	  let mut matrix = Matrix::new();
	  for i in 0..D {
		matrix[i][i] = value;
	  }
	  matrix
	}
  
	pub fn identity() -> Matrix<D> {
	  Matrix::diagonal(1.0)
	}
  
	pub fn transpose(&self) -> Matrix<D> {
	  let mut matrix = Matrix::new();
	  for row in 0..D {
		for column in 0..D {
		  matrix[column][row] = self.data[row][column];
		}
	  }
	  matrix
	}
  }

impl<const D: usize> Mul<Self> for Matrix<D>
{
	type Output = Self;
	fn mul(self, other: Self) -> Self::Output{
		let mut res:Matrix<D> = Matrix::new();
		for row in 0..D {
			for column in 0..D  {
				for i in 0..D {
					res[row][column] += self[row][i] * other[i][column];
				}
			}
		}
		res
	}
}
impl<const D: usize> Index<usize> for Matrix<D> {
	type Output = [F; D];
  
	fn index(&self, index: usize) -> &Self::Output {
	  &self.data[index]
	}
  }

  impl<const D: usize> IndexMut<usize> for Matrix<D> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
	  &mut self.data[index]
	}
  }

//TODO: implement this operation as a '==' oveload
//it's totally possible to do that and would be much better to use
//not doing that for now because deviating too far from example is risky
impl<const D: usize> FuzzyEq<Self> for Matrix<D>
{
	fn fuzzy_eq(&self, other: Self) -> bool {
		for row in 0..D {
			for col in 0..D{
				if !self[row][col].fuzzy_eq(other[row][col])
				{
					return false;
				}
			}
		}
		return true;
	}
}

impl Matrix<2>
{
	pub	fn determinant(&self) -> F {
		self[0][0] * self[1][1] - self[0][1] * self[1][0]
	}
}

impl Matrix<3>
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
	pub fn submatrix(&self, row: usize, column: usize) -> Matrix<2>
	{
		let mut m: Matrix<2> = Matrix::new();
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
	pub fn minor(&self, row: usize, column: usize) -> F {
		self.submatrix(row, column).determinant()
	}
	pub	fn cofactor(&self, row: usize, column: usize) -> F {
		let res = self.minor(row, column);
		if (row + column) % 2 != 0{
			return -res;
		}
		return res;
	}
	pub fn determinant(&self) -> F {
		return 	self[0][0] * self.cofactor(0, 0)
				+ self[0][1] * self.cofactor(0, 1)
				+ self[0][2] * self.cofactor(0, 2);
	}
}

impl Matrix<4>
{
	pub fn submatrix(&self, row: usize, column: usize) -> Matrix<3> {
		let mut m = Matrix::new();
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
	pub fn minor(&self, row: usize, column: usize) -> F {
		self.submatrix(row, column).determinant()
	}
	pub	fn cofactor(&self, row: usize, column: usize) -> F {
		let res = self.minor(row, column);
		if (row + column) % 2 != 0{
			return -res;
		}
		return res;
	}
	pub fn determinant(&self) -> F {
		return 	self[0][0] * self.cofactor(0, 0)
				+ self[0][1] * self.cofactor(0, 1)
				+ self[0][2] * self.cofactor(0, 2)
				+ self[0][3] * self.cofactor(0, 3);

	}
	pub	fn	invertible(&self) -> bool {
		if self.determinant() == 0.0 {
			return false;
		}
		return true;
	}
	pub fn	inverse(&self) -> Self {
		let mut resmatrix = Matrix::new();
		let	determinant = self.determinant();
		for row in 0..4 {
			for col in 0..4 {
				resmatrix[col][row] = self.cofactor(row, col) / determinant;
			}
		}
		resmatrix
	}
	pub fn	translation(x: F, y: F, z: F) -> Self{
		let mut ident = Matrix::identity();
		ident[0][3] = x;
		ident[1][3] = y;
		ident[2][3] = z;
		ident
	}
	pub fn	scaling(x: F, y: F, z: F) -> Self{
		let mut ident = Matrix::identity();
		ident[0][0] = x;
		ident[1][1] = y;
		ident[2][2] = z;
		ident
	}
	pub fn	rotation_x(r: F) -> Self{
		let mut ident = Matrix::identity();
		ident[1][1] = r.cos();
		ident[1][2] = -r.sin();
		ident[2][1] = r.sin();
		ident[2][2] = r.cos();
		ident
	}
	pub fn	rotation_y(r: F) -> Self{
		let mut ident = Matrix::identity();
		ident[0][0] = r.cos();
		ident[0][2] = r.sin();
		ident[2][0] = -r.sin();
		ident[2][2] = r.cos();
		ident
	}
	pub fn	rotation_z(r: F) -> Self{
		let mut ident = Matrix::identity();
		ident[0][0] = r.cos();
		ident[0][1] = -r.sin();
		ident[1][0] = r.sin();
		ident[1][1] = r.cos();
		ident
	}
	pub fn	shearing(xy: F, xz: F, yx: F, yz: F, zx: F, zy: F) -> Self{
		let mut ident = Matrix::identity();
		ident[0][1] = xy;
		ident[0][2] = xz;
		ident[1][0] = yx;
		ident[1][2] = yz;
		ident[2][0] = zx;
		ident[2][1] = zy;
		ident
	}
}

impl Mul<Tuple> for Matrix<4>
{
	type Output = Tuple;
	fn mul(self, other: Tuple) -> Self::Output {
		Tuple::new(
			self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3] * other.w,
			self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3] * other.w,
			self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3] * other.w,
			self[3][0] * other.x + self[3][1] * other.y + self[3][2] * other.z + self[3][3] * other.w,
		)
	}
}

#[cfg(test)]
mod tests{
	use crate::tuple::Tuple;
	use std::f64::consts::PI;
	use num_traits::Float;
	use super::*;
	#[test]
	fn	constructing_a_4fmatrix()
	{
		let matrix1 = Matrix::from([
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
		let matrix1 = Matrix::from([
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
		let matrix1 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		let matrix2 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		assert!(matrix1.fuzzy_eq(matrix2));
	}
	#[test]
	fn	comparing_4fmatrixes_false()
	{
		let matrix1 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],]);
		let matrix2 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 1.5],]);
		assert!(!matrix1.fuzzy_eq(matrix2));
	}
	#[test]
	fn	comparing_3fmatrixes_true(){
		let matrix1 = Matrix::from([
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],]);
		let matrix2 = Matrix::from([
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],]);
		assert!(matrix1.fuzzy_eq(matrix2));
	}
	#[test]
	fn	comparing_2fmatrixes_true(){
		let matrix1 = Matrix::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		let matrix2 = Matrix::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		assert!(matrix1.fuzzy_eq(matrix2));
	}
	#[test]
	fn	multiplying_4f_matrixes()
	{
		let matrix1 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 8.0, 7.0, 6.0],
			[5.0, 4.0, 3.0, 2.0],]);
		let matrix2 = Matrix::from([
			[-2.0, 1.0, 2.0, 3.0],
			[3.0, 2.0, 1.0, -1.0],
			[4.0, 3.0, 6.0, 5.0],
			[1.0, 2.0, 7.0, 8.0],]);
		let expected = Matrix::from([
			[20.0, 22.0, 50.0, 48.0],
			[44.0, 54.0, 114.0, 108.0],
			[40.0, 58.0, 110.0, 102.0],
			[16.0, 26.0, 46.0, 42.0],]);
		let actual_result = matrix1 * matrix2;
		assert!(actual_result.fuzzy_eq(expected));
	}
	#[test]
	fn	multiplying_4f_matrixes_by_identity_matrix()
	{
		let matrix1 = Matrix::from([
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 8.0, 7.0, 6.0],
			[5.0, 4.0, 3.0, 2.0],]);
		let identitym = Matrix::identity();

		let actual_res = matrix1 * identitym;
		assert!(actual_res.fuzzy_eq(matrix1));
	}
	#[test]
	fn	multiplying_4f_matrixes_by_tuple()
	{
		let matrix1 = Matrix::from([
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
		let matrix1 = Matrix::from([
			[0.0, 9.0, 3.0, 0.0],
			[9.0, 8.0, 0.0, 8.0],
			[1.0, 8.0, 5.0, 3.0],
			[0.0, 0.0, 5.0, 8.0],]);
		let expected = Matrix::from([
			[0.0, 9.0, 1.0, 0.0],
			[9.0, 8.0, 8.0, 0.0],
			[3.0, 0.0, 5.0, 5.0],
			[0.0, 8.0, 3.0, 8.0],]);
		let actual_res = matrix1.transpose();
		assert!(actual_res.fuzzy_eq(expected));
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
		let matrix1 = Matrix::from([
			[1.0, 5.0, 0.0],
			[-3.0, 2.0, 7.0],
			[0.0, 6.0, -3.0],]);
		let expected_res = Matrix::from([
			[-3.0, 2.0],
			[0.0, 6.0],]);
		let	actual_res = matrix1.submatrix(0, 2);
		assert!(actual_res.fuzzy_eq(expected_res));
	}
	#[test]
	fn	submatrix_of_matrix4f()
	{
		let matrix1 = Matrix::from([
			[-6.0, 1.0, 1.0, 6.0],
			[-8.0, 5.0, 8.0, 6.0],
			[-1.0, 0.0, 8.0, 2.0],
			[-7.0, 1.0, -1.0, 1.0],]);
		let expected_res = Matrix::from([
			[-6.0, 1.0, 6.0],
			[-8.0, 8.0, 6.0],
			[-7.0, -1.0, 1.0],]);
		let	actual_res = matrix1.submatrix(2, 1);
		assert!(actual_res.fuzzy_eq(expected_res));
	}
	#[test]
	fn	minor_of_matrix3f()
	{
		let matrix1 = Matrix::from([
			[3.0, 5.0, 0.0],
			[2.0, -1.0, -7.0],
			[6.0, -1.0, 5.0],]);
		let expected_res = 25.0;
		let actual_res = matrix1.minor(1, 0);
		assert_eq!(expected_res, actual_res);
	}
	#[test]
	fn	cofactor_of_matrix3f(){
		let matrix1 = Matrix::from([
			[3.0, 5.0, 0.0],
			[2.0, -1.0, -7.0],
			[6.0, -1.0, 5.0],]);
		assert_eq!(matrix1.minor(0, 0), -12.0);
		assert_eq!(matrix1.cofactor(0, 0), -12.0);
		assert_eq!(matrix1.minor(1,0), 25.0);
		assert_eq!(matrix1.cofactor(1,0), -25.0);
	}
	#[test]
	fn	determinant_of_matrix43(){
		let matrix1 = Matrix::from([
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
		let matrix1 = Matrix::from([
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
		let matrix1 = Matrix::from([
			[6.0, 4.0, 4.0, 4.0],
			[5.0, 5.0, 7.0, 6.0],
			[4.0, -9.0, 3.0, -7.0],
			[9.0, 1.0, 7.0, -6.0],]);
		let matrix2 = Matrix::from([
			[-4.0, 2.0, -2.0, -3.0],
			[9.0, 6.0, 2.0, 6.0],
			[0.0, -5.0, 1.0, -5.0],
			[0.0, 0.0, 0.0, 0.0],]);
		assert!(matrix1.invertible());
		assert!(!matrix2.invertible());
	}
	#[test]
	fn	matrix4f_inversion() {
		let matrix1 = Matrix::from([
			[-5.0, 2.0, 6.0, -8.0],
			[1.0, -5.0, 1.0, 8.0],
			[7.0, 7.0, -6.0, -7.0],
			[1.0, -3.0, 7.0, 4.0],]);
		let expected_res = Matrix::from([
			[0.21805, 0.45113, 0.24060, -0.04511],
			[-0.80827, -1.45677, -0.44361, 0.52068],
			[-0.07895, -0.22368, -0.05263, 0.19737],
			[-0.52256, -0.81391, -0.30075, 0.30639],]);
		let	actual_res = matrix1.inverse();
		assert_eq!(matrix1.cofactor(2, 3), -160.0);
		assert_eq!(matrix1.cofactor(3, 2), 105.0);
		assert_eq!(matrix1.determinant(), 532.0);
		assert!(actual_res.fuzzy_eq(expected_res));
	}
	#[test]
	fn	matrix4f_inversion2() {
		let matrix1 = Matrix::from([
			[8.0, -5.0, 9.0, 2.0],
			[7.0, 5.0, 6.0, 1.0],
			[-6.0, 0.0, 9.0, 6.0],
			[-3.0, 0.0, -9.0, -4.0],]);
		let expected_res = Matrix::from([
			[-0.15385, -0.15385, -0.28205, -0.53846],
			[-0.07692, 0.12308, 0.02564, 0.03077],
			[0.35897, 0.35897, 0.43590, 0.92308],
			[-0.69231, -0.69231, -0.76923, -1.92308],]);
		let	actual_res = matrix1.inverse();
		assert!(actual_res.fuzzy_eq(expected_res));
	}
	#[test]
	fn	matrix4f_inversion3() {
		let matrix1 = Matrix::from([
			[9.0, 3.0, 0.0, 9.0],
			[-5.0, -2.0, -6.0, -3.0],
			[-4.0, 9.0, 6.0, 4.0],
			[-7.0, 6.0, 6.0, 2.0],]);
		let expected_res = Matrix::from([
			[-0.04074, -0.07778, 0.14444, -0.22222],
			[-0.07778, 0.03333, 0.36667, -0.33333],
			[-0.02901, -0.14630, -0.10926, 0.12963],
			[0.17778, 0.06667, -0.26667, 0.33333],]);
		let	actual_res = matrix1.inverse();
		assert!(actual_res.fuzzy_eq(expected_res));
	}
	#[test]
	fn	matrix4f_inversion4() {
		let matrix1 = Matrix::from([
			[3.0, -9.0, 7.0, 3.0],
			[3.0, -8.0, 2.0, -9.0],
			[-4.0, 4.0, 4.0, 1.0],
			[-6.0, 5.0, -1.0, 1.0],]);
		let matrix2 = Matrix::from([
			[8.0, 2.0, 2.0, 2.0],
			[3.0, -1.0, 7.0, 0.0],
			[7.0, 0.0, 5.0, 4.0],
			[6.0, -2.0, 0.0, 5.0],]);
		let	actual_res = matrix1 * matrix2;
		let expected_res = actual_res * matrix2.inverse();
		assert!(expected_res.fuzzy_eq(matrix1));
	}
	#[test]
	fn	applying_translation_matrix_to_point()
	{
		let matrix_translate = Matrix::translation(5.0, -3.0, 2.0);
		let p = Tuple::point(-3.0, 4.0, 5.0);
		let expected = Tuple::point(2.0, 1.0, 7.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn	applying_inverse_translation_matrix_to_point()
	{
		let matrix_translate = Matrix::translation(5.0, -3.0, 2.0);
		let p = Tuple::point(-3.0, 4.0, 5.0);
		let expected = Tuple::point(-8.0, 7.0, 3.0);
		let actual_res = matrix_translate.inverse() * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn	translation_doesnt_work_on_vectors()
	{
		let matrix_translate = Matrix::translation(5.0, -3.0, 2.0);
		let p = Tuple::vector(-3.0, 4.0, 5.0);
		let expected = Tuple::vector(-3.0, 4.0, 5.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn scaling_a_point()
	{
		let matrix_translate = Matrix::scaling(2.0, 3.0, 4.0);
		let p = Tuple::point(-4.0, 6.0, 8.0);
		let expected = Tuple::point(-8.0, 18.0, 32.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn scaling_a_vector()
	{
		let matrix_translate = Matrix::scaling(2.0, 3.0, 4.0);
		let p = Tuple::vector(-4.0, 6.0, 8.0);
		let expected = Tuple::vector(-8.0, 18.0, 32.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn	applying_inverse_scaling_matrix_to_vector()
	{
		let matrix_translate = Matrix::scaling(2.0, 3.0, 4.0);
		let p = Tuple::vector(-4.0, 6.0, 8.0);
		let expected = Tuple::vector(-2.0, 2.0, 2.0);
		let actual_res = matrix_translate.inverse() * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn reflection_is_scaling_by_negative()
	{
		let matrix_translate = Matrix::scaling(-1.0, 1.0, 1.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(-2.0, 3.0, 4.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn rotating_a_point_around_x_axis()
	{
		let half_quarter_matrix = Matrix::rotation_x(PI / 4.0);
		let full_quarter_matrix = Matrix::rotation_x(PI / 2.0);
		let p = Tuple::point(0.0, 1.0, 0.0);
		let half_quarter_res = half_quarter_matrix * p;
		let full_quarter_res = full_quarter_matrix * p;

		let half_quarter = Tuple::point(0.0, (2.0).sqrt() / 2.0, (2.0).sqrt() / 2.0);
		let full_quarter = Tuple::point(0.0, 0.0, 1.0);
		assert_eq!(half_quarter_res, half_quarter);
		assert_eq!(full_quarter_res, full_quarter);
	}
	#[test]
	fn rotating_a_point_around_y_axis()
	{
		let half_quarter_matrix = Matrix::rotation_y(PI / 4.0);
		let full_quarter_matrix = Matrix::rotation_y(PI / 2.0);
		let p = Tuple::point(0.0, 0.0, 1.0);
		let half_quarter_res = half_quarter_matrix * p;
		let full_quarter_res = full_quarter_matrix * p;

		let half_quarter = Tuple::point((2.0).sqrt() / 2.0, 0.0, (2.0).sqrt() / 2.0);
		let full_quarter = Tuple::point(1.0, 0.0, 0.0);
		assert_eq!(half_quarter_res, half_quarter);
		assert_eq!(full_quarter_res, full_quarter);
	}
	#[test]
	fn rotating_a_point_around_z_axis()
	{
		let half_quarter_matrix = Matrix::rotation_z(PI / 4.0);
		let full_quarter_matrix = Matrix::rotation_z(PI / 2.0);
		let p = Tuple::point(0.0, 1.0, 0.0);
		let half_quarter_res = half_quarter_matrix * p;
		let full_quarter_res = full_quarter_matrix * p;

		let half_quarter = Tuple::point(-(2.0).sqrt() / 2.0, (2.0).sqrt() / 2.0, 0.0);
		let full_quarter = Tuple::point(-1.0, 0.0, 0.0);
		assert_eq!(half_quarter_res, half_quarter);
		assert_eq!(full_quarter_res, full_quarter);
	}
	#[test]
	fn shearing_x_in_proportion_of_y()
	{
		let matrix_translate = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(5.0, 3.0, 4.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn shearing_x_in_proportion_of_z()
	{
		let matrix_translate = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(6.0, 3.0, 4.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn shearing_y_in_proportion_of_x()
	{
		let matrix_translate = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(2.0, 5.0, 4.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn shearing_y_in_proportion_of_z()
	{
		let matrix_translate = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(2.0, 7.0, 4.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn shearing_z_in_proportion_of_x()
	{
		let matrix_translate = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(2.0, 3.0, 6.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn shearing_z_in_proportion_of_y()
	{
		let matrix_translate = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
		let p = Tuple::point(2.0, 3.0, 4.0);
		let expected = Tuple::point(2.0, 3.0, 7.0);
		let actual_res = matrix_translate * p;
		assert_eq!(actual_res, expected);
	}
	#[test]
	fn chaining_successives_transformations()
	{
		let a = Matrix::rotation_x(PI / 2.0);
		let b = Matrix::scaling(5.0, 5.0 ,5.0);
		let c = Matrix::translation(10.0, 5.0, 7.0);
		let p = Tuple::point(1.0, 0.0, 1.0);
		let expected_p2 = Tuple::point(1.0, -1.0, 0.0);
		let expected_p3 = Tuple::point(5.0, -5.0, 0.0);
		let expected_p4 = Tuple::point(15.0, 0.0, 7.0);
		let p2 = a * p;
		assert_eq!(expected_p2, p2);
		let p3 = b * p2;
		assert_eq!(expected_p3, p3);
		let p4 = c * p3;
		assert_eq!(expected_p4, p4);
	}
	#[test]
	fn chaining_multiple_transformations_in_reverse_order()
	{
		let a = Matrix::rotation_x(PI / 2.0);
		let b = Matrix::scaling(5.0, 5.0 ,5.0);
		let c = Matrix::translation(10.0, 5.0, 7.0);
		let p = Tuple::point(1.0, 0.0, 1.0);
		let chain = c * b * a;
		let expected_p = Tuple::point(15.0, 0.0, 7.0);
		let p = chain * p;
		assert_eq!(expected_p, p);
	}
}