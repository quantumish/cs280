use experiments::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, BenchmarkGroup, measurement::WallTime, PlotConfiguration, AxisScale};
use image::GenericImageView;

// fn bench_greys(c: &mut Criterion) {
// 	let plot_config = PlotConfiguration::default()
// 		.summary_scale(AxisScale::Logarithmic);
//     let mut group = c.benchmark_group("Grey");
// 	// group.plot_config(plot_config);
	
// 	for sz in 8..14 {
// 		let sz = 2_u32.pow(sz);
// 		let img = gen_image(sz);				
// 		group.bench_with_input(BenchmarkId::new("riley", sz), &img, |b, i| {
// 			let (w, h) = i.dimensions();
// 			let mut out: Vec<u8> = vec![0; (w*h) as usize];		
// 			let rgba = img.to_rgba8();
// 			let rgba_bytes = rgba.as_raw();
// 			b.iter(|| less_cringe_greyscale(&rgba_bytes[..], &mut out[..]))
// 		});
// 		group.bench_with_input(BenchmarkId::new("simd4", sz), &img, |b, i| {
// 			let (w, h) = i.dimensions();
// 			let mut out: Vec<u8> = vec![0; (w*h) as usize];		
// 			let bytes = i.as_bytes();
// 			b.iter(|| simd4_greyscale(&bytes[..], &mut out[..]))
// 		});
// 		group.bench_with_input(BenchmarkId::new("simd8", sz), &img, |b, i| {
// 			let (w, h) = i.dimensions();
// 			let mut out: Vec<u8> = vec![0; (w*h) as usize];		
// 			let bytes = i.as_bytes();
// 			b.iter(|| simd8_greyscale(&bytes[..], &mut out[..]))
// 		});
// 		group.bench_with_input(BenchmarkId::new("grey", sz), &img, |b, i| {
// 			let (w, h) = i.dimensions();
// 			let mut out: Vec<u8> = vec![0; (w*h) as usize];		
// 			let bytes = i.as_bytes();
// 			b.iter(|| greyscale(&bytes[..], &mut out[..]))
// 		});
// 		group.bench_with_input(BenchmarkId::new("kinda_bad_grey", sz), &img, |b, i| {
// 			let (w, h) = i.dimensions();
// 			let mut out: Vec<u8> = vec![0; (w*h) as usize];		
// 			let bytes = i.as_bytes();
// 			b.iter(|| slightly_bad_grey(&bytes[..], &mut out[..]))
// 		});
// 		// group.bench_with_input(BenchmarkId::new("naive", sz), &img, |b, i| b.iter(|| naive_greyscale(i.clone())));
// 		// group.bench_with_input(BenchmarkId::new("simd4", sz), (&bytes, &out), |b, i| {
// 		// 	b.iter(|| unsafe {simd4_greyscale(i.0.clone())});
// 		// });		
// 		// group.bench_with_input(BenchmarkId::new("simd8", sz), i, |b, i| b.iter(|| unsafe {simd8_greyscale(i.clone());}));
// 		// group.bench_with_input(BenchmarkId::new("image-rs", sz), i2, |b, i| b.iter(|| i.grayscale()));
// 	}
// 	group.finish()
// }

fn bench_dims(c: &mut Criterion) {
	let mut group = c.benchmark_group("Dimming");
	for sz in 8..14 {
		let sz = 2_u32.pow(sz);
		let img = utils::gen_image(sz);
		group.bench_with_input(BenchmarkId::new("dim", sz), &img, |b, i| {
			let (w, h) = i.dimensions();
			let mut out: Vec<u8> = vec![0; (w*h*3) as usize];		
			let bytes = i.as_bytes();
			b.iter(|| optimized::cpu::dim(&bytes[..], &mut out[..], black_box(2)))
		});
		group.bench_with_input(BenchmarkId::new("dim2", sz), &img, |b, i| {
			let (w, h) = i.dimensions();
			let mut out: Vec<u8> = vec![0; (w*h*3) as usize];		
			let bytes = i.as_bytes();
			b.iter(|| optimized::cpu::dim2(&bytes[..], &mut out[..]))
		});
	}
}

criterion_group!(benches, bench_dims);
criterion_main!(benches);
