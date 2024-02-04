use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

/// `runner` config JSON serializer.
#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "project_dir")]
    project_dir_str: String,
    #[serde(rename = "sol_dir")]
    sol_dir_str: String,
}

impl Config {
    /// Reads from `config_path` and returns a [`Config`].
    pub fn read(config_path: &Option<String>) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(if let Some(config) = config_path.as_ref() {
            config
        } else {
            "config.json"
        })?))?)
    }

    /// Returns a [`Config`] with the specified configurations.
    pub fn new(project_dir_str: String, sol_dir_str: String) -> Self {
        Config { project_dir_str, sol_dir_str }
    }

    /// Get the project and solution directories paths.
    pub fn get_dirs(&self) -> (&str, &str) {
        (self.project_dir_str.as_str(), self.sol_dir_str.as_str())
    }
}
