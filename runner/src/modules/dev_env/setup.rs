use std::path::PathBuf;
use std::{fs, io};

use colored::Colorize;

/// A structure defining language-specific dev environment setups.
pub struct Setup {
    lang: String,
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
