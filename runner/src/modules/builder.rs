use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::config::Config;
use super::lang::Lang;
use super::output_streams::OutputStream;
use super::solution::Solution;

const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-stdlib=libc++", "-Wall", "-fsanitize=address", "-g3", "-O2"];

/// A solution builder, defining the language-specific solution compiler and solution-testing bin runner.
pub struct Builder {
    lang: Lang,
    compiler: Command,
    binfile: PathBuf,
}

impl Builder {
    /// Constructs a [`Builder`] for the language `lang`.
    pub fn new(lang: &Lang, config: &Config) -> Self {
        let compiler = match lang {
            Lang::Cpp => clang(config),
            Lang::Python => python(config),
            Lang::Rust => rustc(config),
        };

        Builder { lang: lang.clone(), compiler, binfile: config.binfile(&lang.to_string()) }
    }

    /// Compiles `solution` via [`Builder`]'s `compiler` command.
    pub fn compile(&mut self, solution: &Solution) -> Result<OutputStream, OutputStream> {
        let solfile = solution.solfile(&self.lang.to_string());

        let output = self
            .compiler
            .arg(solfile.display().to_string())
            .args([
                "-o",
                &self
                    .binfile
                    .to_str()
                    .ok_or_else(|| OutputStream::error("Can't parse bin filename"))?,
            ])
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

/// Forms the [`Command`] that executes `clang++`.
fn clang(config: &Config) -> Command {
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

/// Forms the [`Command`] that executes (a wrapper around) `py_compile`.
fn python(config: &Config) -> Command {
    let mut compiler = Command::new("python");
    compiler.arg(format!("{}/runner/wrappers/compile.py", config.project_dir_str));

    compiler
}

/// Forms the [`Command`] that executes `rustc`.
fn rustc(config: &Config) -> Command {
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--extern",
            format!("libproctor={}/target/release/libproctor.rlib", config.project_dir_str).as_str(),
        ])
        .args(RUSTC_COMPILE_FLAGS);

    compiler
}
