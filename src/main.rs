// https://hardmath123.github.io/scratch-mandelbrot.html

// include png output
// include complex numbers
use num::complex::Complex;
use ndarray::Array2;

fn main() {
    println!("Welcome to mandelbrot set display!");
    let zoom = 100;
    let zero = Complex{re: 10.0, im: 10.0};
    let size = Complex{re: 50, im: 50};
    let boundary = 50;

let mut image = Array2::<u32>::zeros((size.re, size.im));

for i in 0..image.rows() {
    for j in 0..image.cols(){
        image[[i,j]] = calculate_mandelbrot_pixel(Complex{re: i as f64 , im: j as f64} - zero, boundary);
        println!("item {}{} value: {}", i, j, image[[i,j]]);
    }
}

// create png from image array

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
