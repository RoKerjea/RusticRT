pub mod to_rgba32;
pub mod to_ppm;
pub mod to_png;

use std::vec::Vec;
use crate::color::Color;

// pub trait	Sized {
// 	fn	width(&self) -> usize;
// 	fn	height(&self) -> usize;
// }

#[derive(Debug, Clone)]
pub struct	Canvas{
	pub width : usize,
	pub height : usize,

	pixels: Vec<Color>,
}

pub trait  Sized
{
	fn width(&self) -> usize;
	fn height(&self) -> usize;
}

impl Sized for Canvas {
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self)-> usize {
		self.height
	}
}

impl Canvas{
	pub fn new(width: usize, height: usize) -> Self{
		Self {
			width,
			height,
			pixels: vec![Color::black(); width * height],
		}
	}
	pub	fn color_at(&self, x: usize, y:usize) -> Color {
		self.pixels[self.get_pixel_index(x, y)]
	}
	pub fn	get_pixel_index(&self, x: usize, y: usize) ->usize {
		y * self.width + x
	}
	pub fn	write_pixel(&mut self, x: usize, y: usize, color:Color){
		let index = self.get_pixel_index(x, y);
		self.pixels[index] = color;
	}
}

#[cfg(test)]
mod tests{
	use super::*;
	use super::to_ppm::ToPPM;
	#[test]
	fn	creating_canvas()
	{
		let can: Canvas = Canvas::new(10, 20);
		assert_eq!(can.width, 10);
		assert_eq!(can.height, 20);
		for x in 0..can.width{
			for y in 0..can.height{
				assert_eq!(can.color_at(x, y), Color::black())
			}
		}
	}
	#[test]
	fn	writing_to_canvas()
	{
		let mut can = Canvas::new(10, 20);
		let col = Color::new(1.0, 0.0, 0.0);
		can.write_pixel(2, 3, col);
		assert_eq!(can.color_at(2, 3), col);
	}
	#[test]
	fn	constructing_ppm_header()
	{
		let can: Canvas = Canvas::new(5, 3);
		let ppm_image = can.to_ppm();
		let actual_res = &ppm_image[..11];
		// let ppm = canvas_to_ppm(can);
		let expected = String::from("P3\n5 3\n255\n").into_bytes();

		assert_eq!(actual_res, expected);
	}
	#[test]
	fn	constructing_ppm_pixel_data()
	{
		let mut can = Canvas::new(5, 3);
		let col1 = Color::new(1.5, 0.0, 0.0);
		let col2 = Color::new(0.0, 0.5, 0.0);
		let col3 = Color::new(-0.5, 0.0, 1.0);
		can.write_pixel(0, 0, col1);
		can.write_pixel(2, 1, col2);
		can.write_pixel(4, 2, col3);
		let header = String::from("P3\n5 3\n255\n").into_bytes();
		let pixel_data = String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n").into_bytes();
		let mut expected : Vec<u8> = Vec::new();
		expected.extend(header);
		expected.extend(pixel_data);
		assert_eq!(can.to_ppm(), expected);
	}
	#[test]
	fn	splitting_long_lines_ppm_files()
	{
		let mut canvas = Canvas::new(10, 2);
		let color = Color::new(1.0, 0.8, 0.6);
		for x in 0..10
		{
			for y in 0..2
			{
				canvas.write_pixel(x, y, color);
			}
		}

		let actual_result = canvas.to_ppm();
		let header = String::from("P3\n10 2\n255\n").into_bytes();
		let pixel_data = String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n153 255 204 153 255 204 153 255 204 153 255 204 153\n").into_bytes();
		let mut expected : Vec<u8> = Vec::new();
		expected.extend(header);
		expected.extend(pixel_data);

		assert_eq!(actual_result, expected);
	}
}