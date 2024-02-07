mod pyright;
mod rust_analyzer;

use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use super::super::config::Config;
use super::setup::{Setup, Setups};

use pyright::Pyright;
use rust_analyzer::RustAnalyzer;

/// A trait that allows LSP config generation.
pub trait Lsp {
    /// Generates the associated [`Setup`] and additional commands to run.
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>>;
}

/// Generates [`Setups`] detailing the setups for all available languages.
pub fn generate_setups(config: &Config) -> Result<Setups, Box<dyn Error>> {
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
        Pyright::from(String::from("./venv"), String::from("py311"), false).generate_setup(config)?,
        RustAnalyzer::from(config).generate_setup(config)?,
    ])
}
