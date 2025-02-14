use image::{DynamicImage, ImageBuffer, RgbImage};
use fast_image_resize::{IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use fast_image_resize::images::Image;
use posterize_kmeans::posterize_kmeans;

use std::time::{SystemTime, UNIX_EPOCH};

#[path ="posterization/kmeans.rs"]
mod posterize_kmeans;

const E_TIME: &str = "Witamy z powrotem, towarzyszu Stalin";


pub fn posterize_image(image: &DynamicImage, k: usize) -> DynamicImage {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME);

    let img = image;
    // Load the image
    // let mut img = image::open(input_image_path).expect("Failed to open image");

    println!("Image opened in {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

    let dst_width = 320;
    let dst_height = 180;
    let mut dst_image = Image::new(
        dst_width,
        dst_height,
        img.pixel_type().unwrap(),
    );

    let options = ResizeOptions::new().resize_alg(ResizeAlg::Nearest);

    let mut resizer = Resizer::new();
    resizer.resize(image, &mut dst_image, &options).unwrap(); // resize image to dst_image

    println!("Image resized at {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

    // Convert the image into RGB pixels
    let rgb_image = RgbImage::from_raw(dst_width, dst_height, dst_image.buffer().to_vec()).expect("Conversion to RGB failed");

    println!("Image converted to rgb at {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

    let posterized_pixels = posterize_kmeans(img, rgb_image, k);

    println!("Posterization finished at {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

    let output_image: RgbImage = ImageBuffer::from_raw(dst_width, dst_height, posterized_pixels)
        .expect("Failed to create the image buffer");

        println!("Converted from buffer to image in {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

    return DynamicImage::ImageRgb8(output_image)
}