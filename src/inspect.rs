use std::path::PathBuf;

use walkdir::WalkDir;

pub struct Inspector {
    spritesheets: PathBuf,
}

impl Inspector {
    pub fn new(spritesheets: PathBuf) -> Self {
        Self { spritesheets }
    }

    pub fn identifiers(&self) -> Vec<String> {
        let mut identifiers = vec![];

        for entry in WalkDir::new(&self.spritesheets)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with(".png") {
                let mut layer_names = vec![];
                for part in entry.path().iter() {
                    layer_names.push(part.to_string_lossy())
                }
                let identifier = layer_names.join("::");
                let identifier = identifier.strip_prefix(".::").unwrap_or(&identifier);
                identifiers.push(
                    identifier
                        .strip_prefix(&format!(
                            "{}::",
                            self.spritesheets
                                .to_string_lossy()
                                .strip_prefix("./")
                                .unwrap_or(&self.spritesheets.to_string_lossy())
                        ))
                        .expect("must start with given folder")
                        .strip_suffix(".png")
                        .expect("must finish by .png")
                        .to_string(),
                );
            }
        }

        identifiers
    }
}
