use crate::matrix::*;
use crate::ray::*;
use crate::body::*;
use crate::intersections::*;
use crate::tuple::*;
use crate::F;
use crate::color::*;

pub trait Illuminated {

}

pub enum Material {
    Phong(Phong),
}

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

impl Phong {
    fn new(
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
}