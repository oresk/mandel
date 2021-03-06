// https://hardmath123.github.io/scratch-mandelbrot.html
#[macro_use]
extern crate clap;
use clap::{Arg, App};
use num::complex::Complex;
use image;

fn main() {
    println!("Welcome to mandelbrot set display!");

    let config = parse_config();

    let img = image::ImageBuffer::from_fn(config.size.re, config.size.im, |x, y|
        image::Rgb([0, calculate_mandelbrot_pixel((Complex{re: x as f64, im: y as f64} - config.zero) / config.zoom, config.boundary), 0]));

    img.save("fractal.png").unwrap();
}

struct Config{
    zoom: Complex<f64>,
    zero: Complex<f64>,
    size: Complex<u32>,
    boundary: u8,
}

fn calculate_mandelbrot_pixel(location: Complex<f64>, boundary: u8 ) -> u8{
    let mut zn = Complex{re: 0.0, im: 0.0 };
    let mut result = boundary;

    for i in 0..boundary {
        zn = zn.powf(2.0) + location;
        if zn.re > 2.0 {
            result = i;
            break;
        }
    }
    result
}

fn parse_config() -> Config {
    let matches = App::new("Mandelbrot generator")
                          .version("0.1.0")
                          .author("Lovro Oreskovic <lovro@oreskovic.me>")
                          .about("Generates ascii representation of mandelbrot set")
                          .arg(Arg::with_name("ZEROx")
                               .help("Sets the x zero location.")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("ZEROy")
                               .help("Sets the y zero location.")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("SIZEx")
                               .help("Sets the x size of the output.")
                               .required(true)
                               .index(3))
                          .arg(Arg::with_name("SIZEy")
                               .help("Sets the y size of the output.")
                               .required(true)
                               .index(4))
                          .arg(Arg::with_name("ZOOM")
                               .help("Sets the zoom of the images. By default set to 10.")
                               .index(5))
                          .arg(Arg::with_name("BOUNDARY")
                               .help("Sets the boundary of the calculation. By default set to 50.")
                               .index(6))
                          .get_matches();

    let zero = Complex{re: value_t!(matches, "ZEROx", f64).unwrap(), im: value_t!(matches, "ZEROy", f64).unwrap() };
    let size = Complex{re: value_t!(matches, "SIZEx", u32).unwrap(), im: value_t!(matches, "SIZEy", u32).unwrap() };
    let zoom = Complex{re: value_t!(matches, "ZOOM", f64).unwrap_or(size.re as f64), im: 0.0};
    let boundary = value_t!(matches, "BOUNDARY", u8).unwrap_or(50);

    Config { zoom, zero, size, boundary }
}
