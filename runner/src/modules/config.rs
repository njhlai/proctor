use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use colored::Colorize;
use serde::Deserialize;

use super::extcolorize::ExtColorize;

/// `runner` config JSON serializer.
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "project_dir")]
    pub project_dir_str: String,
    #[serde(rename = "sol_dir")]
    pub sol_dir_str: String,
}

impl Config {
    /// Reads from `config_path` and returns a [`Config`].
    pub fn read(config_path: &Option<String>) -> Result<(Self, Option<PathBuf>), Box<dyn Error>> {
        let mut maybe_config_files = vec![];

        if let Some(config) = config_path.as_ref() {
            maybe_config_files.push(PathBuf::from(config));
        }
        maybe_config_files.push(PathBuf::from("./config.json"));
        if let Some(config_local_dir) = dirs::config_local_dir() {
            let default_config_file = config_local_dir.join("proctor/config.json");
            maybe_config_files.push(default_config_file);
        }

        for (i, pathbuf) in maybe_config_files.iter().enumerate() {
            if i > 0 { println!("{}!", "FAILED".red().bold()); }

            print!("Reading config from {}... ", pathbuf.display().to_string().orange().bold());
            io::stdout().flush()?;

            if let Ok(file) = File::open(pathbuf) {
                if let Ok(config) = serde_json::from_reader(BufReader::new(file)) {
                    println!("{}!", "OK".green().bold());

                    return Ok((config, Some(pathbuf.clone())));
                }
            }
        }

        println!("{}!", "FAILED".red().bold());
        println!("{}: Can't read configuration, proceeding with default configuration", "WARNING".yellow().bold());

        Ok((Config::new(String::from("."), String::from("./data")), None))
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
