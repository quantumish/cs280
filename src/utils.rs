use image::{ImageBuffer, DynamicImage};
use rand::Rng;

pub fn gen_image(sz: u32) -> DynamicImage {
	let img = ImageBuffer::from_fn(sz, sz, |_x, _y| {
		let mut rng = rand::thread_rng();
		image::Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()])
	});
	DynamicImage::ImageRgb8(img)
}
