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
	pub fn	create_ppm_header(&self) -> Vec<u8>{
		let mut header = Vec::new();
		header.extend(String::from("P3\n").into_bytes());
		header.extend(format!("{} {}\n", self.width, self.height).into_bytes());
		header.extend(format!("{}\n", 255).into_bytes());
		return header;
	}

	//This is quite litterally the worst code i wrote in more han a year
	//If the video doesn't refactor it really heavily, I HAVE to do it!!!
	pub fn	create_ppm_pixel_data(&self) -> Vec<u8>{
		let mut pixel_strings : Vec <String> = Vec::new();
		for pixel in self.pixels.iter()
		{
			let clamped_color = pixel.clamp(0.0, 1.0);
			let r: u8 = (clamped_color.red * 255.0).round() as u8;
			let g: u8 = (clamped_color.green * 255.0).round() as u8;
			let b: u8 = (clamped_color.blue * 255.0).round() as u8;
			pixel_strings.push(format!("{}", r));
			pixel_strings.push(format!("{}", g));
			pixel_strings.push(format!("{}", b));
		}
		let mut pixel_data : Vec<u8> = Vec::new();
		let mut colum_count: usize = 0;
		let mut last_image_row: usize = 0;
		for (i, pixel_string) in pixel_strings.iter().enumerate()
		{
			//Line break for each row
			let current_img_row = i / (self.width * 3);
			if current_img_row != last_image_row
			{
				last_image_row = current_img_row;
				pixel_data.extend(String::from("\n").into_bytes());
				colum_count = 0;
			}
			let mut needed_space: usize = 0;
			if colum_count != 0
			{
				needed_space += 1;
			}
			needed_space += pixel_string.len();

			//Do not exceed 70 char per line
			if colum_count + needed_space > 70
			{
				pixel_data.extend(String::from("\n").into_bytes());
				colum_count = 0;
			}
			if colum_count != 0
			{
				pixel_data.extend(String::from(" ").into_bytes());
				colum_count += 1;
			}
			pixel_data.extend(pixel_string.clone().into_bytes());
			colum_count += pixel_string.len();
		}
		//last linebreak at the end of data
		pixel_data.extend(String::from("\n").into_bytes());
		return pixel_data;
	}
}

pub trait ToPPM {
	fn	to_ppm(&self) -> Vec<u8>;
}

impl ToPPM for Canvas {
	fn	to_ppm(&self) -> Vec<u8> {
		let header = self.create_ppm_header();
		let pixel_data = self.create_ppm_pixel_data();
		let mut ppm = Vec::new();
		ppm.extend(header);
		ppm.extend(pixel_data);
		return ppm;
	}
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
		let can = Canvas::new(5, 3);
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