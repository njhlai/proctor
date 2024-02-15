use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::modules::config::Config;
use crate::modules::lang::Lang;

use super::output_streams::OutputStream;
use super::solution::Solution;

/// A solution builder, defining the language-specific solution compiler and solution-testing bin runner.
pub struct Builder {
    lang: Lang,
    compiler: Command,
    binfile: PathBuf,
}

impl Builder {
    /// Constructs a [`Builder`] for the language `lang`.
    pub fn new(lang: &Lang, config: &Config) -> Self {
        let compiler = lang.compiler(config);

        Builder { lang: lang.clone(), compiler, binfile: config.binfile(&lang.to_string()) }
    }

    /// Compiles `solution` via [`Builder`]'s `compiler` command.
    pub fn compile(&mut self, solution: &Solution) -> Result<OutputStream, OutputStream> {
        let solfile = solution.solfile(&self.lang.to_string());

        let output = self
            .compiler
            .arg(solfile.display().to_string())
            .args(["-o", &self.binfile.display().to_string()])
            .output()
            .unwrap_or_else(|_| panic!("Failed to compile solution to problem {}", solution.id()));

        if output.status.success() {
            Ok(OutputStream::from(&output))
        } else {
            let _ = fs::remove_file(&self.binfile);

            Err(OutputStream::from(&output))
        }
    }
}
