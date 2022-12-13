use std::fmt::Display;

use crate::layer::{Layer, Part};

#[derive(Default)]
pub struct Input {
    layers: Vec<Layer>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_str(input: &str) -> Result<Self, Error> {
        let mut layers = vec![];

        for layer_str in input.split(" ") {
            let mut layer_parts = vec![];
            let parts_str: Vec<String> = layer_str.split("::").map(str::to_string).collect();
            for (i, part_str) in parts_str.iter().enumerate() {
                if part_str.len() > 0 {
                    let part = if part_str[..].contains("*") {
                        if part_str[..].ends_with("*") {
                            Part::WildCard(None)
                        } else {
                            let variant = &part_str[2..part_str.len() - 1];
                            Part::WildCard(Some(variant.to_string()))
                        }
                    } else {
                        if i == parts_str.len() - 1 {
                            Part::Image(part_str.to_string())
                        } else {
                            Part::Folder(part_str.to_string())
                        }
                    };
                    layer_parts.push(part);
                }
            }

            if layer_parts.len() == 0 {
                return Err(Error::NoStrReadableInput);
            }

            layers.push(Layer::new(layer_str.to_string(), layer_parts));
        }

        if layers.len() == 0 {
            return Err(Error::NoStrReadableInput);
        }

        Ok(Self { layers })
    }

    pub fn layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }
}

pub enum Error {
    NoStrReadableInput,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoStrReadableInput => f.write_str(&format!("No readable input given")),
        }
    }
}
