use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use serde::Serialize;

use crate::modules::config::Config;
use crate::modules::dev_env::Setup;
use crate::modules::lang::Lang;

use super::Lsp;

/// `pyright` config JSON serializer.
#[derive(Serialize)]
pub struct Pyright {
    #[serde(rename = "venvPath")]
    venv_path: String,
    venv: String,
    #[serde(rename = "reportUnusedImport")]
    report_unused_import: bool,
}

impl Pyright {
    /// Returns a [`Pyright`] detailing a `pyright` config for the dev environment.
    pub fn from(venv_path: String, venv: String, report_unused_import: bool) -> Self {
        Pyright { venv_path, venv, report_unused_import }
    }
}

impl Lsp for Pyright {
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        let additional_command = if PathBuf::from(&config.sol_dir_str)
            .join("venv/py311")
            .exists()
        {
            None
        } else {
            let mut venv_command = Command::new("python");
            venv_command
                .args(["-m", "venv", "venv/py311"])
                .current_dir(&config.sol_dir_str);

            Some(venv_command)
        };

        Ok((
            Setup::from(
                Lang::Python,
                PathBuf::from(&config.sol_dir_str),
                vec![
                    (PathBuf::from(".python-version"), String::from("3.11.7\n")),
                    (PathBuf::from("pyrightconfig.json"), serde_json::to_string_pretty(self)?),
                ],
            ),
            additional_command,
        ))
    }
}
