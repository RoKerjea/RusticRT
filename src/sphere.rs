use crate::matrix::*;
use crate::ray::*;
use crate::body::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::material::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
	pub transform: Matrix<4>,
	pub material: Material,
}

impl Sphere {
 	pub fn new(transform: Option<Matrix<4>>) -> Self {
    	match transform {
      		None => Sphere{
        		transform: Matrix::identity(),
				material: Material::default(),
      		},
      		Some(transform) => Sphere {
				transform,
				material: Material::default(), },
    	}
  	}
	pub fn with_material(material: Material, transform: Option<Matrix<4>>) -> Self {
		let mut s = Self::new(transform);
		s.material = material;
		s
	}
}

impl Intersectable for Sphere{
	fn intersect(&self, ray: Ray, ) -> Intersections {
		let object_space_ray = ray.transform(self.transform.inverse());

		let sphere_to_ray = object_space_ray.origin - Tuple::point(0.0, 0.0, 0.0);
		let a = object_space_ray.direction.dot(object_space_ray.direction);
		let b = 2.0 * object_space_ray.direction.dot(sphere_to_ray);
		let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
		let discriminant = b.powi(2) - 4.0 * a * c;

		if discriminant < 0.0 {
			Intersections::new(vec![])
		}
		else {
			let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
			let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
			Intersections::from(vec![
					Intersection::new(t1, Body::from(*self)),
					Intersection::new(t2, Body::from(*self)),
			])
		}
  	}
	fn normal_at(&self, point: Tuple) -> Tuple {
		let object_point = self.transform.inverse() * point;
		let object_normal = (object_point - Tuple::new(0.0, 0.0, 0.0, 1.0)).normalize();
		let mut world_normal = self.transform.inverse().transpose() * object_normal;
		world_normal.w = 0.0;
		world_normal.normalize()
	}
	fn material(&self) -> Material {
		self.material
	}
}

#[cfg(test)]
mod tests{
	use crate::fuzzy_eq::FuzzyEq;
	use std::f64::consts::PI;
	use crate::F;
	use crate::color::*;
	use super::*;
    // use crate::{tuple::Tuple, matrix::Matrix};
	#[test]
	fn ray_intersect_sphere_at_two_point()
	{
		let origin = Tuple::point(0.0, 0.0, -5.0);
		let direction = Tuple::vector(0.0, 0.0, 1.0);
		let r = Ray::new(origin, direction);
		let sphere = Sphere::new(None);
		let xs = sphere.intersect(r);

		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, 4.0);
		assert_eq!(xs[1].t, 6.0);
	}
	#[test]
	fn ray_intersect_sphere_at_tangent()
	{
		let origin = Tuple::point(0.0, 1.0, -5.0);
		let direction = Tuple::vector(0.0, 0.0, 1.0);
		let r = Ray::new(origin, direction);
		let sphere = Sphere::new(None);
		let xs = sphere.intersect(r);

		assert_eq!(xs.len(), 2);
		assert_eq!(xs[0].t, 5.0);
		assert_eq!(xs[1].t, 5.0);
	}
	#[test]
	fn ray_miss_sphere()
	{
		let origin = Tuple::point(0.0, 2.0, -5.0);
		let direction = Tuple::vector(0.0, 0.0, 1.0);
		let r = Ray::new(origin, direction);
		let sphere = Sphere::new(None);
		let xs = sphere.intersect(r);

		assert_eq!(xs.len(), 0);
	}

	#[test]
	fn a_ray_originates_inside_a_sphere() {
	  let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
	  let s = Sphere::new(None);

	  let xs = s.intersect(r);

	  assert_eq!(2, xs.len());
	  assert_eq!(-1.0, xs[0].t);
	  assert_eq!(1.0, xs[1].t);
	}

	#[test]
	fn a_sphere_is_behind_a_ray() {
	  let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
	  let s = Sphere::new(None);

	  let xs = s.intersect(r);

	  assert_eq!(2, xs.len());
	  assert_eq!(-6.0, xs[0].t);
	  assert_eq!(-4.0, xs[1].t);
	}
	#[test]
	fn	sphere_default_transformation(){
		let s = Sphere::new(None);

		assert!(s.transform.fuzzy_eq(Matrix::identity()));
	}
	#[test]
	fn	changing_a_sphere_matrix(){
		let t = Matrix::translation(2.0, 3.0, 4.0);
		let s = Sphere::new(Some(t));

		assert!(s.transform.fuzzy_eq(t));
	}
	#[test]
	fn	intersecting_a_scaled_sphere_with_a_ray(){
		let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let t = Matrix::scaling(2.0, 2.0, 2.0);
		let s = Sphere::new(Some(t));
		let xs = s.intersect(r);

		assert_eq!(2, xs.len());
		assert_eq!(3.0, xs[0].t);
		assert_eq!(7.0, xs[1].t);
	}
	#[test]
	fn	intersecting_a_translated_sphere_with_a_ray(){
		let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
		let t = Matrix::translation(5.0, 0.0, 0.0);
		let s = Sphere::new(Some(t));
		let xs = s.intersect(r);

		assert_eq!(0, xs.len());
	}

	//intersection tests

// 	#[test]
// 	fn hit_when_all_intersections_have_positive_t()
// 	{
// 		let s = Sphere::new(None);

// 		let i1 = Intersection::new(1.0, Body::from(s));
// 		let i2 = Intersection::new(2.0, Body::from(s));

// 		let xs = Intersections::new(Vec![i2, i1]);
// 		assert_eq!(xs.hit(), Some(i1));
// 	}
	#[test]
	fn the_normal_at_a_sphere_x_axis()
	{
		let s = Sphere::new(None);
		let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
		
		let expected = Tuple::vector(1.0, 0.0, 0.0);
		assert_eq!(n, expected);
	}
	#[test]
	fn the_normal_at_a_sphere_y_axis()
	{
		let s = Sphere::new(None);
		let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
		
		let expected = Tuple::vector(0.0, 1.0, 0.0);
		assert_eq!(n, expected);
	}
	#[test]
	fn the_normal_at_a_sphere_z_axis()
	{
		let s = Sphere::new(None);
		let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
		
		let expected = Tuple::vector(0.0, 0.0, 1.0);
		assert_eq!(n, expected);
	}
	#[test]
	fn the_normal_at_a_sphere_nonaxial_point()
	{
		let sqrt3 = (3.0 as F).sqrt() / 3.0;
		let s = Sphere::new(None);
		let n = s.normal_at(Tuple::point(sqrt3, sqrt3,sqrt3));
		
		let expected = Tuple::vector(sqrt3, sqrt3,sqrt3);
		assert_eq!(n, expected);
		assert_eq!(n, n.normalize());
	}

	#[test]
	fn	normal_of_translated_sphere(){
		let t = Matrix::translation(0.0, 1.0, 0.0);
		let s = Sphere::new(Some(t));
		let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
		let expected = Tuple::vector(0.0, 0.70711, -0.70711);
		assert_eq!(n, expected);
	}
	#[test]
	fn	normal_of_transformed_sphere(){
		let sqrt2 = (2.0 as F).sqrt() / 2.0;
		let t = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(PI / 5.0);
		let s = Sphere::new(Some(t));
		let n = s.normal_at(Tuple::point(0.0, sqrt2, -sqrt2));
		let expected = Tuple::vector(0.0, 0.97014, -0.24254);
		assert_eq!(n, expected);
	}
	#[test]
	fn sphere_has_default_material(){
		let s = Sphere::new(None);
		let m = Material::default();

		assert_eq!(s.material, m);
	}
	#[test]
	fn sphere_may_be_assigned_material(){
		let m = Material::from(Phong::new(
			Color::new(1.0, 1.0, 0.0),
			0.05,
			0.7,
			0.95,
			400.0,
		  ));

		let s = Sphere::with_material(m, None);

		assert_eq!(s.material, m);
	}
}