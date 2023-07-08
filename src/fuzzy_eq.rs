
//trait are kinda like interface class in c++?
pub	trait	FuzzyEq<T> {
	fn	fuzzy_eq(&self, other: &T) -> bool;

	fn	fuzzy_ne(&self, other: &T) -> bool{
		!self.fuzzy_eq(other)
	}
}

impl FuzzyEq<f64> for f64 {
    fn fuzzy_eq(&self, other: &f64) -> bool {
        (*self - *other).abs() < 0.00001
    }
}

// pub fn fuzzy_eq(left : f64, right: f64) -> bool {
// 	let epsilon = 0.00001;
// 	(left- right).abs() < epsilon
// }

//Macro "adapted" from 'assert_eq!
//i don't understand what's happening here for now
//and honestly, i really don't need it for now

// #[macro_export]
// macro_rules! assert_fuzzy_eq {
// 	($left:expr, $right:expr $(,)?) => {
		
// 	};
// }