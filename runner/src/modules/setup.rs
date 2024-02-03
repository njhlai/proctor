use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

struct Setup<C> {
    lang: String,
    sol_dir: PathBuf,
    file_to_content: Vec<(PathBuf, C)>,
}

impl<C: AsRef<[u8]>> Setup<C> {
    pub fn from(lang: String, sol_dir: PathBuf, file_to_content: Vec<(PathBuf, C)>) -> Self {
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
struct RustAnalyzerProject {
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

impl RustAnalyzerProject {
    fn new(project_dir: &Path) -> RustAnalyzerProject {
        let toolchain = Command::new("rustc")
            .arg("--print")
            .arg("sysroot")
            .output()
            .expect("hello")
            .stdout;

        let toolchain = String::from_utf8_lossy(&toolchain);
        let mut whitespace_iter = toolchain.split_whitespace();
        let toolchain = whitespace_iter.next().unwrap_or(&toolchain);

        let sysroot_src = Path::new(toolchain)
            .join("lib")
            .join("rustlib")
            .join("src")
            .join("rust")
            .join("library")
            .display()
            .to_string();

        let libproctor_root_module = project_dir
            .join("lib")
            .join("rs")
            .join("src")
            .join("lib.rs")
            .display()
            .to_string();
        let libproctor_crate = Crate {
            root_module: libproctor_root_module,
            edition: String::from("2021"),
            deps: vec![],
            cfg: vec![],
        };

        RustAnalyzerProject { sysroot_src, crates: vec![libproctor_crate] }
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

    fn parse_directory_as_crates(&mut self, sol_dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(sol_dir)? {
            self.cratify(entry?.path().as_path());
        }

        self.crates[1..].sort_unstable_by(|a, b| a.root_module.cmp(&b.root_module));

        Ok(())
    }
}

pub fn setup(project_dir_str: &str, sol_dir_str: &str) -> io::Result<()> {
    let sol_dir = PathBuf::from(sol_dir_str);
    let project_dir = Path::new(project_dir_str);

    let cpp_setup = Setup::from(
        String::from("C++"),
        sol_dir.clone(),
        vec![(PathBuf::from(".clangd"), format!("CompileFlags:\n  Add: -I{project_dir_str}/lib/cpp/src/"))],
    );
    cpp_setup.write(false)?;

    let python_setup = Setup::from(
        String::from("Python"),
        sol_dir.clone(),
        vec![
            (PathBuf::from(".python-version"), "3.11.7\n"),
            (
                PathBuf::from("pyrightconfig.json"),
                "{\n    \"venvPath\": \"./venv\",\n    \"venv\": \"py311\",\n    \"reportUnusedImport\": false,\n}\n",
            ),
        ],
    );
    python_setup.write(false)?;

    let mut rust_analyzer_project = RustAnalyzerProject::new(project_dir);
    rust_analyzer_project
        .parse_directory_as_crates(sol_dir.as_path())
        .unwrap_or_else(|_| panic!("Couldn't parse directory structure of solution root {sol_dir_str}"));

    let rust_setup = Setup::from(
        String::from("Rust"),
        sol_dir.clone(),
        vec![(
            PathBuf::from("rust-project.json"),
            serde_json::to_string_pretty(&rust_analyzer_project).expect("Failed to serialize to JSON"),
        )],
    );
    rust_setup.write(false)?;

    Ok(())
}
