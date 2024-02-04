use std::path::Path;
use std::process::Command;
use std::{fs, io};

use serde::Serialize;

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
    pub fn parse_directory_as_crates(&mut self, sol_dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(sol_dir)? {
            self.cratify(entry?.path().as_path());
        }

        self.crates[1..].sort_unstable_by(|a, b| a.root_module.cmp(&b.root_module));

        Ok(())
    }
}
