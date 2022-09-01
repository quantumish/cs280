
use experiments::naive::*;
use palette::{Lab, Hsv};

fn main () {
	let img = image::open("./image1.jpg").unwrap();

	// dim a copy of image
	let out = dim(img.clone(), 2).unwrap();
	out.save("./dim.png").unwrap();

	// gen greyscaled image
	let out = greyscale(img.clone()).unwrap();
	out.save("./grey.png").unwrap();

	// generate RGB exclusion images
	let out = rgb_restrict(img.clone(), 'R').unwrap();
	out.save("./rgb_r.png").unwrap();
	let out = rgb_restrict(img.clone(), 'G').unwrap();
	out.save("./rgb_g.png").unwrap();
	let out = rgb_restrict(img.clone(), 'B').unwrap();
	out.save("./rgb_b.png").unwrap();

	// generate LAB exclusion images
	let out = restrict::<Lab>(img.clone(), 'L').unwrap();
	out.save("./lab_l.png").unwrap();

	// produce image without L or A
	let out = restrict::<Lab>(out, 'A').unwrap();
	out.save("./lab_la.png").unwrap();	

	let out = restrict::<Lab>(img.clone(), 'A').unwrap();
	out.save("./lab_a.png").unwrap();
	let out = restrict::<Lab>(img.clone(), 'B').unwrap();
	out.save("./lab_b.png").unwrap();

	// generate HSV exclusion images
	let out = restrict::<Hsv>(img.clone(), 'H').unwrap();
	out.save("./hsv_h.png").unwrap();

	// produce image without H or S
	let out = restrict::<Hsv>(out, 'S').unwrap();
	out.save("./hsv_hs.png").unwrap();
	
	let out = restrict::<Hsv>(img.clone(), 'S').unwrap();
	out.save("./hsv_s.png").unwrap();
	let out = restrict::<Hsv>(img.clone(), 'V').unwrap();
	out.save("./hsv_v.png").unwrap();

	// open a second image and combine both
	let img2 = image::open("./image2.jpg").unwrap();
	let img2 = rgb_restrict(img2.clone(), 'G').unwrap();
	let img1 = rgb_restrict(img.clone(), 'B').unwrap();	
	let out = combine(img1, img2).unwrap();
	out.save("./combined.png").unwrap();

	// apply a custom quarter-based function
	let out = quarters(img.clone());
	out.save("./quarters.png").unwrap();	
}
