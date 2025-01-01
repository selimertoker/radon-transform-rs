use image::{Rgb, ImageReader, RgbImage, ImageFormat};
use std::{env, cmp};
use libm::{sin,cos,sqrt};
use progression::Bar;

const BLACKPIXEL:Rgb<u8> = Rgb{0:[0;3]};

fn main() {
        let args:Vec<String>=env::args().collect();
	if args.len()<3 {
		eprintln!("Usage: Radon in.ppm out.ppm <degree>");
		eprintln!("Radon transform is calculated for 0 to [degree) degrees, degree=180 if not specified");
                return;
	}
	let maxdegree:u32 = if args.len()==4 { args[3].parse::<u32>().unwrap() } else { 180 };

        let in_image:RgbImage = ImageReader::open(&args[1]).unwrap().decode().unwrap().into_rgb8();

	let rad:u32 = cmp::min(in_image.width(), in_image.height())/2;
        let mut out_image = RgbImage::new(maxdegree, 2*rad as u32);

        let bar:Option<Bar> = if env::var("NO_BAR").is_ok() { None } else {Some(Bar::new(maxdegree as u64, progression::Config { prefix: "(items) ", ..progression::Config::cargo() } ) ) };

	for degree in 0..maxdegree {
		let sind=sin((degree as f64)*3.14159265/180.0);
		let cosd=cos((degree as f64)*3.14159265/180.0);

		for rof in -(rad as i32)..(rad as i32) {
                        let mut colorofline:[u32;3]=[0;3];
			let maxrac:u32 = sqrt((rad*rad) as f64 - (rof*rof) as f64) as u32;

			for rac in -(maxrac as i32)..(maxrac as i32) {
                                let coords:[u32;2] = [
                                    rad-(((rof as f64)*cosd-(rac as f64)*sind) as u32),
                                    rad+(((rac as f64)*cosd-(rof as f64)*sind) as u32)];
				let color:&Rgb<u8>=in_image.get_pixel_checked(coords[0], coords[1]).unwrap_or(&BLACKPIXEL);
				colorofline[0] += color[0] as u32;
                                colorofline[1] += color[1] as u32;
                                colorofline[2] += color[2] as u32;
			}
			let color:&mut Rgb<u8> = out_image.get_pixel_mut(degree,(rof + rad as i32) as u32);
                        color[0] = (colorofline[0]/(2*maxrac+1)) as u8;
                        color[1] = (colorofline[1]/(2*maxrac+1)) as u8;
                        color[2] = (colorofline[2]/(2*maxrac+1)) as u8;
		}
                if let Some(ref bar_some)=bar { bar_some.inc(1); }
	}
	out_image.save_with_format(&args[2],ImageFormat::Pnm).unwrap();
        if let Some(bar_some)=bar { bar_some.finish(); }
}
