// https://hardmath123.github.io/scratch-mandelbrot.html

// include png output
// include complex numbers

fn main() {
    println!("Welcome to mandelbrot set display!");
    let zoom = 100;
    let zero = Complex<f32>.new(100, 100);
    let size = Complex<f32<.new(500,500);
    let boundary = 50

let image: Vec<Vec<i32>> = Vec::new();

// later implement first just in an array let mut image = Image.new("image.png")
// make image with resolution

for (i = 0; i < res.real(); i++){
    for (j = 0; j < res.imag(); j++){
        int image(i,j) = calculate_mandelbrot(complex(i - zero.real()), complex(j - zero.imag()), boundary);
    }
}

// create png from image array

}

fn calculate_mandelbrot(location: complex<f32>, boundary: u32 ){

}
