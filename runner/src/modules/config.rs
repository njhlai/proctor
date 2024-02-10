use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use colored::Colorize;
use serde::Deserialize;

/// `runner` config JSON serializer.
#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "project_dir")]
    pub project_dir_str: String,
    #[serde(rename = "sol_dir")]
    pub sol_dir_str: String,
}

impl Config {
    /// Reads from `config_path` and returns a [`Config`].
    pub fn read(config_path: &Option<String>) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(if let Some(config) = config_path.as_ref() {
            PathBuf::from(config)
        } else if let Some(config_local_dir) = dirs::config_local_dir() {
            let default_config_file = config_local_dir.join("proctor/config.json");
            if default_config_file.exists() {
                println!("Reading config from {}", default_config_file.display().to_string().yellow().bold());

                default_config_file
            } else {
                println!("Reading config from {}", "./config.json".yellow().bold());

                PathBuf::from("config.json")
            }
        } else {
            println!("Reading config from {}", "./config.json".yellow().bold());

            PathBuf::from("config.json")
        })?))?)
    }

    /// Returns a [`Config`] with the specified configurations.
    pub fn new(project_dir_str: String, sol_dir_str: String) -> Self {
        Config { project_dir_str, sol_dir_str }
    }

    /// Returns the `PathBuf` to the testing bin file for language (with extension `ext`).
    pub fn binfile(&self, ext: &str) -> PathBuf {
        PathBuf::from(&self.project_dir_str).join(format!("bin/test_{ext}"))
    }
}
