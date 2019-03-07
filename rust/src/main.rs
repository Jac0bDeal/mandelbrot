extern crate clap;
extern crate num_complex;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use clap::{Arg, App};
use num_complex::Complex;

fn main() -> std::io::Result<()>{
    let matches = App::new("Mandelbrot Set Generator (Rust Implementation)")
        .version("0.1.0")
        .author("Jacob Deal <jacob.r.deal@gmail.com>")
        .about("Constructs a .ppm image of the Mandelbrot Set.")
        .arg(Arg::with_name("resolution")
            .short("p")
            .long("pixel-resolution")
            .takes_value(true)
            .help("The width and height of the image in number of pixels; defaults to 1200. Must be an unsigned integer."))
        .arg(Arg::with_name("filepath")
            .short("f")
            .long("filepath")
            .takes_value(true)
            .help("The filepath for the generated image. Defaults to './mandelbrot.ppm'"))
        .arg(Arg::with_name("radius")
            .short("r")
            .long("radius")
            .takes_value(true)
            .help("The radius of the circle to display; defaults to 2.0. Must be a float less than length 32. Yes, I know it displays a square: Kill me :P"))
        .get_matches();

    let filepath: &str;
    match matches.value_of("filepath") {
        None => filepath = "mandelbrot.ppm",
        Some(s) => filepath = s,
    }
    let resolution: u16;
    match matches.value_of("resolution") {
        None => resolution = 1200,
        Some(s) => {
            match s.parse::<u16>() {
                Ok(n) => resolution = n,
                Err(_) => panic!("{} is not an unsigned integer less than length 16!", s),
            }
        }
    }
        let radius: f32;
    match matches.value_of("radius") {
        None => radius = 2.0,
        Some(s) => {
            match s.parse::<f32>() {
                Ok(n) => radius = n,
                Err(_) => panic!("{} is not a float less than length 32!", s),
            }
        }
    }

    let path = Path::new(filepath);
    let mut file = File::create(&path)?;

    let _ = file.write_all(format!("P3\n{} {} 255\n", resolution, resolution).as_bytes())?;

    for y in 0..resolution {
        for x in 0..resolution {
            let real = (2.0*(x as f32)/(resolution as f32)-1.0)*radius;
            let imag = (2.0*(y as f32)/(resolution as f32)-1.0)*radius;
            let c = Complex::new(real, imag);

            let val = get_color_value(c);

            let _ = file.write_all(format!("{} {} {}\n", val, 0, 0).as_bytes())?;
        }
    }

    Ok(())
}

fn get_color_value(c: num_complex::Complex<f32>) -> u8 {
    let mut z: Complex<f32> = Complex::new(0.0, 0.0);

    let mut iter: u16 = 0;
    while z.norm() < 2.0 && iter <= 34 {
        z = &z*&z + &c;
        iter += 1;
    }

    let color_value: u8;
    if iter < 34 {
        color_value = ((255 * iter) / 33) as u8;
    } else {
        color_value = 0;
    }

    return color_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_color_value_inside_set() {
        let val = get_color_value(Complex::new(-1.0, 0.0));
        assert_eq!(0, val);
    }

    #[test]
    fn test_get_color_value_outside_set() {
        let val = get_color_value(Complex::new(4.0, 4.0));
        assert_eq!(7, val);
    }

    #[test]
    fn test_get_color_value_escapes_set() {
        let val = get_color_value(Complex::new(1.0, 1.0));
        assert_eq!(15, val);
    }
}
