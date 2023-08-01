extern crate rustic_rt as raytracer;

use raytracer::body::Body;
use raytracer::body::Intersectable;
use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::lights::PointLight;
use raytracer::matrix::Matrix;
use raytracer::ray::Ray;
use raytracer::sphere::*;
use raytracer::canvas::to_png::*;
use raytracer::canvas::*;
use raytracer::tuple::*;
use raytracer::material::{Material, Illuminated, Phong};
use raytracer::world::World;

use std::f64::consts::PI;
use std::fs::write;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;
use indicatif::ProgressBar;

fn main(){
	const WIDTH: usize = 1920;
	const HEIGHT: usize = 1080;

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let floor_material = Material::from(Phong {
        color : Color::new(0.5, 0.45, 0.45),
        specular: 0.0,
        ..Phong::default()
    });

    let floor_sphere = Sphere::with_material(
        floor_material,
        Some(Matrix::scaling(10.0, 0.01, 10.0)),
    );
  
    let left_wall_sphere = Sphere::with_material(
        floor_material,
        Some(
          Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-PI / 3.0)
            * Matrix::rotation_x(PI / 2.0)
            * Matrix::scaling(10.0, 0.01, 10.0),
        ),
      );
    
      let right_wall_sphere = Sphere::with_material(
        floor_material,
        Some(
          Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(PI / 4.0)
            * Matrix::rotation_x(PI / 2.0)
            * Matrix::scaling(10.0, 0.01, 10.0),
        ),
      );

	// Spheres
    let left_material = Material::from(Phong {
        color: Color::new(0.78, 0.28, 0.96),
        ..Phong::default()
      });
      let left_sphere = Sphere::with_material(
        left_material,
        Some(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33)),
      );
    
      let middle_material = Material::from(Phong {
        color: Color::new(1.0, 0.49, 0.0),
        diffuse: 0.7,
        specular: 0.1,
        shine: 50.0,
        ..Phong::default()
      });
      let middle_sphere =
        Sphere::with_material(middle_material, Some(Matrix::translation(-0.5, 1.0, 0.5)));
    
      let right_material = Material::from(Phong {
        color: Color::new(0.51, 0.75, 0.06),
        ..Phong::default()
      });
      let right_sphere = Sphere::with_material(
        right_material,
        Some(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5)),
      );
    
      let world = World::new(
        vec![
          Body::from(floor_sphere),
          Body::from(left_wall_sphere),
          Body::from(right_wall_sphere),
          Body::from(left_sphere),
          Body::from(middle_sphere),
          Body::from(right_sphere),
        ],
        vec![light],
      );
      let camera = Camera::new(WIDTH, HEIGHT, PI / 3.0).view_transform(
        Tuple::point(0.0, 3.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
      );
	
	let canvas_mutex = Mutex::new(Canvas::new(WIDTH, HEIGHT));
	println!(
		"Raytracing {} pixels. Please be patient...", WIDTH * HEIGHT);

	let progress = ProgressBar::new((WIDTH * HEIGHT) as u64);
	progress.set_draw_rate(5);

	(0..WIDTH)
	.cartesian_product(0..HEIGHT)
	.par_bridge()
	.for_each(|(x, y)| {
        let color = world.color_at(camera.ray_for_pixel(x, y));
        let mut canvas = canvas_mutex.lock().unwrap();
        canvas.write_pixel(x, y, color);
        progress.inc(1);
	});

	progress.finish();
    println!("Writing ./output5.png");
	let canvas = canvas_mutex.lock().unwrap();
    let png = canvas.to_png();
    write("./output5.png", png).expect("Could not write ouput5.png to disk.");

    println!("Everything done.");
}