use crate::HILBERT_VALUES;
use palette::rgb::Rgb;

#[path="../sort.rs"]
mod sort;
use sort::sort3;

// #[path="./hilbert_values.rs"]
// mod hilbert_values;
// use hilbert_values::hilbert_values;


/*
    convert RGB channel to linear value
*/
#[allow(dead_code)]
fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 { 
        return channel / 12.92;
    }
    else {
        return ((channel + 0.05)/1.055).powf(2.4);
    }
}

#[allow(dead_code)]
pub fn lightness_linear(pixel: &Rgb) -> f32 { 
    return srgb_to_linear(pixel.red)*0.299 + srgb_to_linear(pixel.green) * 0.587 + srgb_to_linear(pixel.blue)*0.114
}

#[allow(dead_code)]
pub fn lightness(pixel: &Rgb) -> f32 { 
    return pixel.red*0.299 + pixel.green * 0.587 + pixel.blue*0.114
}

#[allow(dead_code)]
pub fn lightness_unweighted(pixel: &Rgb) -> f32 { 
    return srgb_to_linear(pixel.red) + srgb_to_linear(pixel.green) + srgb_to_linear(pixel.blue)
}

#[allow(dead_code)]
pub fn hue(pixel: &Rgb) -> f32 { /* Get hue of a pixel from hls */
    // let k = 1.3;
    
    // println!("{:?}", var);
    // return color.lightness + k * color.saturation
    // return pixel.green
    // return pixel.red + pixel.green + pixel.blue / 3.0
    
    let mut x = pixel.red;
    let mut y = pixel.green;
    let mut z = pixel.blue;
    // println!("{:?}", pixel);
    let sorted = sort3(&mut x, &mut y, &mut z);
    // println!("{:?}", sorted);
    let maxc = *sorted[2]; let minc = *sorted[0];
    if maxc == minc { return 0.0; }

    // println!("{}, {}", maxc, minc);

    let mcminusmc: f32 = (maxc - minc).into();
    let rc: f32 = (maxc - pixel.red) / mcminusmc;
    let gc: f32 = (maxc - pixel.green) / mcminusmc;
    let bc: f32 = (maxc - pixel.blue) / mcminusmc;

    let mut h: f32;

    if pixel.red == maxc {
        h = bc - gc;
    }
    else if pixel.green == maxc {
        h = 2.0 + rc - bc; 
    }
    else {
        h = 4.0 + gc - rc;
    }

    h = (h/6.0) as f32 % 1.0;

    return h;
}

#[allow(dead_code)]
fn hilbert_transform(rx: u8, ry: u8, rz: u8) -> u8 {
    (rx << 2) | (ry << 1) | rz
}

#[allow(dead_code)]
fn rotate_and_flip(mut x: u8, mut y: u8, mut z: u8, rx: u8, ry: u8, rz: u8) -> (u8, u8, u8) {
    if rz == 0 {
        if ry == 0 {
            std::mem::swap(&mut x, &mut z);
        }
        if rx == 0 {
            std::mem::swap(&mut y, &mut z);
        }
    }

    return (x, y, z)
}

#[allow(dead_code)]
pub fn hilbert_index_3d(mut x: u8, mut y: u8, mut z: u8, bits: u8) -> u32 { 
    let mut hilbert_index: u32 = 0;
    
    for i in (0..bits).rev() {
        let rx = (x >> i) & 1;
        let ry = (y >> i) & 1;
        let rz = (z >> i) & 1;

        let d = hilbert_transform(rx, ry, rz);
        hilbert_index = (hilbert_index << 3) | d as u32;

        (x, y, z) = rotate_and_flip(x, y, z, rx, ry, rz);
    }

    return hilbert_index;
}

#[allow(dead_code)]
pub fn hilbert_wrapper(pixel: &Rgb) -> f32 {
    let (x, y, z) = (pixel.red, pixel.green, pixel.blue);
    hilbert_index_3d(x as u8, y as u8, z as u8, 8) as f32
}

#[allow(dead_code)]
#[allow(static_mut_refs)]
pub fn fill_lookup() {
    if unsafe { HILBERT_VALUES.len() } > 0 {
        return;
    }

    for x in 0..256 {
        unsafe { HILBERT_VALUES.push(Vec::new()) };
        for y in 0..256 {
            unsafe { HILBERT_VALUES[x].push(Vec::new()) };
            for z in 0..256 {
                unsafe {HILBERT_VALUES[x][y].push(hilbert_index_3d(x as u8, y as u8, z as u8, 8))}
            }
        }
    }
}

#[allow(dead_code)]
pub fn hilbert_lookup(pixel: &Rgb) -> f32 {
    let (x, y, z) = (pixel.red, pixel.green, pixel.blue);
    return unsafe { HILBERT_VALUES[x as usize][y as usize][z as usize] as f32 };
}