extern crate rustic_rt as raytracer;

// use raytracer::canvas;
use raytracer::color::Color;
use std::f64::consts::PI;
use std::fs::write;
use rustic_rt::matrix::Matrix;

use raytracer::canvas::to_png::*;
use raytracer::canvas::to_ppm::*;
use raytracer::canvas::*;
use raytracer::tuple::*;

enum Pixel {
	Coordinate{
		x: usize,
		y: usize,
	},
	OutOfBounds {x: f64, y: f64},
}

impl Pixel
{
	fn from_point_for_canvas(point: Tuple, canvas: &Canvas) -> Pixel
	{
		if !point.is_point() {
			panic!("Given tuple is not point. Point is needed to conversion for screen space.");
		}

		let rx = point.x.round();
		let ry = point.y.round();

		let ux = rx as usize;
        let uy = ry as usize;

		if rx.is_sign_negative() || ry.is_sign_negative() || ux >= canvas.width || uy >= canvas.height{
			return Pixel::OutOfBounds{ x: rx, y: ry };
		}
		let screen_x = ux;
		let screen_y = canvas.height - uy;

		Pixel::Coordinate{
			x: screen_x,
			y: screen_y,
		}
	}
}

fn main(){
	const WIDTH: usize = 500;
	const HEIGHT: usize = 500;

	let mut	canvas: Canvas = Canvas::new(WIDTH, HEIGHT);
	let color = Color::new(1.0, 1.0, 0.0);

	let new_origin = Tuple::point((WIDTH / 2) as f64, (HEIGHT /2 ) as f64, 0.0);

	let origin_transform = Matrix::translation(new_origin.x, new_origin.y, new_origin.z);

	let point = Tuple::point(0.0, 200.0, 0.0);
	for hour in 0..12 {
		let rotation_transform = Matrix::rotation_z(2.0 * PI / 12.0 * (hour as f64));
		let transformed_point = origin_transform * rotation_transform * point;

		match Pixel::from_point_for_canvas(transformed_point, &canvas) {
            Pixel::Coordinate { x, y } => canvas.write_pixel(x, y, color),
            Pixel::OutOfBounds { x, y } => panic!(
                "Could not map point to screen/canvas: Out of bounds: {:?} x {:?}",
                x, y
            ),
        }
	}

	println!("Writing ./output.ppm");
    let ppm = canvas.to_ppm();
    write("./output.ppm", ppm).expect("Could not write ouput.ppm to disk.");
    println!("Writing ./output.png");
    let png = canvas.to_png();
    write("./output.png", png).expect("Could not write ouput.png to disk.");

    println!("Everything done.");
}