use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use serde::Serialize;

use crate::modules::config::Config;
use crate::modules::lang::Lang;

use super::super::setup::Setup;
use super::Lsp;

/// `rust-analyzer` config JSON serializer.
#[derive(Serialize)]
pub struct RustAnalyzer {
    sysroot_src: String,
    crates: Vec<Crate>,
}

/// A `crate` in `rust-analyzer`.
#[derive(Serialize)]
struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<Dep>,
    cfg: Vec<String>,
}

/// `dep` component of a `crate`.
#[derive(Serialize)]
struct Dep {
    #[serde(rename = "crate")]
    pos: usize,
    name: String,
}

impl RustAnalyzer {
    /// Returns a [`RustAnalyzer`] detailing a `rust-analyzer` config for the dev environment.
    pub fn new(project_dir: &Path) -> RustAnalyzer {
        let toolchain = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()
            .expect("hello")
            .stdout;

        let toolchain = String::from_utf8_lossy(&toolchain);
        let mut whitespace_iter = toolchain.split_whitespace();
        let toolchain = whitespace_iter.next().unwrap_or(&toolchain);

        RustAnalyzer {
            sysroot_src: Path::new(toolchain)
                .join("lib")
                .join("rustlib")
                .join("src")
                .join("rust")
                .join("library")
                .display()
                .to_string(),
            crates: vec![Crate {
                root_module: project_dir
                    .join("lib")
                    .join("rs")
                    .join("src")
                    .join("lib.rs")
                    .display()
                    .to_string(),
                edition: String::from("2021"),
                deps: vec![],
                cfg: vec![],
            }],
        }
    }

    /// Returns a [`RustAnalyzer`] using values from `config`.
    pub fn from(config: &Config) -> RustAnalyzer {
        let mut rust_analyzer = RustAnalyzer::new(Path::new(&config.project_dir_str));
        rust_analyzer
            .parse_directory_as_crates(Path::new(&config.sol_dir_str))
            .unwrap_or_else(|_| panic!("Couldn't parse directory structure of solution root {}", config.sol_dir_str));

        rust_analyzer
    }

    /// Converts `path` into a [`Crate`] and pushes it into [`RustAnalyzer`] if it qualifies.
    fn cratify(&mut self, path: &Path) {
        let filepath = path.join("sol.rs");

        if path.is_dir() && filepath.exists() {
            self.crates.push(Crate {
                root_module: filepath.display().to_string(),
                edition: String::from("2021"),
                deps: vec![Dep { pos: 0, name: String::from("libproctor") }],
                cfg: vec![String::from("test")],
            });
        }
    }

    /// Parses and adds qualifying directory under `sol_dir` as a [`Crate`] into [`RustAnalyzer`].
    fn parse_directory_as_crates(&mut self, sol_dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(sol_dir)? {
            self.cratify(entry?.path().as_path());
        }

        self.crates[1..].sort_unstable_by(|a, b| a.root_module.cmp(&b.root_module));

        Ok(())
    }
}

impl Lsp for RustAnalyzer {
    fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        Ok((
            Setup::from(
                Lang::Rust,
                PathBuf::from(&config.sol_dir_str),
                vec![(PathBuf::from("rust-project.json"), serde_json::to_string_pretty(&self)?)],
            ),
            None,
        ))
    }
}
