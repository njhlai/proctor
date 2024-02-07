use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use crate::modules::config::Config;

use super::super::setup::Setup;
use super::Lsp;

/// A structure for `clangd` config for [`Lsp`] trait application.
pub struct Clangd {
    content: String,
}

impl Clangd {
    pub fn from(config: &Config) -> Self {
        Clangd { content: format!("CompileFlags:\n  Add: -I{}/lib/cpp/src/\n", config.project_dir_str) }
    }
}

impl Lsp for Clangd {
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        Ok((
            Setup::from(
                String::from("C++"),
                PathBuf::from(&config.sol_dir_str),
                vec![(PathBuf::from(".clangd"), self.content.clone())],
            ),
            None,
        ))
    }
}
