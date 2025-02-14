use palette::rgb::Rgb;

#[path="../sort.rs"]
mod sort;
use sort::sort3;

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
pub fn lightness(pixel: &Rgb) -> f32 { 
    return srgb_to_linear(pixel.red) + srgb_to_linear(pixel.green) * 3.0 + srgb_to_linear(pixel.blue)
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