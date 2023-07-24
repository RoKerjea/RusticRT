use crate::lights::PointLight;
use crate::matrix::*;
use crate::ray::*;
use crate::body::*;
use crate::intersections::*;
use crate::sphere::Sphere;
use crate::tuple::*;
use crate::material::*;

pub struct World {
	pub bodies: Vec<Body>,
	pub lights: Vec<PointLight>,
}

impl World {
	pub fn new(bodies: Vec<Body>, lights: Vec<PointLight>) -> Self {
		World {
			bodies,
			lights,
		}
	}
	pub fn intersect(&self, ray: Ray) -> Intersections
	{
		// need more infos about iter, flat_map and collect
		let xs = self.bodies.iter().flat_map(|body| body.intersect(ray)).collect();
		Intersections::new(xs)
	}
}

impl Default for World {
	fn default() -> Self {
		World { bodies: vec![], lights: vec![] }
	}
}


#[cfg(test)]
mod tests{
	use crate::color::*;
	use super::*;
	#[test]
	fn empty_wordl()
	{
		let world = World::default();

		assert_eq!(world.bodies.len(), 0);
		assert_eq!(world.lights.len(), 0);
	}
	fn create_default_world() -> World {
		let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
		let material = Phong {
		  color: Color::new(0.8, 1.0, 0.6),
		  diffuse: 0.7,
		  specular: 0.2,
		  ..Phong::default()
		};
		let s1 = Body::from(Sphere::with_material(Material::from(material), None));
		let s2 = Body::from(Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5))));

		World::new(vec![s1, s2], vec![light])
	}

	#[test]
	fn the_default_world() {
		let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
		let material = Phong {
			color: Color::new(0.8, 1.0, 0.6),
			diffuse: 0.7,
			specular: 0.2,
			..Phong::default()
		};
		let s1 = Body::from(Sphere::with_material(Material::from(material), None));
		let s2 = Body::from(Sphere::new(Some(Matrix::scaling(0.5, 0.5, 0.5))));

		let world = create_default_world();

		assert_eq!(2, world.bodies.len());
		assert_eq!(1, world.lights.len());

		assert!(world.bodies.contains(&s1));
		assert!(world.bodies.contains(&s2));

		assert!(world.lights.contains(&light));
	}

	#[test]
  fn intersect_a_world_with_a_ray() {
    let w = create_default_world();
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

    let xs = w.intersect(r);

    assert_eq!(4, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(4.5, xs[1].t);
    assert_eq!(5.5, xs[2].t);
    assert_eq!(6.0, xs[3].t);
  }
}