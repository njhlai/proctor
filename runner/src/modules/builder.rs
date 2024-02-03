use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::output_streams::OutputStream;
use super::solution::Solution;

const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-stdlib=libc++", "-Wall", "-fsanitize=address", "-g3", "-O2"];

/// A solution builder, defining the language-specific solution compiler and solution-testing bin runner.
pub struct Builder {
    lang: String,
    compiler: Command,
    binfile: PathBuf,
}

impl Builder {
    /// Constructs a [`Builder`] for the language `lang`.
    pub fn new(lang: &str, project_dir: &str) -> Self {
        match lang {
            "cpp" => clang(project_dir),
            "py" => python(project_dir),
            "rs" => rustc(project_dir),
            _ => todo!(),
        }
    }

    /// Returns the `PathBuf` to the solution-testing bin file.
    pub fn binfile(&self) -> PathBuf {
        self.binfile.clone()
    }

    /// Compiles `solution` via [`Builder`]'s `compiler` command.
    pub fn compile(&mut self, solution: &Solution) -> Result<OutputStream, OutputStream> {
        let solfile = solution.solfile(&self.lang);

        let output = self
            .compiler
            .arg(solfile.to_str().expect("Can't find file"))
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

/// Returns the `PathBuf` to the testing bin file.
fn binfile(project_dir: &str, lang: &str) -> PathBuf {
    PathBuf::from(project_dir).join(format!("bin/test_{lang}"))
}

/// [`Builder`] for `C++` solutions, using `clang++`.
fn clang(project_dir: &str) -> Builder {
    let lang = String::from("cpp");

    let mut compiler = Command::new("clang++");
    compiler
        .args([
            format!("-I{project_dir}/lib/cpp/src").as_str(),
            format!("-L{project_dir}/lib/cpp/build").as_str(),
            "-lproctor",
        ])
        .args(CLANG_COMPILE_FLAGS);

    let binfile = binfile(project_dir, &lang);

    Builder { lang, compiler, binfile }
}

/// [`Builder`] for `Python` solutions, using (a wrapper around) `py_compile`.
fn python(project_dir: &str) -> Builder {
    let lang = String::from("py");

    let mut compiler = Command::new("python");
    compiler.arg(format!("{project_dir}/runner/wrappers/compile.py"));

    let binfile = binfile(project_dir, &lang);

    Builder { lang, compiler, binfile }
}

/// [`Builder`] for `Rust` solutions, using `rustc`.
fn rustc(project_dir: &str) -> Builder {
    let lang = String::from("rs");

    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--extern",
            format!("libproctor={project_dir}/target/release/libproctor.rlib").as_str(),
        ])
        .args(RUSTC_COMPILE_FLAGS);

    let binfile = binfile(project_dir, &lang);

    Builder { lang, compiler, binfile }
}
