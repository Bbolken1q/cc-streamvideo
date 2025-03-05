pub mod frame {

    pub struct Frame {
        video: String,
    }

    impl Frame {
        pub fn get_video(&self) -> String {
            self.video.clone()
        }

        pub fn new(value_video: &String) -> Frame {
            Frame {
                video: value_video.clone().to_string(),
            }
        }
    }
}