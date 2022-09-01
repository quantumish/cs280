pub mod cpu {
	use std::arch::aarch64::*;
	
	///
	/// TODO Write the rest of this docstring
	/// Algorithm is inspired by
	/// http://ftp.cvut.cz/kernel/people/geoff/cell/ps3-linux-docs/CellProgrammingTutorial/BasicsOfSIMDProgramming.html
	///
	pub fn simd4_greyscale(buf: &[u8], out: &mut [u8]) {
		for i in 0..buf.len()/12 {
			let off = i*12;
			let r = [buf[off + 0] as f32, buf[off + 3] as f32, buf[off + 6] as f32, buf[off + 9] as f32];
			let g = [buf[off + 1] as f32, buf[off + 4] as f32, buf[off + 7] as f32, buf[off + 10] as f32];
			let b = [buf[off + 2] as f32, buf[off + 5] as f32, buf[off + 8] as f32, buf[off + 11] as f32];
			let mut ret = [0.0; 4];
			unsafe {
				let r_simd = vld1q_f32(r.as_ptr());
				let g_simd = vld1q_f32(g.as_ptr());
				let b_simd = vld1q_f32(b.as_ptr());
				let r_simd = vmulq_n_f32(r_simd, 0.33);
				let r_simd = vmlaq_n_f32(g_simd, r_simd, 0.33);
				let r_simd = vmlaq_n_f32(b_simd, r_simd, 0.33);				
				vst1q_f32(ret.as_mut_ptr(), r_simd);
			}
			for j in 0..4 {
				out[i*4 + j] = ret[j] as u8;
			}
		}
	}

	pub fn simd8_greyscale(buf: &[u8], out: &mut [u8]) {
		let (mut r, mut g, mut b) = ([0; 8], [0; 8], [0; 8]);
		let ptr = out.as_mut_ptr();
		for i in 0..buf.len()/24 {
			let off = i*24;
			r = [buf[off+0], buf[off+3], buf[off+6], buf[off+ 9], buf[off+12], buf[off+15], buf[off+18], buf[off+21]];
			g = [buf[off+1], buf[off+4], buf[off+7], buf[off+10], buf[off+13], buf[off+16], buf[off+19], buf[off+22]];
			b = [buf[off+2], buf[off+5], buf[off+8], buf[off+11], buf[off+14], buf[off+17], buf[off+20], buf[off+23]];
			unsafe {
				let r_simd = vld1_u8(r.as_ptr());
				let g_simd = vld1_u8(g.as_ptr());
				let b_simd = vld1_u8(b.as_ptr());
				let res = vadd_u8(
					vadd_u8(
						vshr_n_u8(r_simd, 2),
						vshr_n_u8(g_simd, 2)
					),
					vshr_n_u8(b_simd, 2)
				);
				vst1_u8(ptr.add(i*8), res);
			}
		}
	}

	pub fn dim(bytes: &[u8], out: &mut [u8], factor: u8) {
		for (n, i) in bytes.iter().enumerate() {
			out[n] = i/factor;
		}
	}

	pub fn dim2(bytes: &[u8], out: &mut [u8]) {
		for (n, i) in bytes.iter().enumerate() {
			out[n] = i/2;
		}
	}

	pub fn exclude(bytes: &[u8], out: &mut [u8], index: usize) {
		for n in 0..bytes.len() {
			if n % 3 == index {
				out[n] = 0;
			}
		}
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

}

// mod gpu() 
