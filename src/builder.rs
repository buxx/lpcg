use std::path::Path;

use crate::input::Input;
use image::{imageops, DynamicImage};

pub struct Builder {
    spritesheets_source: String,
}

impl Builder {
    pub fn new(spritesheets_source: String) -> Self {
        Self {
            spritesheets_source,
        }
    }

    pub fn build(&self, input: Input) -> DynamicImage {
        let mut final_image =
            image::open(&format!("{}/background.png", self.spritesheets_source)).unwrap();

        for layer in input.layers() {
            println!("Build from {}", layer);
            let img2 = image::open(&Path::new(&format!(
                "{}/{}",
                self.spritesheets_source, layer,
            )))
            .ok()
            .expect("Opening image failed");
            imageops::overlay(&mut final_image, &img2, 0, 0);
        }

        final_image
    }
}
