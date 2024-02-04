use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use serde::{Deserialize, Serialize};

pub struct Setup {
    lang: String,
    sol_dir: PathBuf,
    file_to_content: Vec<(PathBuf, String)>,
}

impl Setup {
    pub fn from(lang: String, sol_dir: PathBuf, file_to_content: Vec<(PathBuf, String)>) -> Self {
        Setup { lang, sol_dir, file_to_content }
    }

    pub fn write(&self, overwrite: bool) -> io::Result<()> {
        for (file, content) in &self.file_to_content {
            let filepath = self.sol_dir.join(file);

            if filepath.exists() && !overwrite {
                println!(
                    "`{}` for {} dev environment at solution root {} exists, skipping",
                    file.display(),
                    self.lang,
                    self.sol_dir.display(),
                );
            } else {
                println!(
                    "Generating `{}` for {} dev environment at solution root {}",
                    file.display(),
                    self.lang,
                    self.sol_dir.display(),
                );
                fs::write(filepath, content)?;
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct RustAnalyzer {
    sysroot_src: String,
    crates: Vec<Crate>,
}

#[derive(Serialize, Deserialize)]
struct Crate {
    root_module: String,
    edition: String,
    deps: Vec<Dep>,
    cfg: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Dep {
    #[serde(rename = "crate")]
    pos: usize,
    name: String,
}

impl RustAnalyzer {
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

    pub fn parse_directory_as_crates(&mut self, sol_dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(sol_dir)? {
            self.cratify(entry?.path().as_path());
        }

        self.crates[1..].sort_unstable_by(|a, b| a.root_module.cmp(&b.root_module));

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pyright {
    #[serde(rename = "venvPath")]
    venv_path: String,
    venv: String,
    #[serde(rename = "reportUnusedImport")]
    report_unused_import: bool,
}

impl Pyright {
    pub fn from(venv_path: String, venv: String, report_unused_import: bool) -> Self {
        Pyright { venv_path, venv, report_unused_import }
    }
}
