extern crate rustic_rt as raytracer;


use std::fs::write;
use raytracer::tuple::*;
use raytracer::canvas::*;
use raytracer::color::*;

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }
}

impl Environment {
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment { gravity, wind }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
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
	fn from_point_for_canvas(point: Tuple, canvas: &Canvas) -> Pixel {
		if !point.is_point() {
			panic!("Given tuple is not point. Point is needed to conversion for screen space.");
		}

		let rx = point.x.round();
		let ry = point.y.round();

		let ux = rx as usize;
		let uy = ry as usize;

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

	let ppm = canvas.to_ppm();
	write("./ouput.ppm", ppm).expect("Could not write ppm to disk.");
    println!("END");
}
