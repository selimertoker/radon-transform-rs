use image::{Rgb, Rgba, ImageReader, RgbImage, Rgba32FImage, DynamicImage};
use std::env;
use libm::{sin,cos,sqrt};
use progression::Bar;

fn main() {

    let args:Vec<String> = env::args().collect();
    if args.len()<3 {
        eprintln!("Usage: iRadon in.ppm out.ppm");
        eprintln!("Calculates Inverse Radon transform by overlaying projections of the original image");
        return;
    }

    let in_image:RgbImage = ImageReader::open(&args[1]).unwrap().decode().unwrap().into_rgb8();
    let maxdegree = in_image.width();

    let rad:u32 = in_image.height()/2;
    let mut out_image = Rgba32FImage::new(2*rad, 2*rad);

    let bar:Option<Bar> = if env::var("NO_BAR").is_ok() { None } else { Some(Bar::new(maxdegree as u64, progression::Config { prefix: "(items) ", ..progression::Config::cargo() } ) ) };

    for degree in 0..maxdegree {
        let sind = sin((degree as f64)*3.14159265/180.0);
        let cosd = cos((degree as f64)*3.14159265/180.0);

        for rof in -(rad as i32)..(rad as i32) {
            let colorofline:&Rgb<u8> = in_image.get_pixel(degree, (rof + rad as i32) as u32);
            let maxrac:u32 = sqrt((rad*rad) as f64 - (rof*rof) as f64) as u32;

            for rac in -(maxrac as i32)..(maxrac as i32) {
                let coords:[u32;2] = [
                    ((rad as f64)-(rof as f64)*cosd-(rac as f64)*sind) as u32,
                    ((rad as f64)+(rac as f64)*cosd-(rof as f64)*sind) as u32];

                if let Some(color) = out_image.get_pixel_mut_checked(coords[0], coords[1]) {
                    color[0] += colorofline[0] as f32;
                    color[1] += colorofline[1] as f32;
                    color[2] += colorofline[2] as f32;
                    color[3] += 1.0;
                }
            }
        }
        if let Some(ref bar_some) = bar { bar_some.inc(1); }
    }
    for h in 0..out_image.height() { 
        for w in 0..out_image.width() { 
            let color:&mut Rgba<f32> = out_image.get_pixel_mut(w, h);
            color[0] = color[0]/(255.0*color[3]+1.0);
            color[1] = color[1]/(255.0*color[3]+1.0);
            color[2] = color[2]/(255.0*color[3]+1.0);
            color[3] = 1.0;
        }

    }
    DynamicImage::ImageRgba32F(out_image).into_rgb8().save(&args[2]).unwrap();
    if let Some(bar_some) = bar { bar_some.finish(); }
}
