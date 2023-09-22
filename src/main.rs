extern crate image;
extern crate clap;

use image::{GenericImageView, ImageBuffer, Rgb};
use clap::{App, Arg};

fn main() {
    let matches = App::new("Negative to Positive Image Converter")
        .author("Dylan Duhamel, duhadm19@alumni.wfu.edu")
        .version("1.0")
        .about("This CLI tool converts negative images intop positive images. The perfect tool for processing scanned film!")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input image file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output image file")
            .required(true)
            .index(2))
        .get_matches();

    let input_path = matches.value_of("INPUT").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap();

    convert_negative_to_positive(input_path, output_path);
}

fn convert_negative_to_positive(input_path: &str, output_path: &str) {
    // Open the image
    let img = image::open(input_path).expect("Failed to open image");

    // Output image buffer
    let mut output_image = ImageBuffer::new(img.width(), img.height());

    // Process each pixel
    for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
        let r = 255 - pixel[0];
        let g = 255 - pixel[1];
        let b = 255 - pixel[2];

        output_image.put_pixel(x, y, Rgb([r, g, b]));
    }

    output_image.save(output_path).expect("Failed to save image");
}