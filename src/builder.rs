use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::{input::Input, layer::Layer};
use image::{imageops, DynamicImage};

pub struct Builder {
    spritesheets: PathBuf,
}

impl Builder {
    pub fn new(spritesheets: PathBuf) -> Self {
        Self { spritesheets }
    }

    pub fn build(&self, input: Input) -> BuildResult {
        let mut result = BuildResult::new();
        let mut background_path = self.spritesheets.clone();
        background_path.push("background.png");
        let mut final_image = match image::open(&background_path) {
            Ok(image) => image,
            Err(error) => {
                return BuildResult::error(BuildError::UnexpectedError(format!(
                    "Unable to open '{:?}' file : {}",
                    background_path, error
                )))
            }
        };

        for layer in input.layers() {
            let layer_path = match layer.path(Path::new(&self.spritesheets)) {
                Ok(layer_path) => layer_path,
                Err(error) => {
                    result
                        .errors
                        .push(BuildError::LayerError(layer.clone(), error.to_string()));
                    continue;
                }
            };
            let layer_image = match image::open(&layer_path) {
                Ok(image) => image,
                Err(error) => {
                    result.errors.push(BuildError::LayerError(
                        layer.clone(),
                        format!("{:?} : '{}'", layer_path, error),
                    ));
                    continue;
                }
            };
            imageops::overlay(&mut final_image, &layer_image, 0, 0);
        }

        result.output = Some(final_image);
        result
    }
}

#[derive(Clone)]
pub enum BuildError {
    LayerError(Layer, String),
    UnexpectedError(String),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::LayerError(layer, error) => {
                f.write_str(&format!("Layer error '{}' : '{}'", layer.name(), error))
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
