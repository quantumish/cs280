use image::DynamicImage;
use image::GenericImageView;
use rand::Rng;

pub mod utils;
pub mod naive;
pub mod optimized;

// #[cfg(test)]
// mod tests {
// 	use image::RgbaImage;
// 	use super::*;

// 	// #[test]
// 	// fn it_works() {
// 	//	let img = image::open("./astro.jpg").unwrap();
// 	//	let (w, h) = img.dimensions();
// 	//	let out = unsafe { simd4_greyscale(img) };
// 	//	image::save_buffer("./ohgod2.png", out.as_slice(), w, h, image::ColorType::L8).unwrap();
// 	// }

// 	}

// 	// #[test]
// 	// fn naive_rgb_restrict() {
// 	//	let img = image::open("./astro.jpg").unwrap();
// 	//	let out = naive::rgb_restrict(img.clone(), 'R').unwrap();
// 	//	out.save("./test1.jpg").unwrap();
// 	//	let out = naive::rgb_restrict(img.clone(), 'G').unwrap();
// 	//	out.save("./test2.jpg").unwrap();
// 	//	let out = naive::rgb_restrict(img, 'B').unwrap();
// 	//	out.save("./test3.jpg").unwrap();
// 	// }

// 	// #[test]
// 	// fn naive_lab_restrict() {
// 	//	use palette::{Lab, Hsv};
// 	//	let img = image::open("./index.png").unwrap();
// 	//	let out = naive::restrict::<Lab>(img.clone(), 'L').unwrap();
// 	//	out.save("./labtest_l.png").unwrap();
// 	//	let out = naive::restrict::<Hsv>(img.clone(), 'H').unwrap();
// 	//	out.save("./hsvtest_v.png").unwrap();
// 	// }

// 	#[test]
// 	fn naive_combine() {
// 		let img = image::open("./index.png").unwrap();
// 		let img2 = image::open("./hsvtest_v.png").unwrap();
// 		let out = naive::combine(img, img2).unwrap();
// 		out.save("./both.png").unwrap();
// 	}

// 	#[test]
// 	fn naive_quarters() {
// 		let img = image::open("./index.png").unwrap();
// 		let out = naive::quarters(img);
// 		out.save("./quarters.png").unwrap();
// 	}

// 	#[test]
// 	fn optim_test() { 
// 		let i = image::open("./index.png").unwrap();
// 		let i = i.to_rgb8();
// 		let (w, h) = i.dimensions();
// 		let mut out: Vec<u8> = vec![0; (w*h) as usize];
// 		let bytes = i.as_bytes();
// 		simd8_riley(&bytes[..], &mut out[..]);
// 		image::save_buffer("./ohgod2.png", out.as_slice(), w, h, image::ColorType::L8).unwrap();
// 	}
// }
