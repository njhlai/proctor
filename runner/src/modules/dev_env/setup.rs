use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;

use crate::modules::colorize::MoreColorize;
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
        let n = self.configs.len();

        for (i, (file, content)) in self.configs.iter().enumerate() {
            let filepath = self.sol_dir.join(file);

            print!("  {} ", format!("[{}/{}]", i + 1, n).dimmed());
            if filepath.exists() && !overwrite {
                println!(
                    "{} for {} dev environment at solution root {} exists, skipping",
                    file.display().to_string().orange().bold(),
                    self.lang.get_name().cyan().bold(),
                    self.sol_dir.display().to_string().orange().bold()
                );
            } else {
                print!(
                    "Generating {} for {} dev environment at solution root {}... ",
                    file.display().to_string().orange().bold(),
                    self.lang.get_name().cyan().bold(),
                    self.sol_dir.display().to_string().orange().bold()
                );
                io::stdout().flush()?;

                fs::write(filepath, content)?;
                println!("{}!", "OK".green().bold());
            }
        }

        Ok(())
    }

    /// Runs the setup defined by [`Setup`] as well as any additional commands.
    pub fn run(&self, additional_command: Option<Command>, overwrite: bool) -> Result<(), Box<dyn Error>> {
        Ok(self.write(overwrite).and_then(|()| {
            if let Some(mut cmd) = additional_command {
                println!("  {} Running additional commands for {}...", "*".dimmed(), self.lang.get_name().cyan().bold());

                cmd.output().map(|_| ())
            } else {
                Ok(())
            }
        })?)
    }
}

/// A list of pairs of [`Setup`] and additional commands to run.
pub type Setups = Vec<(Setup, Option<Command>)>;
