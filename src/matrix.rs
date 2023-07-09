use crate::fuzzy_eq::*;
use std::convert::From;
use std::ops::{Index, IndexMut, Mul};

type Matrix4fArray= [Matrix4fArrayRow; 4];
type Matrix4fArrayRow = [f64; 4];
type Matrix3fArray= [Matrix3fArrayRow; 3];
type Matrix3fArrayRow = [f64; 3];
type Matrix2fArray= [Matrix2fArrayRow; 2];
type Matrix2fArrayRow = [f64; 2];

// @TODO refactor matrix to have one universal type

#[derive(Debug, Copy, Clone)]
pub struct Matrix2f{
	data: Matrix2fArray,
}

impl From<Matrix2fArray> for Matrix2f {
	fn from(data: Matrix2fArray) -> Self {
		Matrix2f { data }
	}
}

impl Index<usize> for Matrix2f{
	type Output = Matrix2fArrayRow;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl Matrix2f {
	pub fn new() -> Matrix2f {
		Matrix2f::from([
			[0.0, 0.0],
			[0.0, 0.0],])
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix3f{
	data: Matrix3fArray,
}

impl From<Matrix3fArray> for Matrix3f {
	fn from(data: Matrix3fArray) -> Self {
		Matrix3f { data }
	}
}

impl Index<usize> for Matrix3f{
	type Output = Matrix3fArrayRow;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl Matrix3f {
	pub fn new() -> Matrix3f {
		Matrix3f::from([
			[0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0],
			[0.0, 0.0, 0.0],])
	}
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

//TODO:
//I'm absolutly sure i can do an oveload for '*' multiplication between matrixes

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

impl FuzzyEq<Matrix3f> for Matrix3f {
	fn fuzzy_eq(&self, other: &Matrix3f) -> bool {
	 	self[0][0].fuzzy_eq(&other[0][0])
		&& self[0][1].fuzzy_eq(&other[0][1])
		&& self[0][2].fuzzy_eq(&other[0][2])
		&& self[1][0].fuzzy_eq(&other[1][0])
		&& self[1][1].fuzzy_eq(&other[1][1])
		&& self[1][2].fuzzy_eq(&other[1][2])
		&& self[2][0].fuzzy_eq(&other[2][0])
		&& self[2][1].fuzzy_eq(&other[2][1])
		&& self[2][2].fuzzy_eq(&other[2][2])
	}
}

impl FuzzyEq<Matrix2f> for Matrix2f {
	fn fuzzy_eq(&self, other: &Matrix2f) -> bool {
		self[0][0].fuzzy_eq(&other[0][0])
		&& self[0][1].fuzzy_eq(&other[0][1])
		&& self[1][0].fuzzy_eq(&other[1][0])
		&& self[1][1].fuzzy_eq(&other[1][1])
	}
}

#[cfg(test)]
mod tests{
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
		let matrix1 = Matrix2f::from([
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
		let matrix1 = Matrix2f::from([
			[-3.0, 5.0],
			[1.0, -2.0],]);
		let matrix2 = Matrix2f::from([
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
}