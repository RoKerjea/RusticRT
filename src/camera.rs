use crate::F;
use crate::fuzzy_eq::FuzzyEq;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Clone, Debug)]
pub struct Camera {
	pub transform: Matrix<4>,
	pub hsize: usize,
	pub vsize: usize,
	pub field_of_view: F,
	half_width: F,
	half_height: F,
	pixel_size: F,
}

impl Camera{
	pub fn new(hsize: usize, vsize: usize, field_of_view: F) -> Self {
		let half_size = (field_of_view/2.0).tan();
		let aspect_ratio = hsize as F / vsize as F;
		let half_width;
		let half_height;

		if aspect_ratio >= 1.0 {
			half_width = half_size;
			half_height = half_size / aspect_ratio;
		} else {
			half_height = half_size;
			half_width = half_size * aspect_ratio;
		}

		let pixel_size = (half_width * 2.0) / hsize as F;

		Camera {
			hsize,
			vsize,
			field_of_view,
			transform: Matrix::identity(),
			half_height,
			half_width,
			pixel_size,
		}
	}
	pub fn with_transform(mut self, transform : Matrix<4>) -> Self {
		self.transform = transform;
		self
	}
	pub fn view_transform(mut self, from: Tuple, to: Tuple, up: Tuple) -> Self {
		let forward = (to - from).normalize();
		let upn = up.normalize();
		let left = forward.cross(upn);
		let true_up = left.cross(forward);
		
		let orientation = Matrix::from([
			[left.x, left.y, left.z, 0.0],
			[true_up.x, true_up.y, true_up.z, 0.0],
			[-forward.x, -forward.y, -forward.z, 0.0],
			[0.0, 0.0, 0.0, 1.0],
			]);
		let translation = Matrix::translation(-from.x, -from.y, -from.z);
		self.transform = orientation * translation;
		self

	}
	pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
		let offset_x = (0.5 + x as f64) * self.pixel_size;
		let offset_y = (0.5 + y as f64) * self.pixel_size;
		let world_x = self.half_width - offset_x;
		let world_y = self.half_height - offset_y;

		let inverse_view_transform = self.transform.inverse();

		let wall_point = inverse_view_transform * Tuple::point(world_x, world_y, -1.0);
		let ray_origin = inverse_view_transform * Tuple::point(0.0, 0.0, 0.0);
		let ray_direction = (wall_point - ray_origin).normalize();
		let ray = Ray::new(ray_origin, ray_direction);

		ray
	}	
}

impl FuzzyEq<Camera> for Camera {
  fn fuzzy_eq(&self, other: Camera) -> bool {
    self.transform.fuzzy_eq(other.transform)
      && self.vsize == other.vsize
      && self.hsize == other.hsize
      && self.field_of_view.fuzzy_eq(other.field_of_view)
  }
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::f64::consts::PI;
	#[test]
  fn constructing_a_camera() {
    let hsize = 160;
    let vsize = 120;
    let fov = PI / 2.0;

    let camera = Camera::new(hsize, vsize, fov);

    assert_eq!(camera.vsize, vsize);
    assert_eq!(camera.hsize, hsize);
    assert_eq!(camera.field_of_view, fov);
  }

  #[test]
  fn constructed_a_camera_has_an_identity_transform() {
    let hsize = 160;
    let vsize = 120;
    let fov = PI / 2.0;

    let camera = Camera::new(hsize, vsize, fov);

    assert_eq!(camera.transform, Matrix::identity());
  }

  #[test]
  fn constructing_a_camera_with_a_transform() {
    let hsize = 160;
    let vsize = 120;
    let fov = PI / 2.0;
    let transform = Matrix::translation(1.2, 3.4, -20.6);

    let camera = Camera::new(hsize, vsize, fov).with_transform(transform);

    assert_eq!(camera.vsize, vsize);
    assert_eq!(camera.hsize, hsize);
    assert_eq!(camera.field_of_view, fov);
    assert_eq!(camera.transform, transform);
  }

  #[test]
  fn constructing_a_ray_through_the_center_of_the_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);

    assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
  }

  #[test]
  fn constructing_a_ray_through_the_corner_of_a_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);

    assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
  }

  #[test]
  fn constructing_a_ray_when_camera_is_transformed() {
    let c = Camera::new(201, 101, PI / 2.0)
      .with_transform(Matrix::rotation_y(PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);

    assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
    assert_eq!(
      r.direction,
      Tuple::vector((2.0 as f64).sqrt() / 2.0, 0.0, -((2.0 as f64).sqrt()) / 2.0)
    );
  }

  #[test]
  fn pixel_size_for_horizontal_canvas() {
    let c = Camera::new(200, 125, PI / 2.0);
    assert!(c.pixel_size.fuzzy_eq(0.01));
  }

  #[test]
  fn pixel_size_for_vertical() {
    let c = Camera::new(125, 200, PI / 2.0);
    assert!(c.pixel_size.fuzzy_eq(0.01));
  }

  #[test]
  fn view_transform_for_the_default_orientation() {
    let from = Tuple::point(0.0, 0.0, 0.0);
    let to = Tuple::point(0.0, 0.0, -1.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let camera = Camera::new(100, 100, PI / 2.0).view_transform(from, to, up);
    assert_eq!(camera.transform, Matrix::identity());
  }

  #[test]
  fn view_transformation_looking_into_positive_z_direction() {
    let from = Tuple::point(0.0, 0.0, 0.0);
    let to = Tuple::point(0.0, 0.0, 1.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let camera = Camera::new(100, 100, PI / 2.0).view_transform(from, to, up);
    assert_eq!(camera.transform, Matrix::scaling(-1.0, 1.0, -1.0));
  }

  #[test]
  fn view_transformation_moves_the_world() {
    let from = Tuple::point(0.0, 0.0, 8.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    let camera = Camera::new(100, 100, PI / 2.0).view_transform(from, to, up);
    assert_eq!(camera.transform, Matrix::translation(0.0, 0.0, -8.0));
  }

  //need to really redo comparaison operator between matrix,
  //i refuse to use a macro for equality, it's disgusting
//   #[test]
//   fn an_arbitrary_view_transformation() {
//     let from = Tuple::point(1.0, 3.0, 2.0);
//     let to = Tuple::point(4.0, -2.0, 8.0);
//     let up = Tuple::vector(1.0, 1.0, 0.0);
//     let camera = Camera::new(100, 100, PI / 2.0).view_transform(from, to, up);
//     assert_eq!(
//       camera.transform,
//       Matrix::from([
//         [-0.50709, 0.50709, 0.67612, -2.36643],
//         [0.76772, 0.60609, 0.12122, -2.82843],
//         [-0.35857, 0.59761, -0.71714, 0.0],
//         [0.0, 0.0, 0.0, 1.0],
//       ])
//     );
//   }
}