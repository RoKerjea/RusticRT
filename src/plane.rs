use crate::body::*;
use crate::fuzzy_eq::FuzzyEq;
use crate::material::*;
use crate::matrix::*;
use crate::ray::*;
use crate::EPSILON;
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub transform: Matrix<4>,
    pub material: Material,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: Default::default(),
        }
    }
}

impl Plane {
    pub fn new(material: Material, transform: Matrix<4>) -> Self {
        Plane {
            material,
            transform,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }
    pub fn with_transform(mut self, transform: Matrix<4>) -> Self {
        self.transform = transform;
        self
    }
}

impl Intersectable for Plane {
    fn intersect_in_object_space(&self, object_space_ray: Ray) -> Vec<(crate::F, Body)> {
        if object_space_ray.direction.y.abs() < EPSILON
            {return vec![];}
        else {
            let t = -object_space_ray.origin.y / object_space_ray.direction.y;
            vec![
                (t, Body::from(*self))
            ]
        }
    }
    fn normal_at_in_object_space(&self, _object_space_point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
    fn material(&self) -> Material {
        self.material
    }
    fn transform(&self) -> Matrix<4> {
        self.transform
    }
}

impl FuzzyEq<&Plane> for Plane {
    fn fuzzy_eq(&self, other: &Plane) -> bool {
      self.transform.fuzzy_eq(other.transform) && self.material.fuzzy_eq(other.material)
    }
  }

#[cfg(test)]
mod tests {
  use super::*;
use crate::ray::Ray;

  #[test]
  fn normal_of_a_plane_is_constant_everywhere() {
    let p = Plane::default();
    let n1 = p.normal_at_in_object_space(Tuple::point(0.0, 0.0, 0.0));
    let n2 = p.normal_at_in_object_space(Tuple::point(10.0, 0.0, -10.0));
    let n3 = p.normal_at_in_object_space(Tuple::point(-5.0, 0.0, 150.0));

    assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
    assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
    assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
  }

  #[test]
  fn intersect_with_a_ray_parallel_to_the_plane() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,10.0, 0.0), Tuple::vector(0.0,0.0,1.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 0);
  }

  #[test]
  fn intersect_with_a_coplanar_ray() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,0.0, 0.0), Tuple::vector(0.0,0.0,1.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 0);
  }

  #[test]
  fn intersect_from_above() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,1.0, 0.0), Tuple::vector(0.0,-1.0,0.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 1);
    assert_eq!(ts[0].0, 1.0); // t
    assert_eq!(ts[0].1, Body::from(p)) // body
  }

  #[test]
  fn intersect_from_below() {
    let p = Plane::default();
    let r = Ray::new(Tuple::point(0.0,-1.0, 0.0), Tuple::vector(0.0,1.0,0.0));
    let ts = p.intersect_in_object_space(r);

    assert_eq!(ts.len(), 1);
    assert_eq!(ts[0].0, 1.0); // t
    assert_eq!(ts[0].1, Body::from(p)) // body
  }
}