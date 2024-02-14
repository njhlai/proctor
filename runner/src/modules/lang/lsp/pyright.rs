use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::modules::config::Config;
use crate::modules::dev_env::Setup;
use crate::modules::lang::Lang;

use super::Lsp;

/// `pyright` config JSON serializer.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pyright {
    venv_path: String,
    venv: String,
    report_unused_import: bool,
}

impl Pyright {
    /// Returns a [`Pyright`] detailing a `pyright` config for the dev environment.
    pub fn from(config: &Config) -> Self {
        if let Some(pyconf) = config.lang.get(&Lang::Python.to_string()) {
            if let Some(pyrightconf) = pyconf.get("pyright") {
                serde_json::from_value(pyrightconf.clone()).unwrap()
            } else {
                Pyright { venv_path: String::from("./venv"), venv: String::from("py311"), report_unused_import: false }
            }
        } else {
            panic!(
                "{}: Can't find entry for {} in lang of config!",
                "ERR".red().bold(),
                Lang::Python.get_name().cyan().bold()
            )
        }
    }
}

impl Lsp for Pyright {
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        let venv_dir = format!("{}/{}", self.venv_path, self.venv);
        let additional_command = if PathBuf::from(&config.sol_dir_str).join(&venv_dir).exists() {
            None
        } else {
            let mut venv_command = Command::new("python");
            venv_command
                .args(["-m", "venv", &venv_dir])
                .current_dir(&config.sol_dir_str);

            Some(venv_command)
        };

        Ok((
            Setup::from(
                Lang::Python,
                PathBuf::from(&config.sol_dir_str),
                vec![
                    (
                        PathBuf::from(".python-version"),
                        config
                            .lang
                            .get(&Lang::Python.to_string())
                            .unwrap()
                            .get("version")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    ),
                    (PathBuf::from("pyrightconfig.json"), serde_json::to_string_pretty(self)?),
                ],
            ),
            additional_command,
        ))
    }
}
