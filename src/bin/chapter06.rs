extern crate rustic_rt as raytracer;

use raytracer::body::Intersectable;
use raytracer::color::Color;
use raytracer::lights::PointLight;
use raytracer::ray::Ray;
use raytracer::sphere::*;
use raytracer::canvas::to_png::*;
use raytracer::canvas::*;
use raytracer::tuple::*;
use raytracer::material::{Material, Illuminated, Phong};

use std::fs::write;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;
use indicatif::ProgressBar;

fn main(){
	let ray_origin = Tuple::point(0.0, 0.0, -5.0);
	let wall_position = 10.0;
	let wall_size = 10.0;
	let half = wall_size/2.0;
	const WIDTH: usize = 2048;
	const HEIGHT: usize = 2048;

	let canvas_pixel_world_size = wall_size / WIDTH as f64;

	let material = Material::from(Phong::with_color(Color::new(1.0, 0.15, 1.0)));
	let sphere = Sphere::with_material(material, None);

	let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
	let canvas_mutex = Mutex::new(Canvas::new(WIDTH, HEIGHT));
	println!(
		"Raytracing {} pixels. Please be patient...", WIDTH * HEIGHT);

	let progress = ProgressBar::new((WIDTH * HEIGHT) as u64);
	progress.set_draw_rate(5);

	(0..WIDTH)
	.cartesian_product(0..HEIGHT)
	.par_bridge()
	.for_each(|(x, y)|
	{
		let ray_x = -half + (x as f64) * canvas_pixel_world_size;
		let ray_y = half - (y as f64) * canvas_pixel_world_size;
		let wall_point = Tuple::point(ray_x, ray_y, wall_position);
		let ray = Ray::new(ray_origin, (wall_point - ray_origin).normalize());
		let xs = sphere.intersect(ray);
		let hit = xs.hit();
		if let Some(hit) = hit {
			let computed = hit.get_computed();
			let color = hit.body.material().lighting(light, computed.point, computed.eyev, computed.normalv);
			let mut canvas = canvas_mutex.lock().unwrap();
			canvas.write_pixel(x, y, color);
		}
		progress.inc(1);
	});

	progress.finish();
    println!("Writing ./output3.png");
	let canvas = canvas_mutex.lock().unwrap();
    let png = canvas.to_png();
    write("./output3.png", png).expect("Could not write ouput2.png to disk.");

    println!("Everything done.");
}