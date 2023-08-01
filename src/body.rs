use crate::F;
use crate::matrix::Matrix;
// use crate::intersections::*;
use crate::ray::*;
use crate::material::Material;
use crate::sphere::*;
use crate::intersections::*;
use crate::tuple::*;

pub trait Intersectable {
	fn normal_at(&self, point: Tuple) -> Tuple;
	fn material(&self) -> Material;
	fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(F, Body)>;
	fn transform(&self) -> Matrix<4>;
	fn intersect(&self, ray: Ray) -> Intersections
	{
        let object_space_ray = ray.transform(self.transform().inverse());
		let ts = self.intersect_in_object_space(object_space_ray);
		Intersections::new(ts.into_iter().map(|(t, body)| {
			Intersection::new(t, ray, body)
		}).collect())
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Body{
	Sphere(Sphere),
}

impl From<Sphere> for Body {
	fn from(sphere: Sphere) -> Self {
		Body::Sphere(sphere)
	}
}

impl Intersectable for Body{
	fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(F, Body)> {
		match  *self {
			Body::Sphere(ref sphere) => sphere.intersect_in_object_space(object_space_ray),
		}
	}
	fn normal_at(&self, point: Tuple) -> Tuple {
		match *self {
			Body::Sphere(ref sphere) => sphere.normal_at(point),
		}
	}
	fn material(&self) -> Material {
		match  *self {
			Body::Sphere(ref sphere) => sphere.material(),
		}
	}
	fn transform(&self) -> Matrix<4> {
		match  *self {
			Body::Sphere(ref sphere) => sphere.transform(),
		}
	}
}



#[cfg(test)]
mod tests{
	// use crate::fuzzy_eq::FuzzyEq;
	use super::*;
	// use crate::tuple::*
	#[test]
	fn	intersection_encapsulate_t_and_body()
	{
		let s = Sphere::default();

		let r = Ray::new(Tuple::point(1.0, 1.0, 1.0), Tuple::vector(0.0, 0.0, 1.0));
		let i = Intersection::new(3.5, r, Body::from(s));
		assert_eq!(i.t, 3.5);
		assert_eq!(i.body, Body::from(s));
	}
	// #[test]
	// fn	aggregating_intersections()
	// {
	// 	let s = Sphere::new(None);
	// 	let i = Intersection::new(1.0, Body::from(s));
	// 	let i2 = Intersection::new(2.0, Body::from(s));
	// 	let xs = intersections(i1, i2);
	// 	assert_eq!(xs.len(), 2);
	// 	assert_eq!(xs[0].t, 1.0);
	// 	assert_eq!(xs[2].t, 1.0);
	// }
	// #[test]
	// fn intersections_set_object()
	// {
	// 	let s = Sphere::new(None);
	// 	let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

	// 	let xs = s.intersect(r);
	// 	assert_eq!(xs.len(), 2);
	// 	assert_eq!(xs[0].body, Body::from(s));
	// 	assert_eq!(xs[2].body, Body::from(s));
	// }
}