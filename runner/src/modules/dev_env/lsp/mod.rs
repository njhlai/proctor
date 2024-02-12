mod clangd;
mod pyright;
mod rust_analyzer;

use std::error::Error;
use std::process::Command;

use crate::modules::config::Config;

use super::setup::Setup;

pub use self::clangd::Clangd;
pub use self::pyright::Pyright;
pub use self::rust_analyzer::RustAnalyzer;

/// A trait that allows LSP config generation.
pub trait Lsp {
    /// Generates the associated [`Setup`] and additional commands to run.
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>>;
}
