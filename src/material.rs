use crate::body::Body;
use crate::color::*;
use crate::fuzzy_eq::FuzzyEq;
use crate::lights::PointLight;
use crate::pattern::Pattern;
use crate::pattern::Stencil;
use crate::tuple::*;
use crate::F;

pub trait Illuminated {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color;
}

pub trait Reflective {
    fn reflectiveness(&self) -> F;
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
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        match *self {
            Material::Phong(ref m) => m.lighting(body, light, position, eyev, normalv, in_shadow),
        }
    }
}

impl Reflective for Material {
    fn reflectiveness(&self) -> F {
        match *self {
        Material::Phong(ref m) => m.reflective
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub pattern: Option<Pattern>,
    pub ambient: F,
    pub diffuse: F,
    pub specular: F,
    pub shine: F,
    pub reflective: F,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: Color::new(1.0, 1.0, 1.0),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shine: 200.0,
            reflective: 0.0,
        }
    }
}


impl Illuminated for Phong {
    fn lighting(
        &self,
        body: &Body,
        light: PointLight,
        position: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let diffuse: Color;
        let specular: Color;
        let mut color = self.color;
        if let Some(pattern) = self.pattern {
            color = pattern.color_at(position, body);
        }
        let effective_color = color * light.intensity;
        let lightv = (light.position - position).normalize();
        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shine);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Phong {
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: F) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: F) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: F) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: F) -> Self {
        self.shine = shininess;
        self
    }
    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = Some(pattern);
        self
    }
    pub fn with_reflective(mut self, reflective: F) -> Self {
        self.reflective = reflective;
        self
    }
}

impl FuzzyEq<Phong> for Phong {
    fn fuzzy_eq(&self, other: Phong) -> bool {
        self.color.fuzzy_eq(other.color)
            && self.ambient.fuzzy_eq(other.ambient)
            && self.diffuse.fuzzy_eq(other.diffuse)
            && self.specular.fuzzy_eq(other.specular)
            && self.shine.fuzzy_eq(other.shine)
            && self.pattern.fuzzy_eq(other.pattern)
    }
}

impl FuzzyEq<Material> for Material {
    fn fuzzy_eq(&self, other: Material) -> bool {
        match (self, other) {
            (Material::Phong(ref m), Material::Phong(other)) => m.fuzzy_eq(other),
            // Add default case (different types) to return false, once more than one
            // Material exists
            // _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::fuzzy_eq::FuzzyEq;
    use super::*;
    use crate::sphere::Sphere;
    #[test]
    fn default_material() {
        let m = Phong::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shine, 200.0);
    }
    #[test]
    fn lighting_eyes_between_light_and_surface() {
        let m = Material::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let res = m.lighting(&body, light, position, eyev, normalv, false);
        let expected = Color::new(1.9, 1.9, 1.9);
        assert_eq!(res, expected);
    }
    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
        let m = Phong::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);

        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, sqrt2_over_2, -sqrt2_over_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

        let expected_result = Color::new(1.0, 1.0, 1.0);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
        let m = Phong::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

        let expected_result = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
        let m = Phong::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);

        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let eyev = Tuple::vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

        let expected_result = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Phong::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&body, light, position, eyev, normalv, false);

        let expected_result = Color::new(0.1, 0.1, 0.1);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Phong::default();
        let body = Body::from(Sphere::default());
        let position = Tuple::point(0.0, 0.0, 0.0);

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&body, light, position, eyev, normalv, true);

        let expected_result = Color::new(0.1, 0.1, 0.1);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn phong_material_has_reflectiveness_by_default() {
        let m = Phong::default();

        assert_eq!(m.reflective, 0.0);
    }
	#[test]
	fn phong_material_can_be_assigned_reflectivness() {
		let m = Phong::default().with_reflective(0.5);

		assert_eq!(m.reflective, 0.5);
	}
}
