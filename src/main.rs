// https://hardmath123.github.io/scratch-mandelbrot.html

// include png output
// include complex numbers
use easy_complex::Complex;

fn main() {
    println!("Welcome to mandelbrot set display!");
    let zoom = 100;
    let zero = Complex{real: 100, imag: 100};
    let size = Complex{real: 500, imag: 500};
    let boundary = 50;

let mut image: [[f64; size.real()]; size.imag()];

// later implement first just in an array let mut image = Image.new("image.png")
// make image with resolution

for i in 0..image.x_size(){
    for j in 0..image.y_size(){
        //
        image(i,j) = calculate_mandelbrot_pixel(complex(i - zero.real()), complex(j - zero.imag()), boundary);
    }
}

// create png from image array

}

fn calculate_mandelbrot_pixel(location: Complex<f64>, boundary: u32 ) -> u32{
    let mut zn: Complex<f64> = Complex{real: 0.0, imag: 0.0 };
    let mut result = boundary;

let kk = Complex<f64>::powc(zn,33);
let ff = zn.powc(3);

    for i in 0..boundary {
        zn = location + Complex<f64>::powc(zn,2);
        if zn.real() > 2.0 {
            result = i;
            break;
        }
    }
    result
}
