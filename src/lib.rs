use image::DynamicImage;
use image::GenericImageView;
use std::arch::aarch64::*;
use rand::Rng;

pub mod utils;
pub mod naive;
pub mod optimized;

pub fn naive_greyscale(img: DynamicImage, out: &mut [u8]) -> Vec<u8> {
	let (w, h) = img.dimensions();
	let mut out: Vec<u8> = vec![0; (w*h) as usize];
	for (n, i) in img.pixels().enumerate() {
		let (_x, _y, vals) = (i.0, i.1, i.2.0);
		let sum = vals[0] as u16
			+ vals[1] as u16
			+ vals[2] as u16;
		out[n] = (sum/3) as u8;
	}
	out
}

pub fn dim(bytes: &[u8], out: &mut [u8]) {
	for (n, i) in bytes.iter().enumerate() {
		out[n] = i/3;
	}
}

pub fn exclude(bytes: &[u8], out: &mut [u8], index: usize) {
	for n in 0..bytes.len() {
		if n % 3 == index {
			out[n] = 0;
		}
	}
}

pub fn greyscale(bytes: &[u8], out: &mut [u8]) {
	for i in 0..bytes.len()/3 {
		let sum = bytes[i*3] as u16 + bytes[i*3 + 1] as u16 + bytes[i*3 + 2] as u16;
		out[i] = (sum/3) as u8;		
	}
}

pub fn slightly_bad_grey(bytes: &[u8], out: &mut [u8]) {
	for i in 0..bytes.len()/3 {
		out[i] = bytes[i*3]/3 + bytes[i*3 + 1]/3 + bytes[i*3 + 2]/3;		
	}
}


pub fn simd4_dim(bytes: &[u8], out: &mut [u8]) {
	let bytes: &[u32] = bytemuck::cast_slice::<u8, u32>(bytes);
	for (n, i) in bytes.iter().enumerate() {
		let inp = i.to_be_bytes();
		let inp = [inp[0] as f32, inp[1] as f32, inp[2] as f32, inp[3] as f32];
		let mut tmp = [0.0; 4];
		unsafe {
			let simd = vld1q_f32(inp.as_ptr());
			let simd = vmulq_n_f32(simd, 0.33);
			vst1q_f32(tmp.as_mut_ptr(), simd);
		}
		out[n*4] = tmp[0] as u8;
		out[n*4 + 1] = tmp[1] as u8;
		out[n*4 + 2] = tmp[2] as u8;
		out[n*4 + 3] = tmp[3] as u8;
	}
}

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
