use crate::tuple::*;
use crate::color::*;

struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight{
    pub fn new (position: Tuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests{
	// use crate::fuzzy_eq::FuzzyEq;
	use super::*;

	#[test]
	fn	point_light_has_position_and_intensity()
	{
		let intensity = Color::new(1.0, 1.0, 1.0);
		let pos = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(pos, intensity);

		assert_eq!(light.position, pos);
		assert_eq!(light.intensity, intensity);
	}
}