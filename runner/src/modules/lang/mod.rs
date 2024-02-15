mod lsp;

use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString};

use super::config::Config;
use super::dev_env::Setups;

use self::lsp::Lsp;

const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-stdlib=libc++", "-Wall", "-fsanitize=address", "-g3", "-O2"];
const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];

/// An enum listing available code languages.
#[derive(Clone, Debug, Display, EnumCount, EnumIter, EnumProperty, EnumString, PartialEq)]
pub enum Lang {
    #[strum(serialize = "cpp", props(name = "C++"))]
    Cpp,
    #[strum(serialize = "py", props(name = "Python3"))]
    Python,
    #[strum(serialize = "rs", props(name = "Rust"))]
    Rust,
}

impl Lang {
    /// Get the full name of the language.
    pub fn get_name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    /// Returns the [`Command`] that executes the solution-testing `binfile`.
    pub fn tester(&self, binfile: &PathBuf, config: &Config) -> Command {
        match self {
            Lang::Cpp => {
                let mut runner = Command::new(binfile);
                runner
                    .arg("--success")
                    .args(CLANG_COLOR_ARGS)
                    .env("LD_LIBRARY_PATH", format!("{}/lib/cpp/build", config.project_dir_str));

                runner
            }
            Lang::Python => {
                let mut runner = Command::new("python");
                runner
                    .arg(binfile)
                    .arg("-v")
                    .env("PATH", format!("{}/venv/py311/bin:$PATH", config.sol_dir_str));

                runner
            }
            Lang::Rust => {
                let mut runner = Command::new(binfile);
                runner.arg("--show-output").args(RUSTC_COLOR_ARGS);

                runner
            }
        }
    }

    /// Returns the [`Command`] that executes the language compiler.
    pub fn compiler(&self, config: &Config) -> Command {
        match self {
            Lang::Cpp => {
                let mut compiler = Command::new("clang++");
                compiler
                    .args([
                        format!("-I{}/lib/cpp/src", config.project_dir_str).as_str(),
                        format!("-L{}/lib/cpp/build", config.project_dir_str).as_str(),
                        "-lproctor",
                    ])
                    .args(CLANG_COMPILE_FLAGS);

                compiler
            }
            Lang::Python => {
                let mut compiler = Command::new("python");
                compiler.arg(format!("{}/runner/wrappers/compile.py", config.project_dir_str));

                compiler
            }
            Lang::Rust => {
                let mut compiler = Command::new("rustc");
                compiler
                    .args([
                        "--extern",
                        format!("libproctor={}/target/release/libproctor.rlib", config.project_dir_str).as_str(),
                    ])
                    .args(RUSTC_COMPILE_FLAGS);

                compiler
            }
        }
    }
}

impl LangIter {
    /// Generates [`Setups`] detailing the setups for all available languages.
    pub fn generate_setups(self, config: &Config) -> Result<Setups, Box<dyn Error>> {
        self.map(|lang| match lang {
            Lang::Cpp => lsp::Clangd::from(config).generate_setup(config),
            Lang::Python => lsp::Pyright::from(config).generate_setup(config),
            Lang::Rust => lsp::RustAnalyzer::from(config).generate_setup(config),
        })
        .collect()
    }
}
