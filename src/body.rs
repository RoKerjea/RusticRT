use crate::F;
use crate::fuzzy_eq::FuzzyEq;
use crate::matrix::Matrix;
use crate::plane::Plane;
// use crate::intersections::*;
use crate::ray::*;
use crate::material::Material;
use crate::sphere::*;
use crate::intersections::*;
use crate::tuple::*;

pub trait Intersectable {
	fn material(&self) -> Material;
	fn normal_at_in_object_space(&self, point: Tuple) -> Tuple;
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

	fn normal_at(&self, point: Tuple) -> Tuple{
        let object_point = self.transform().inverse() * point;
        let object_normal = self.normal_at_in_object_space(object_point); 
        let mut world_normal = self.transform().inverse().transpose() * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize() 
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Body{
	Sphere(Sphere),
	Plane(Plane),
}

impl From<Sphere> for Body {
	fn from(sphere: Sphere) -> Self {
		Body::Sphere(sphere)
	}
}

impl From<Plane> for Body {
	fn from(plane: Plane) -> Self {
		Body::Plane(plane)
	}
}

impl Intersectable for Body{
	fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(F, Body)> {
		match  *self {
			Body::Sphere(ref sphere) => sphere.intersect_in_object_space(object_space_ray),
			Body::Plane(ref plane) => plane.intersect_in_object_space(object_space_ray),
		}
	}
	fn normal_at_in_object_space(&self, object_space_point: Tuple) -> Tuple {
		match *self {
			Body::Sphere(ref sphere) => sphere.normal_at_in_object_space(object_space_point),
			Body::Plane(ref plane) => plane.normal_at_in_object_space(object_space_point),
		}
	}
	fn material(&self) -> Material {
		match  *self {
			Body::Sphere(ref sphere) => sphere.material(),
			Body::Plane(ref plane) => plane.material(),
		}
	}
	fn transform(&self) -> Matrix<4> {
		match  *self {
			Body::Sphere(ref sphere) => sphere.transform(),
			Body::Plane(ref plane) => plane.transform(),
		}
	}
}
impl FuzzyEq<Body> for Body {
	fn fuzzy_eq(&self, other: Body) -> bool {
	  match (*self, other) {
		(Body::Sphere(ref sphere), Body::Sphere(ref other)) => sphere.fuzzy_eq(other),
		(Body::Plane(ref plane), Body::Plane(ref other)) => plane.fuzzy_eq(other),
		_ => false,
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