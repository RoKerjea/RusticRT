
type Matrix4f= [[f64; 4]; 4];

trait Matrix4fNew {
	fn new(rows: Matrix4f)-> Matrix4f {
		rows
	}
}

impl Matrix4fNew for Matrix4f {}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn	constructing_a_matrix()
	{
		let matrix1: Matrix4f =[
			[1.0, 2.0, 3.0, 4.0],
			[5.5, 6.5, 7.5, 8.5],
			[9.0, 10.0, 11.0, 12.0],
			[13.5, 14.5, 15.5, 16.5],];
	}
}