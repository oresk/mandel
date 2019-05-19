// https://hardmath123.github.io/scratch-mandelbrot.html
#[macro_use]
extern crate clap;
// include png output
use std::env;
use std::convert::TryFrom;
use num::complex::Complex;
use clap::{Arg, ArgMatches, App, SubCommand};

fn main() {
    println!("Welcome to mandelbrot set display!");

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut image = vec![vec![0; usize::try_from(config.size.re).unwrap()]; usize::try_from(config.size.im).unwrap()];

    for i in 0..image.len() {
        for j in 0..image[i].len() {
            image[i][j] = calculate_mandelbrot_pixel((Complex{re: i as f64 , im: j as f64} - config.zero) / config.zoom, config.boundary);
            print!("{:3} ",image[i][j]);
        }
        println!("");
        println!("");
    }

// create png from image array

}

struct Config{
    zoom: Complex<f64>,
    zero: Complex<f64>,
    size: Complex<u32>,
    boundary: u32,
}

fn parse_config(args: &[String]) -> Config {
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

    let zoom = Complex{re: value_t!(matches, "ZOOM", f64).unwrap_or(10.0), im: 0.0};
    let zero = Complex{re: value_t!(matches, "ZEROx", f64).unwrap(), im: value_t!(matches, "ZEROy", f64).unwrap() };
    let size = Complex{re: value_t!(matches, "SIZEx", u32).unwrap(), im: value_t!(matches, "SIZEy", u32).unwrap() };
    let boundary = value_t!(matches, "BOUNDARY", u32).unwrap_or(50);

    Config { zoom, zero, size, boundary }
}

fn calculate_mandelbrot_pixel(location: Complex<f64>, boundary: u32 ) -> u32{
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
