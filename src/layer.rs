extern crate rand;
use rand::seq::SliceRandom;
use std::{fmt::Display, fs};
use walkdir::WalkDir;

use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Layer {
    name: String,
    parts: Vec<Part>,
}

impl Layer {
    pub fn new(name: String, parts: Vec<Part>) -> Self {
        Self { name, parts }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self, relative_to: &Path) -> Result<PathBuf, Error> {
        let mut final_path = PathBuf::new();
        final_path.push(relative_to);

        for part in &self.parts {
            match part {
                Part::Folder(folder_name) => final_path.push(folder_name),
                Part::Image(file_name) => final_path.push(format!("{}.png", file_name)),
                Part::WildCard(variant) => final_path = self.random_from(&final_path, variant)?,
            }
        }

        Ok(final_path)
    }

    fn random_from(
        &self,
        relative_path: &PathBuf,
        variant: &Option<String>,
    ) -> Result<PathBuf, Error> {
        let mut entries = vec![];
        for entry in WalkDir::new(&relative_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|item| item.file_type().is_dir())
        {
            if let Some(variant) = &variant {
                // If folder contain variant, take it
                if let Some(variant_file) = fs::read_dir(entry.path())
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|f| f.path().ends_with(&format!("{}.png", variant)))
                    .next()
                {
                    entries.push(variant_file)
                } else {
                    // else collect all image files in this folder
                    for image_file in fs::read_dir(entry.path())
                        .unwrap()
                        .filter_map(Result::ok)
                        .filter(|d| d.path().to_string_lossy().ends_with("png"))
                    {
                        entries.push(image_file)
                    }
                }
            } else {
                // Collect all image files in this folder
                for image_file in fs::read_dir(entry.path())
                    .unwrap()
                    .filter_map(Result::ok)
                    .filter(|d| d.path().to_string_lossy().ends_with("png"))
                {
                    entries.push(image_file)
                }
            }
        }

        let file = match entries.choose(&mut rand::thread_rng()) {
            Some(file) => file.clone(),
            None => {
                return Err(Error::FailToRandomChooseFile(
                    "Directory is empty".to_string(),
                ))
            }
        };

        Ok(file.path().to_path_buf())
    }
}

#[derive(Clone)]
pub enum Part {
    WildCard(Option<String>),
    Folder(String),
    Image(String),
}

pub enum Error {
    FailToRandomChooseFile(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailToRandomChooseFile(message) => {
                f.write_str(&format!("Cant randomly choose from files : {}", message))
            }
        }
    }
}
