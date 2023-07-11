use crate::canvas::to_rgba32::ToRGBA32;
use crate::canvas::Sized;

pub trait ToPPM {
	pub fn	create_ppm_header(&self) -> Vec<u8>
	where
		Self:Sized,
	{
		let mut header = Vec::new();
		header.extend(String::from("P3\n").into_bytes());
		header.extend(format!("{} {}\n", self.width, self.height).into_bytes());
		header.extend(format!("{}\n", 255).into_bytes());
		return header;
	}
	fn	to_ppm(&self) -> Vec<u8>;
}

//Oh my god! It's even worse than before...
//I don't doubt it's efficient and do precisely what's needed,
//but it's also completly unreadble.
//At the very least i need to extract methods and name them correctly
impl<T> ToPPM for T
where
	T: ToRGBA32,
	T:Sized,
{
	fn	to_ppm(&self) -> Vec<u8>
	{
		let mut last_image_row: usize = 0;
		let mut	column_count: usize = 0;
		let pixel_data = self
		.to_rgba32()
		.into_iter()
		.map(|byte| format!("{}", byye))
		.enumerate()
		.filter(|(i, _)| (i +1) % 4 != 0)
		.enumerate()
		.flat_map(|(i, (_, pixel_string))| {
			let mut data: Vec<u8> = Vec::new();
			let current_img_row = i/ (self/width() * 3);
			if current_img_row != last_image_row {
				last_image_row = current_img_row;
				data.extend(String::from("\n").into_bytes());
				column_count = 0;
			}
			let mut needed_space: usize = 0;
			if column_count != 0 {
				needed_space += 1;
			}
			needed_space += pixel_string.len();

			if column_count + needed_space > 70 {
				data.extend(String::from("\n").into_bytes());
				column_count = 0;
			}

			if column_count != 0 {
				data.extend(String::from(" ").into_bytes());
				column_count += 1;
			}

			data.extend(pixel_string.clone().into_bytes());
			column_count += pixel_string.len();

			data
		});
		self
			.create_ppm_header()
			.into_iter()
			.chain(pixel_data)
			.chain(String::from("\n").into_bytes())
			.collect()
		// let header = self.create_ppm_header();
		// let pixel_data = self.create_ppm_pixel_data();
		// let mut ppm = Vec::new();
		// ppm.extend(header);
		// ppm.extend(pixel_data);
		// return ppm;
	}
}

//This is quite litterally the worst code i wrote in more than a year
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