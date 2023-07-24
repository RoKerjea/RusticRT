use crate::tuple::*;
use crate::F;
use crate::color::*;
use crate::lights::PointLight;

pub trait Illuminated {
    fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Phong(Phong),
}

impl From<Phong> for Material {
    fn from(value: Phong) -> Self {
        Material::Phong(value)
    }
}
impl Default for Material {
    fn default() -> Self {
        Material::from(Phong::default())
    }
}

impl Illuminated for Material {
    fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        match  *self {
            Material::Phong(ref m) => m.lighting(light, position, eyev, normalv)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shine: F,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shine: 200.0
        }
    }
}

impl Illuminated for Phong {
    fn lighting(&self, light: PointLight, position: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let diffuse : Color;
        let specular : Color;
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        }
        else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0{
                specular = Color::black();
            }
            else {
            	let factor = reflect_dot_eye.powf(self.shine);
            	specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Phong {
   pub fn new(
        color: Color,
        ambient: F,
        diffuse: F,
        specular: F,
        shine: F)
        -> Self {
            Phong {
                color,
                ambient,
                diffuse,
                specular,
                shine
            }

    }
	pub fn with_color(color: Color) -> Self {
		Phong {
		  color,
		  ..Self::default()
		}
	}
}

#[cfg(test)]
mod tests{
	// use crate::fuzzy_eq::FuzzyEq;
	use super::*;
	#[test]
	fn default_material()
	{
		let m = Phong::default();

		assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
		assert_eq!(m.ambient, 0.1);
		assert_eq!(m.diffuse, 0.9);
		assert_eq!(m.specular, 0.9);
		assert_eq!(m.shine, 200.0);
	}
    #[test]
    fn  lighting_eyes_between_light_and_surface()
    {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let res = m.lighting(light, position, eyev, normalv);
        let expected = Color::new(1.9, 1.9, 1.9);
        assert_eq!(res, expected);
    }
	#[test]
  fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(1.0, 1.0, 1.0);

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(0.7364, 0.7364, 0.7364);

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
    let eyev = Tuple::vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(1.6364, 1.6364, 1.6364);

    assert_eq!(actual_result, expected_result);
  }

  #[test]
  fn lighting_with_light_behind_the_surface() {
    let m = Phong::default();
    let position = Tuple::point(0.0, 0.0, 0.0);

    let eyev = Tuple::vector(0.0, 0.0, -1.0);
    let normalv = Tuple::vector(0.0, 0.0, -1.0);
    let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let actual_result = m.lighting(light, position, eyev, normalv);

    let expected_result = Color::new(0.1, 0.1, 0.1);

    assert_eq!(actual_result, expected_result);
  }
}