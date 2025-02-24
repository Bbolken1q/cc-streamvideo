use kiddo::{float::kdtree::Axis, traits::DistanceMetric};
use palette::rgb::Rgb;
use std::primitive::u8;

use crate::qpixel::qpixel::Qpixel;

#[path = "sorting.rs"]
mod sorting;
use sorting::hue;

#[allow(dead_code)]
fn find_distance(point_a: &[u8; 3], point_b: &[u8; 3]) -> u16 {
    let vert_1: i32 = (((point_a[0] as i32).pow(2)) - (point_b[0] as i32).pow(2) as i32).abs();
    let vert_2: i32 = (((point_a[1] as i32).pow(2) - (point_b[1] as i32).pow(2)) as i32).abs();
    let vert_3: i32 = (((point_a[2] as i32).pow(2) - (point_b[2] as i32).pow(2)) as i32).abs();

    let dist = (vert_1 + vert_2 + vert_3) as u16;
    return dist;
}

#[allow(dead_code)]
pub fn nearest_distance_squared_euclidean(point: Qpixel, centroids: &Vec<Qpixel>) -> usize {
    let mut smallest_distance: u16 = u16::MAX;
    let mut smallest_iter: usize = 0;
    for i in 0..15 {
        // let cdist = (centroids[i].hue - point.hue).abs();
        let cdist = find_distance(
            &[
                point.color.red as u8,
                point.color.green as u8,
                point.color.blue as u8,
            ],
            &[
                centroids[i].color.red as u8,
                centroids[i].color.green as u8,
                centroids[i].color.blue as u8,
            ],
        );
        // println!("|{} - {}| = {}",centroids[i].hue, point.hue, (centroids[i].hue - point.hue).abs());
        if cdist < smallest_distance {
            smallest_distance = cdist;
            smallest_iter = i;
            // println!("Found new smallest! New distance: {}", cdist)
        }
    }
    // println!("{}", smallest_iter);
    return smallest_iter;
}

#[allow(dead_code)]
pub fn quantize_colors(centroids: &Vec<Rgb>, fun: &dyn Fn(&Rgb) -> f32) -> Vec<Qpixel> {
    let mut qpixels: Vec<Qpixel> = Vec::with_capacity(centroids.len());

    for color in centroids {
        // qpixels.push(QPixel::new(*color));
        qpixels.push(Qpixel::new(*color, fun, None))
    }

    return qpixels;
}

#[allow(dead_code)]
pub struct HueSorting {}

impl<A: Axis, const K: usize> DistanceMetric<A, K> for HueSorting
where
    A: Into<f32>,
{
    #[inline]
    fn dist(a: &[A; K], b: &[A; K]) -> A {
        let hue_a = hue(&Rgb::new(a[0].into(), a[1].into(), a[2].into()));
        let hue_b = hue(&Rgb::new(b[0].into(), b[1].into(), b[2].into()));
        return A::from((hue_a - hue_b).abs()).unwrap();
        // a.iter().zip(b.iter())
        // .map(|(&a_val, &b_val)| (a_val - b_val) * (a_val - b_val))
        // .fold(A::zero(), std::ops::Add::add)
    }

    #[inline]
    fn dist1(a: A, b: A) -> A {
        (a - b) * (a - b)
        // (a-b).abs()
    }
}

fn get_closest(vec: &Vec<f32>, index: &usize, hue: &f32) -> usize {
    if *index == 15 {
        return 14;
    }
    if *index == 0 {
        return 0;
    }
    if (vec[*index] - hue).abs() <= (vec[index - 1] - hue).abs() {
        return *index;
    } else {
        return *index - 1;
    }
}

pub fn get_position(v: &Vec<f32>, point: f32) -> usize {
    match v.binary_search_by(|px| px.partial_cmp(&point).expect("Couldn't compare values")) {
        Ok(pos) => {
            return pos;
        }
        Err(pos) => {
            return get_closest(&v, &pos, &point);
        } // Ok(pos) => { return get_closest(&v, &pos, &point); } // element already in vector @ `pos`
          // Err(pos) => { return get_closest(&v, &pos, &point); }
    }
}
