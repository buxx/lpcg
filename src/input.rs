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

    pub fn layer(mut self, selector: Vec<&str>) -> Self {
        self.layers.push(format!("{}.png", selector.join("/")));
        self
    }

    pub fn layers(&self) -> &Vec<String> {
        &self.layers
    }
}
