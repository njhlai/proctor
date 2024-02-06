use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use colored::Colorize;

use crate::modules::config::Config;

use super::lsp::{Pyright, RustAnalyzer};

/// A structure defining language-specific dev environment setup.
pub struct Setup {
    pub lang: String,
    sol_dir: PathBuf,
    configs: Vec<(PathBuf, String)>,
}

impl Setup {
    /// Returns a [`Setup`] for the language `lang` inside `sol_dir`.
    pub fn from(lang: String, sol_dir: PathBuf, file_to_content: Vec<(PathBuf, String)>) -> Self {
        Setup { lang, sol_dir, configs: file_to_content }
    }

    /// Write configurations defined by [`Setup`]'s `file_to_content` to disk.
    pub fn write(&self, overwrite: bool) -> io::Result<()> {
        for (file, content) in &self.configs {
            let filepath = self.sol_dir.join(file);

            if filepath.exists() && !overwrite {
                println!(
                    "{} for {} dev environment at solution root {} exists, skipping",
                    file.display().to_string().yellow().bold(),
                    self.lang.cyan().bold(),
                    self.sol_dir.display().to_string().yellow().bold(),
                );
            } else {
                println!(
                    "Generating {} for {} dev environment at solution root {}",
                    file.display().to_string().yellow().bold(),
                    self.lang.cyan().bold(),
                    self.sol_dir.display().to_string().yellow().bold(),
                );
                fs::write(filepath, content)?;
            }
        }

        Ok(())
    }
}

type Setups = Vec<(Setup, Option<Command>)>;

pub fn generate_setups(config: &Config) -> Result<Setups, Box<dyn Error>> {
    let pyright = Pyright::from(String::from("./venv"), String::from("py311"), false);
    let mut venv_command = Command::new("python");
    venv_command
        .args(["-m", "venv", "venv/py311"])
        .current_dir(&config.sol_dir_str);

    let mut rust_analyzer = RustAnalyzer::new(Path::new(&config.project_dir_str));
    rust_analyzer
        .parse_directory_as_crates(Path::new(&config.sol_dir_str))
        .unwrap_or_else(|_| panic!("Couldn't parse directory structure of solution root {}", config.sol_dir_str));

    Ok(vec![
        (
            Setup::from(
                String::from("C++"),
                PathBuf::from(&config.sol_dir_str),
                vec![(
                    PathBuf::from(".clangd"),
                    format!("CompileFlags:\n  Add: -I{}/lib/cpp/src/\n", config.project_dir_str),
                )],
            ),
            None,
        ),
        (
            Setup::from(
                String::from("Python"),
                PathBuf::from(&config.sol_dir_str),
                vec![
                    (PathBuf::from(".python-version"), String::from("3.11.7\n")),
                    (PathBuf::from("pyrightconfig.json"), serde_json::to_string_pretty(&pyright)?),
                ],
            ),
            Some(venv_command),
        ),
        (
            Setup::from(
                String::from("Rust"),
                PathBuf::from(&config.sol_dir_str),
                vec![(PathBuf::from("rust-project.json"), serde_json::to_string_pretty(&rust_analyzer)?)],
            ),
            None,
        ),
    ])
}
