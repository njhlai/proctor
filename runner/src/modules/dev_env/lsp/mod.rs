mod clangd;
mod pyright;
mod rust_analyzer;

use std::error::Error;
use std::process::Command;

use super::super::config::Config;
use super::setup::{Setup, Setups};

use self::clangd::Clangd;
use self::pyright::Pyright;
use self::rust_analyzer::RustAnalyzer;

/// A trait that allows LSP config generation.
pub trait Lsp {
    /// Generates the associated [`Setup`] and additional commands to run.
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>>;
}

/// Generates [`Setups`] detailing the setups for all available languages.
pub fn generate_setups(config: &Config) -> Result<Setups, Box<dyn Error>> {
    Ok(vec![
        Clangd::from(config).generate_setup(config)?,
        Pyright::from(String::from("./venv"), String::from("py311"), false).generate_setup(config)?,
        RustAnalyzer::from(config).generate_setup(config)?,
    ])
}
