pub mod qpixel {
    use palette::rgb::Rgb;

    #[derive(Clone, Copy, Debug)]
    pub struct Qpixel { 
        pub color: Rgb,
        pub hue: f32,
        pub index: u32
    }

    impl PartialEq for Qpixel {
        fn eq(&self, other: &Self) -> bool {
            self.hue == other.hue
        }

        fn ne(&self, other: &Self) -> bool {
            self.hue != other.hue
        }
    }

    impl Qpixel {
        pub fn new(_pixel: Rgb, _fun: &dyn Fn(&Rgb) -> f32, _index: Option<u32> ) -> Qpixel {
            return Qpixel { color: _pixel, hue: _fun(&_pixel), index: _index.unwrap_or_default() }
        }
    }
}
