use std::fmt::Display;

#[derive(Default)]
pub struct Input {
    layers: Vec<String>,
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
            let mut layer = vec![];
            for part in layer_str.split("::") {
                if part.len() > 0 {
                    layer.push(part.to_string());
                }
            }

            if layer.len() == 0 {
                return Err(Error::NoStrReadableInput);
            }

            layers.push(format!("{}.png", layer.join("/")));
        }

        if layers.len() == 0 {
            return Err(Error::NoStrReadableInput);
        }

        Ok(Self { layers })
    }

    pub fn layer(mut self, selector: Vec<&str>) -> Self {
        self.layers.push(format!("{}.png", selector.join("/")));
        self
    }

    pub fn layers(&self) -> &Vec<String> {
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
