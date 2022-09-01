use crate::utils;
use image::{GenericImageView, ImageBuffer};
use image::{Rgba, DynamicImage};
use anyhow::{Result, anyhow};
use palette::{Srgb, Lab, Hsv, IntoColor, FromColor, RgbHue};

pub fn greyscale(img: DynamicImage) -> Result<DynamicImage> {	
	let (w, h) = img.dimensions();
	let mut out: Vec<u8> = vec![0; (w*h) as usize];	
	for (n, i) in img.pixels().enumerate() {
		let vals = i.2.0;
		let sum = vals[0] as u16
			+ vals[1] as u16
			+ vals[2] as u16;
		out[n] = (sum/3) as u8;
	}
	let buf = ImageBuffer::from_vec(w, h, out)
		.ok_or(anyhow!("Couldn't convert buffer."))?;
	Ok(DynamicImage::ImageLuma8(buf))	
}

pub fn dim(img: DynamicImage, factor: u8) -> Result<DynamicImage> {
	let (w, h) = img.dimensions();
	let mut out: Vec<u8> = Vec::with_capacity((w*h*3) as usize);
	out.extend(img.as_bytes().iter().map(|i| i/factor));	
	let buf = ImageBuffer::from_vec(w, h, out)
		.ok_or(anyhow!("Couldn't convert buffer."))?;
	Ok(DynamicImage::ImageRgb8(buf))
}

pub fn rgb_restrict(img: DynamicImage, channel: char) -> Result<DynamicImage> {
	let (w, h) = img.dimensions();
	let mut out: Vec<u8> = Vec::with_capacity((w*h*3) as usize);	
	let index = match channel {
		'R' => 0,
		'G' => 1,
		'B' => 2,
		_ => { return Err(anyhow!("Invalid color channel")); }
	};
	out.extend(&mut
		img.as_bytes()
			.iter()
			.enumerate()
			.map(|(n,i)| if n % 3 == index { 0 } else { *i })
	);
	let buf = ImageBuffer::from_vec(w, h, out)
		.ok_or(anyhow!("Couldn't convert buffer"))?;
	Ok(DynamicImage::ImageRgb8(buf))
}

pub trait ColorChannel: IntoColor<Srgb> + FromColor<Srgb> {
	fn zero_channel(&mut self, channel: char) -> Result<()>;
}

impl ColorChannel for Lab {
	fn zero_channel(&mut self, channel: char) -> Result<()> {
		match channel.to_ascii_uppercase() {
			'L' => self.l = 0.0,
			'A' => self.a = 0.0,
			'B' => self.b = 0.0,
			_ => { return Err(anyhow!("Invalid color channel")); }
		}
		Ok(())
	}
}

impl ColorChannel for Hsv {
	fn zero_channel(&mut self, channel: char) -> Result<()> {
		match channel.to_ascii_uppercase() {
			'H' => self.hue = RgbHue::from_degrees(0.0),
			'S' => self.saturation = 0.0,
			'V' => self.value = 0.0,
			_ => { return Err(anyhow!("Invalid color channel")); }
		}
		Ok(())
	}
}

pub fn restrict<Space: ColorChannel>(img: DynamicImage, channel: char) -> Result<DynamicImage> {
	let (w, h) = img.dimensions();
	let mut out: Vec<u8> = Vec::with_capacity((w*h*3) as usize);
	for i in img.pixels() {
		let rgb = Srgb::from_components(
			(i.2.0[0], i.2.0[1], i.2.0[2])			
		).into_format::<f32>();
		let mut color: Space = rgb.into_color();
		color.zero_channel(channel)?;
		let orig: Srgb = color.into_color();
		let orig = orig.into_format::<u8>().into_components();
		out.extend([orig.0, orig.1, orig.2]);
	}	
	let buf = ImageBuffer::from_vec(w, h, out)
		.ok_or(anyhow!("Couldn't convert buffer"))?;
	Ok(DynamicImage::ImageRgb8(buf))
}


pub fn combine(img: DynamicImage, img2: DynamicImage) -> Result<DynamicImage> {
	if img.dimensions() != img2.dimensions() {
		return Err(anyhow!("Images not same size"));
	}
	let (w, h) = img.dimensions();
	let img = ImageBuffer::from_fn(w, h, |x, y| {
		if x > w/2 {			
			img2.get_pixel(x, y)
		} else {
			img.get_pixel(x, y)			
		}
	});
	Ok(DynamicImage::ImageRgba8(img))
}


pub fn quarters(img: DynamicImage) -> DynamicImage {
	let (w, h) = img.dimensions();
	let img = ImageBuffer::from_fn(w, h, |x, y| {
		let orig = img.get_pixel(x, y);
		if x > w/2 && y > h/2 {			
			Rgba::<u8>([orig[0].checked_mul(2).unwrap_or(255), orig[1]/2, orig[2], orig[3]])
		} else if x > w/2 && y < h/2 {
			let sum = orig[0] as u16 + orig[1] as u16 + orig[2] as u16;
			let avg = (sum/3) as u8;
			Rgba::<u8>([avg, avg, avg, orig[3]])
		} else if x < w/2 && y > h/2 {
			Rgba::<u8>([orig[2], orig[1], orig[0], orig[3]])
		} else {
			Rgba::<u8>([orig[0], orig[1], orig[2], orig[3]/4])
		}
	});
	DynamicImage::ImageRgba8(img)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn dim() {
		let img = utils::gen_image(64);
		let out = super::dim(img.clone(), 2).unwrap();
		assert_eq!(out.get_pixel(0,0)[0], img.get_pixel(0,0)[0]/2);
	}
}
	
