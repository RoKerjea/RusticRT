extern crate rustic_rt as raytracer;

use std::fs::write;
use num_traits::Float;

use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::canvas::to_ppm::*;
use raytracer::canvas::to_png::*;
use raytracer::tuple::*;

#[derive(Debug)]
struct Environment<T>
where
	T: Float,
{
    gravity: Tuple<T>,
    wind: Tuple<T>,
}

#[derive(Debug)]
struct Projectile<T>
where
	T: Float,
{
    position: Tuple<T>,
    velocity: Tuple<T>,
}

impl<T> Projectile<T>
where
	T: Float,
{
    pub fn new(position: Tuple<T>, velocity: Tuple<T>) -> Self {
        Projectile { position, velocity }
    }
}

impl<T> Environment<T>
where
	T: Float,
{
    pub fn new(gravity: Tuple<T>, wind: Tuple<T>) -> Self {
        Environment { gravity, wind }
    }
}

fn tick<T>(environment: &Environment<T>, projectile: &Projectile<T>) -> Projectile<T>
where
	T: Float,
{
    Projectile::new(
        projectile.position + projectile.velocity,
        projectile.velocity + environment.gravity + environment.wind,
    )
}

enum Pixel {
	Coordinate{
		x: usize,
		y: usize,
	},
	OutOfBound,
}


impl Pixel {
	fn from_point_for_canvas<T>(point: Tuple<T>, canvas: &Canvas) -> Pixel
	where
		T: Float,
	{
		if !point.is_point() {
			panic!("Given tuple is not point. Point is needed to conversion for screen space.");
		}

		let rx = point.x.round();
		let ry = point.y.round();

		let ux = rx.to_usize().unwrap();
        let uy = ry.to_usize().unwrap();

		if rx.is_sign_negative() || ry.is_sign_negative() || ux >= canvas.width || uy >= canvas.height{
			return Pixel::OutOfBound;
		}
		let screen_x = ux;
		let screen_y = canvas.height - uy;

		Pixel::Coordinate{
			x: screen_x,
			y: screen_y,
		}
	}
}
fn main() {
    let environment = Environment::new(Tuple::vector(0.0, -0.1, 0.0), Tuple::vector(-0.02, 0.0, 0.0));
    let projectile = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25);

	let mut canvas = Canvas::new(900, 500);
	let color = Color::new(1.0, 1.0, 0.0);

    println!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;
    while current.position.y > 0.0 {
        println!("{}: {:?}", iteration, current);

		match Pixel::from_point_for_canvas(current.position, &canvas){
			Pixel::Coordinate{x, y} =>{
				canvas.write_pixel(x, y, color);
			}
			Pixel::OutOfBound => {}
		}

        current = tick(&environment, &current);
        iteration += 1;
    }
    println!("FINISHED => {}: {:?}", iteration, current);

    println!("Writing ./output.ppm");
    let ppm = canvas.to_ppm();
    write("./output.ppm", ppm).expect("Could not write ouput.ppm to disk.");
    println!("Writing ./output.png");
    let png = canvas.to_png();
    write("./output.png", png).expect("Could not write ouput.png to disk.");

    println!("Everything done.");
}
