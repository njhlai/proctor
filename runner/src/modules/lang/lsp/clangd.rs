use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;
use serde::Deserialize;

use crate::modules::config::Config;
use crate::modules::dev_env::Setup;
use crate::modules::lang::Lang;

use super::Lsp;

/// A structure for `clangd` config for [`Lsp`] trait application.
#[derive(Deserialize)]
pub struct Clangd {
    content: String,
}

impl Clangd {
    pub fn from(config: &Config) -> Self {
        if let Some(cppconf) = config.lang.get(&Lang::Cpp.to_string()) {
            if let Some(clangdconf) = cppconf.get("clangd") {
                serde_json::from_value(clangdconf.clone()).unwrap()
            } else {
                Clangd { content: format!("CompileFlags:\n  Add: -I{}/lib/cpp/src/\n", config.project_dir_str) }
            }
        } else {
            panic!(
                "{}: Can't find entry for {} in lang of config!",
                "ERR".red().bold(),
                Lang::Cpp.get_name().cyan().bold()
            )
        }
    }
}

impl Lsp for Clangd {
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        Ok((
            Setup::from(
                Lang::Cpp,
                PathBuf::from(&config.sol_dir_str),
                vec![(PathBuf::from(".clangd"), self.content.clone())],
            ),
            None,
        ))
    }
}
