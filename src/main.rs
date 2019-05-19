// https://hardmath123.github.io/scratch-mandelbrot.html

// include png output
use std::env;
use std::convert::TryFrom;
use num::complex::Complex;
use clap::{Arg, ArgMatches, App, SubCommand};

fn main() {
    println!("Welcome to mandelbrot set display!");

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let mut image = vec![vec![0; usize::try_from(config.size.re).unwrap()]; usize::try_from(config.size.im).unwrap()];

    for i in 0..config.size.re {
        for j in 0..config.size.im{
            image[i as usize][j as usize] = calculate_mandelbrot_pixel((Complex{re: i as f64 , im: j as f64} - config.zero) / config.zoom, config.boundary);
            print!("{} ",image[i as usize][j as usize]);
            //print!("|{} {}|",i,j);
        }
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
    let zoom = Complex{re: args[1].clone().parse().unwrap(), im: args[2].clone().parse().unwrap()};
    let zero = Complex{re: args[3].clone().parse().unwrap(), im: args[4].clone().parse().unwrap()};
    let size = Complex{re: args[5].clone().parse().unwrap(), im: args[6].clone().parse().unwrap()};
    let boundary = args[7].clone().parse::<u32>().unwrap();

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
