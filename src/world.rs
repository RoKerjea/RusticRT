
use crate::body::*;
use crate::color::Color;
use crate::computed_intersection::ComputedIntersection;
use crate::fuzzy_eq::FuzzyEq;
use crate::intersections::*;
use crate::lights::PointLight;
use crate::material::*;
use crate::ray::*;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub bodies: Vec<Body>,
    pub lights: Vec<PointLight>,
    pub reflection_limit: usize,
}

impl World {
    pub fn new(bodies: Vec<Body>, lights: Vec<PointLight>) -> Self {
        //FIXME make reflection_limit configurable
        //FIXME change world parsing to update
        World { bodies, lights , reflection_limit: 5}
    }
    pub fn intersect(&self, ray: Ray) -> Intersections {
        // need more infos about iter, flat_map and collect
        let xs = self
            .bodies
            .iter()
            .flat_map(|body| body.intersect(ray))
            .collect();
        Intersections::new(xs)
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        // let xs = self.intersect(ray);

        // let hit = xs.hit();
        // if let Some(hit) = hit {
        //     let c = hit.get_computed();
        //     let material = hit.body.material();
        //     let is_in_shadow = self.is_shadowed(c.over_point);
        //     let surface_color = material.lighting(&hit.body, self.lights[0], c.over_point, c.eyev, c.normalv, is_in_shadow);
        //     let reflected_color = self.reflect_color_at(&material, &c);
        //     surface_color + reflected_color
        // } else {
        //     Color::black()
        // }
        self.color_at_with_reflection_limit(ray, self.reflection_limit)
    }

    fn color_at_with_reflection_limit(&self, ray: Ray, remaining_reflections: usize) -> Color
    {
        let xs = self.intersect(ray);

        let hit = xs.hit();
        if let Some(hit) = hit {
            let c = hit.get_computed();
            let material = hit.body.material();
            let is_in_shadow = self.is_shadowed(c.over_point);
            let surface_color = material.lighting(&hit.body, self.lights[0], c.over_point, c.eyev, c.normalv, is_in_shadow);
            let reflected_color = self.reflect_color_at(&material, &c, remaining_reflections);
            surface_color + reflected_color
        } else {
            Color::black()
        }
    }

    pub fn reflect_color_at(&self, material: &Material, computed_intersection: &ComputedIntersection, remaining_reflections: usize) -> Color {
        if material.reflectiveness() == 0.0 || remaining_reflections == 0
        {
           return Color::black()
        }
        let reflected_ray = Ray::new(computed_intersection.over_point, computed_intersection.reflectv);
        let reflected_color = self.color_at_with_reflection_limit(reflected_ray, remaining_reflections - 1);
        reflected_color * material.reflectiveness()
    }

    fn is_shadowed(&self, position: Tuple) -> bool {
        let v = self.lights[0].position - position;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(position, direction);

        let xs = self.intersect(r);
        if let Some(hit) = xs.hit() {
            if hit.t < distance {
                return true;
            }
        }
        false
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            bodies: vec![],
            lights: vec![],
            reflection_limit : 5,
        }
    }
}

impl FuzzyEq<World> for World {
	fn fuzzy_eq(&self, other: World) -> bool {
	  self.bodies.fuzzy_eq(other.bodies) && self.lights.fuzzy_eq(other.lights)
	}
  }

#[cfg(test)]
mod tests {
    use crate::color::*;

    use crate::sphere::Sphere;

    use super::*;
    use crate::matrix::*;
    use crate::tuple::*;
    #[test]
    fn empty_wordl() {
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
		let s1 = Body::from(Sphere::default().with_material(Material::from(material)));
		let s2 = Body::from(Sphere::default().with_transform(Matrix::scaling(0.5, 0.5, 0.5)));

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
		let s1 = Body::from(Sphere::default().with_material(Material::from(material)));
		let s2 = Body::from(Sphere::default().with_transform(Matrix::scaling(0.5, 0.5, 0.5)));

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
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = create_default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);

        assert_eq!(c, Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = create_default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let w = create_default_world();
        let p = Tuple::point(0.0, 10.0, 0.0);
        let is_in_shadow = w.is_shadowed(p);

        assert_eq!(is_in_shadow, false);
    }

    #[test]
    fn there_is_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = create_default_world();
        let p = Tuple::point(10.0, -10.0, 10.0);
        let is_in_shadow = w.is_shadowed(p);

        assert_eq!(is_in_shadow, true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = create_default_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        let is_in_shadow = w.is_shadowed(p);

        assert_eq!(is_in_shadow, false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = create_default_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        let is_in_shadow = w.is_shadowed(p);

        assert_eq!(is_in_shadow, false);
    }

    //   #[test]
    //   fn the_color_when_a_ray_hits_something_in_shadow() {
    //     // let material = Material::default();
    //     let s1 = Sphere::new(Matrix::identity());
    //     let s2 = Sphere::new(Matrix::translation(0.0, 0.0, 10.0));
    //     let light = PointLight::new(Tuple::point(0.0,0.0,-10.0), Color::new(1.0, 1.0, 1.0));
    //     let w = World::new(vec![s1.into(), s2.into()], vec![light]);

    //     let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let c = w.color_at(r);

    //     assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    //   }
}
