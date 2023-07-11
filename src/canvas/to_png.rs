use crate::canvas::to_rgba32::ToRGBA32;
use crate::canvas::Sized;

pub trait	ToPNG {
	fn	to_png(&self) -> Vec<u8>;
}

impl<T> ToPNG for ToPNG
where
	T: ToRGBA32,
	T: Sized,
{
	fn	to_png(&self) -> Vec<u8>{
		let mut v =Vec::new();
		let mut	encoder = png::Encoder::new(&mut v, self.width() as u32, self.height() as u32);
		encoder.set_color(png::BitDepth::Eight);
		let mut	writer = encoder.write_header().unwrap();
		writer.write_image_data(&self.to_rgba32()).unwrap();
		drop(writer);

		return v;
	}
}