pub fn epsil_compare(left : f64, right: f64) -> bool {
	let epsilon = 0.00001;
	(left- right).abs() < epsilon
}