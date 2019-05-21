// https://hardmath123.github.io/scratch-mandelbrot.html
#[macro_use]
extern crate clap;
// include png output
use std::convert::TryFrom;
use num::complex::Complex;
use clap::{Arg, App};
// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use png::HasParameters;

fn main() {
    println!("Welcome to mandelbrot set display!");

    let config = parse_config();

    let mut image = vec![vec![0; usize::try_from(config.size.re).unwrap()]; usize::try_from(config.size.im).unwrap()];

    for i in 0..image.len() {
        for j in 0..image[i].len() {
            image[i][j] = calculate_mandelbrot_pixel((Complex{re: i as f64 , im: j as f64} - config.zero) / config.zoom, config.boundary);
            //print!("{:3} ",image[i][j]);
        }
        //println!("");
        //println!("");
    }

// create png from image array

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, config.size.re, config.size.im); 
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
     let mut writer = encoder.write_header().unwrap();

    let mut data: Vec<u8> = vec![255; usize::try_from(4 * config.size.re * config.size.im).unwrap()];
    
    for i in 0..image.len(){
        for j in 0..image[i].len() {
            //data[2 + 4 * i * j] = image[i][j] as u8;
            //data[3 + 4 * i * j] = 255; 
            data[0 + 4 * i * j] = 0u8;
            data[1 + 4 * i * j] = 0u8;
            data[2 + 4 * i * j] = 0u8;
        }
    }
    
    writer.write_image_data(&data).unwrap(); // Save
}

struct Config{
    zoom: Complex<f64>,
    zero: Complex<f64>,
    size: Complex<u32>,
    boundary: u32,
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

    let zoom = Complex{re: value_t!(matches, "ZOOM", f64).unwrap_or(10.0), im: 0.0};
    let zero = Complex{re: value_t!(matches, "ZEROx", f64).unwrap(), im: value_t!(matches, "ZEROy", f64).unwrap() };
    let size = Complex{re: value_t!(matches, "SIZEx", u32).unwrap(), im: value_t!(matches, "SIZEy", u32).unwrap() };
    let boundary = value_t!(matches, "BOUNDARY", u32).unwrap_or(50);

    Config { zoom, zero, size, boundary }
}
