pub mod qpixel {
    use palette::rgb::Rgb;

    #[derive(Clone, Copy, Debug)]
    pub struct Qpixel { 
        pub color: Rgb,
        pub hue: f32
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
        pub fn new(pixel: Rgb, fun: &dyn Fn(&Rgb) -> f32 ) -> Qpixel {
            return Qpixel { color: pixel, hue: fun(&pixel)}
        }
    }
}
