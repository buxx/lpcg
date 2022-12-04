use std::{fmt::Display, path::Path, process::Output};

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

    pub fn build(&self, input: Input) -> BuildResult {
        let mut result = BuildResult::new();
        let mut final_image =
            match image::open(&format!("{}/background.png", self.spritesheets_source)) {
                Ok(image) => image,
                Err(error) => {
                    return BuildResult::error(BuildError::UnexpectedError(format!(
                        "Unable to open background.png file : {}",
                        error
                    )))
                }
            };

        for layer in input.layers() {
            let layer_image = match image::open(&Path::new(&format!(
                "{}/{}",
                self.spritesheets_source, layer,
            ))) {
                Ok(image) => image,
                Err(error) => {
                    result
                        .errors
                        .push(BuildError::LayerError(layer.clone(), error.to_string()));
                    continue;
                }
            };
            imageops::overlay(&mut final_image, &layer_image, 0, 0);
        }

        result.output = Some(final_image);
        result
    }
}

pub enum BuildError {
    LayerError(String, String),
    UnexpectedError(String),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::LayerError(layer, error) => {
                f.write_str(&format!("Layer error '{}' : '{}'", layer, error))
            }
            BuildError::UnexpectedError(error) => {
                f.write_str(&format!("Unexpected error : '{}'", error))
            }
        }
    }
}

pub struct BuildResult {
    pub errors: Vec<BuildError>,
    pub output: Option<DynamicImage>,
}

impl BuildResult {
    pub fn error(error: BuildError) -> Self {
        Self {
            errors: vec![error],
            output: None,
        }
    }
    pub fn new() -> Self {
        Self {
            errors: vec![],
            output: None,
        }
    }
}
