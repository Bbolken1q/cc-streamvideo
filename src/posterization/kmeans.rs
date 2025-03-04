use fast_image_resize::{images::Image, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
mod find_distance;
#[allow(unused_imports)]
use find_distance::{quantize_colors, get_position};

use image::{DynamicImage, RgbImage};
#[allow(unused_imports)]
use kiddo::immutable::float::kdtree::ImmutableKdTree;
use kmeans_colors::get_kmeans;
use palette::rgb::Rgb;

#[allow(unused_imports)]
use crate::qpixel::qpixel::Qpixel;

mod sorting;
#[allow(unused_imports)]
use sorting::{lightness_linear, lightness_unweighted, lightness, hue, hilbert_wrapper, hilbert_lookup};

#[path="../sort.rs"]
mod sort;
use sort::merge_sort;

pub fn posterize_kmeans(image: &DynamicImage, rgb_image: RgbImage, k: usize) -> (Vec<Rgb>, Vec<Qpixel>) {  //pixels: &Vec<Rgb>
    /*
        Base implementation, ~230ms for clustering and posterization 
    */

    let img = image;

    let dst_width = 32;
    let dst_height = 18;
    let mut dst_image = Image::new(
        dst_width,
        dst_height,
        img.pixel_type().unwrap()
    );

    let options = ResizeOptions::new().resize_alg(ResizeAlg::Nearest);

    let mut resizer = Resizer::new();
    resizer.resize(img, &mut dst_image, &options).unwrap(); // resize image to dst_image

    let resized_image = RgbImage::from_raw(dst_width, dst_height, dst_image.buffer().to_vec()).expect("Conversion to RGB failed");

    // let pixels: Vec<Rgb> = resized_image.pixels()
    // .map(|p| Rgb::new(p[0] as f32, p[1] as f32, p[2] as f32))
    // .collect();

    let pixels: Vec<Rgb> = resized_image.pixels()
    .map(|p| Rgb::new(p[0] as f32, p[1] as f32, p[2] as f32))
    .collect();

    // let k_num = k; // Number of clusters
    let max_iter = 2;
    let converge = 0.1;
    let verbose = false;
    let seed = 42;

    let result = get_kmeans(
        k,
        max_iter,
        converge,
        verbose,
        &pixels,
        seed
    );

    let sorting = &hilbert_lookup;

    let centroids = result.centroids;
    // println!("{}", centroids.len());
    // let indices = result.indices;

    let mut posterized_pixels:Vec<Rgb> = Vec::with_capacity(rgb_image.len()/3);

    let sorted_centroids = merge_sort(&quantize_colors(&centroids, sorting));

    let mut hues:Vec<f32> = Vec::with_capacity(k+1);

    for color in &sorted_centroids {
        hues.push(color.hue);
    }

    // #[allow(unused_variables)]
    // let hues_slice: &[[f32; 1]] = unsafe {
    //     std::slice::from_raw_parts(hues.as_ptr() as *const [f32; 1], hues.len())
    // };

    // for pixel in 0..rgb_image.enumerate_pixels().len() * 3 {
    //     hue(&mut [black_box(pixel as f32), black_box(pixel as f32), black_box(pixel as f32)]));
    // }


    // let mut biggest_distance: f32 = 0.0;
    // for i in 0..14  {
    //     let cdist = sc[i+1].hue - sc[i].hue;
    //     if cdist > biggest_distance {
    //         biggest_distance = cdist;
    //     } 
    // }

    // biggest_distance = biggest_distance / 2.0;

    // println!("{:?}", sc);

    let mut vec_centroids: Vec<[f32; 3]> = Vec::new();

    for (_, pixel) in sorted_centroids.iter().enumerate() {
        let color = pixel;
        let value = [color.color.red as f32, color.color.green as f32, color.color.blue as f32];
        vec_centroids.push(value);
    }

    // let kdtree: ImmutableKdTree::<f32, u8, 1, 15> = ImmutableKdTree::new_from_slice(hues_slice);
    
     for (_x, _y, pixel) in rgb_image.enumerate_pixels() {
        // let query = Qpixel::new(Rgb::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32), &lightness);
        // let nearest_t = kdtree.nearest_n_within::<HueSorting>(&query, 0.3, NonZero::new(1 as usize).unwrap(), true);
        // let nearest = kdtree.nearest_one::<HueSorting>(&[query.hue; 1]);
        // let nearestsqe = NearestDistanceSquaredEuclidean(QPixel{color: Rgb::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32), hue: hue(&mut [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32])}, &sorted_centroids);hue(&mut [pixel[0] as f32, pixel[1] as f32, pixel[2] as f32])
        // println!("{}, rgb: {:?}", nearest, sc[nearest as usize]);
        let nearestbts = get_position(&hues, sorting(&Rgb::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32)));

        // println!("{}, {}", nearest.item.to_string().truecolor(sorted_centroids[nearest.item as usize].color.red as u8, sorted_centroids[nearest.item as usize].color.green as u8, sorted_centroids[nearest.item as usize].color.blue as u8), nearestbts.to_string().truecolor(sorted_centroids[nearestbts as usize].color.red as u8, sorted_centroids[nearestbts as usize].color.green as u8, sorted_centroids[nearestbts as usize].color.blue as u8));

        // println!("{}", nearest_t)

        let rgbvalue = sorted_centroids[nearestbts as usize];
        // posterized_pixels.push(rgbvalue.color.red as u8);
        // posterized_pixels.push(rgbvalue.color.green as u8);
        // posterized_pixels.push(rgbvalue.color.blue as u8);

        posterized_pixels.push(rgbvalue.color);

        // returns array of width 960 and height 180

        // to get 1d position from x (w) and y (h) use x + y * 320 * 3
    }

    return (posterized_pixels, sorted_centroids)
}