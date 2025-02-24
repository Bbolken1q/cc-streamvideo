#[path ="./posterization/sorting.rs"]
mod sorting;
#[allow(unused_imports)]
use sorting::lightness;

pub mod pixel_group {
    use crate::qpixel::qpixel::Qpixel;
    use palette::rgb::Rgb;

    use super::sorting::hilbert_lookup;

    #[derive(Debug)]
    pub struct PixelGroup {
        pub pixels: Vec<Qpixel>,
        pub structure: u8,
        pub c1: usize,
        pub c2: usize,
        pub character: String
    }

    impl PixelGroup {
        pub fn new(slice: [Rgb; 6]) -> PixelGroup {
            let mut pixels: Vec<Qpixel> = Vec::new();

            for i in 0..slice.len() {
                pixels.push(Qpixel::new(slice[i], &hilbert_lookup, Some(i as u32)));
            }
            return PixelGroup { pixels: pixels, structure: 0b00000000, c1: 0, c2: 0, character: "0x80".to_string()};
        }
    }
}