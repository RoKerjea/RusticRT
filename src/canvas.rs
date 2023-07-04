use std::vec::Vec;
use crate::color::Color;

// use super::util::*;

#[derive(Debug, Clone)]
pub struct	Canvas{
	pub width : usize,
	pub height : usize,

	pixels: Vec<Color>,
}

impl Canvas{
	pub fn new(width: usize, height: usize) -> Self{
		Self {
			width,
			height,
			pixels: vec![Color::black(); width * height],
		}
	}
	pub	fn color_at(&self, x: usize, y:usize) -> &Color {
		&self.pixels[self.get_pixel_index(x, y)]
	}
	pub fn	get_pixel_index(&self, x: usize, y: usize) ->usize {
		y * self.width + x
	}
	pub fn	write_pixel(&mut self, x: usize, y: usize, color:Color){
		let index = self.get_pixel_index(x, y);
		self.pixels[index] = color;
	}
	// pub fn	canvas_to_ppm(&self) -> String {
	// 	"P3"
	// 	self.width
	// 	self.height
	// 	"255"
	// }
}

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn	creating_canvas()
	{
		let can = Canvas::new(10, 20);
		assert_eq!(can.width, 10);
		assert_eq!(can.height, 20);
		for x in 0..can.width{
			for y in 0..can.height{
				assert_eq!(*can.color_at(x, y), Color::black())
			}
		}
	}
	#[test]
	fn	writing_to_canvas()
	{
		let mut can = Canvas::new(10, 20);
		let col = Color::new(1.0, 0.0, 0.0);
		can.write_pixel(2, 3, col);
		assert_eq!(*can.color_at(2, 3), col);
	}
	// #[test]
	// fn	constructing_ppm_header()
	// {
	// 	let can = Canvas::new(5, 3);
	// 	let ppm = String::canvas_to_ppm(can);

	// 	assert_eq!(ppm, "P3\n5 3\n255");
	// }
}