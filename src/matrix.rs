use crate::fuzzy_eq::*;
use std::ops;

type Matrix4f= [[f64; 4]; 4];
type Matrix3f= [[f64; 3]; 3];
type Matrix2f= [[f64; 2]; 2];


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
//
// impl ops::Mul<Matrix4f> for Matrix4f{
// 	type Output = Self;
// 	fn mul(self, other: Self) -> Self{
// 		Matrix4f
// 	}
// }

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
		let matrix1: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],];
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
		let matrix1: Matrix2f =[
			[-3.0, 5.0],
			[1.0, -2.0],];
		assert_eq!(matrix1[0][0], -3.0);
		assert_eq!(matrix1[0][1], 5.0);
		assert_eq!(matrix1[1][0], 1.0);
		assert_eq!(matrix1[1][1], -2.0);
	}
	#[test]
	fn	constructing_a_3fmatrix()
	{
		let matrix1: Matrix3f =[
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],];
		assert_eq!(matrix1[0][0], -3.0);
		assert_eq!(matrix1[1][1], -2.0);
		assert_eq!(matrix1[2][2], 1.0);
	}
	#[test]
	fn	comparing_4fmatrixes_true()
	{
		let matrix1: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],];
		let matrix2: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],];
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_4fmatrixes_false()
	{
		let matrix1: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],];
		let matrix2: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 1.5],];
		assert!(!matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_3fmatrixes_true(){
		let matrix1: Matrix3f =[
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],];
		let matrix2: Matrix3f =[
			[-3.0, 5.0, 0.0],
			[1.0, -2.0, -7.0],
			[0.0, 1.0, 1.0],];
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	comparing_2fmatrixes_true(){
		let matrix1: Matrix2f =[
			[-3.0, 5.0],
			[1.0, -2.0],];
		let matrix2: Matrix2f =[
			[-3.0, 5.0],
			[1.0, -2.0],];
		assert!(matrix1.fuzzy_eq(&matrix2));
	}
	#[test]
	fn	multiplying_4f_matrixes()
	{
		let matrix1: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.0, 6.0, 7.0, 8.0],
			[9.0, 8.0, 7.0, 6.0],
			[5.0, 4.0, 3.0, 2.0],];
		let matrix2: Matrix4f =[
			[-2.0, 1.0, 2.0, 3.0],
			[3.0, 2.0, 1.0, -1.0],
			[4.0, 3.0, 6.0, 5.0],
			[1.0, 2.0, 7.0, 8.0],];
		let expected: Matrix4f =[
			[20.0, 22.0, 50.0, 48.0],
			[44.0, 54.0, 114.0, 108.0],
			[40.0, 58.0, 110.0, 102.0],
			[16.0, 26.0, 46.0, 42.0],];
		assert!((matrix1 * matrix2).fuzzy_eq(&expected));
	}
}