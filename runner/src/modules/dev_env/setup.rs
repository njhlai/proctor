use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;
use strum::EnumProperty;

use crate::modules::extcolorize::ExtColorize;
use crate::modules::lang::Lang;

/// A structure defining language-specific dev environment setup.
pub struct Setup {
    pub lang: Lang,
    sol_dir: PathBuf,
    configs: Vec<(PathBuf, String)>,
}

impl Setup {
    /// Returns a [`Setup`] for the language `lang` inside `sol_dir`.
    pub fn from(lang: Lang, sol_dir: PathBuf, file_to_content: Vec<(PathBuf, String)>) -> Self {
        Setup { lang, sol_dir, configs: file_to_content }
    }

    /// Write configurations defined by [`Setup`]'s `file_to_content` to disk.
    pub fn write(&self, overwrite: bool) -> io::Result<()> {
        for (file, content) in &self.configs {
            let filepath = self.sol_dir.join(file);

            if filepath.exists() && !overwrite {
                println!(
                    "{} for {} dev environment at solution root {} exists, skipping",
                    file.display().to_string().orange().bold(),
                    self.lang.get_str("name").unwrap().cyan().bold(),
                    self.sol_dir.display().to_string().orange().bold()
                );
            } else {
                print!(
                    "Generating {} for {} dev environment at solution root {}... ",
                    file.display().to_string().orange().bold(),
                    self.lang.get_str("name").unwrap().cyan().bold(),
                    self.sol_dir.display().to_string().orange().bold()
                );
                io::stdout().flush()?;

                fs::write(filepath, content)?;
                println!("{}!", "OK".green().bold());
            }
        }

        Ok(())
    }
}

/// A list of pairs of [`Setup`] and additional commands to run.
pub type Setups = Vec<(Setup, Option<Command>)>;
