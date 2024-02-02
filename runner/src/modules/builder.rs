use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::output_streams::OutputStream;
use super::solution::Solution;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];
const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-Wall", "-fsanitize=address", "-g3", "-O2"];

/// A solution builder, defining the language-specific solution compiler and solution-testing bin runner.
pub struct Builder {
    lang: String,
    compiler: Command,
    runner: Command,
    binfile: PathBuf,
}

impl Builder {
    /// Compiles `solution` via [`Builder`]'s `compiler` command.
    pub fn compile(&mut self, solution: &Solution) -> Result<OutputStream, OutputStream> {
        let mut solfile = solution.path.clone();
        solfile.push("sol");
        solfile.set_extension(&self.lang);

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
            .unwrap_or_else(|_| panic!("Failed to compile solution {}", solution.prob));

        if output.status.success() {
            Ok(OutputStream::from(&output))
        } else {
            let _ = fs::remove_file(&self.binfile);

            Err(OutputStream::from(&output))
        }
    }

    /// Runs the compiled solution-testing bin via [`Builder`]'s `runner` command.
    pub fn run(&mut self) -> Result<OutputStream, OutputStream> {
        let output = self.runner.output().expect("Failed to run compiled binary");
        let output_streams = OutputStream::from(&output);

        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }

    /// Constructs a [`Builder`] for the language `lang`.
    pub fn new(lang: &str, project_dir: &str) -> Self {
        match lang {
            "cpp" => clang(project_dir),
            "py" => python(project_dir),
            "rs" => rustc(project_dir),
            _ => todo!(),
        }
    }
}

/// [`Builder`] for `C++` solutions, using `clang++`.
fn clang(project_dir: &str) -> Builder {
    let mut compiler = Command::new("clang++");
    compiler
        .args([
            format!("-I{project_dir}/lib/cpp/src").as_str(),
            format!("-L{project_dir}/lib/cpp/build").as_str(),
            "-lproctor",
        ])
        .args(CLANG_COMPILE_FLAGS);

    let binfile = PathBuf::from(project_dir).join("bin/test_cpp");
    let mut runner = Command::new(&binfile);
    runner
        .env("LD_LIBRARY_PATH", format!("{project_dir}/lib/cpp/build"))
        .arg("--success")
        .args(CLANG_COLOR_ARGS);

    Builder { lang: String::from("cpp"), compiler, runner, binfile }
}

/// [`Builder`] for `Python` solutions, using (a wrapper around) `py_compile`.
fn python(project_dir: &str) -> Builder {
    let mut compiler = Command::new("python");
    compiler.arg(format!("{project_dir}/runner/wrappers/compile.py"));

    let binfile = PathBuf::from(project_dir).join("bin/test_py");
    let mut runner = Command::new("python");
    runner.arg(&binfile).arg("-v");

    Builder { lang: String::from("py"), compiler, runner, binfile }
}

/// [`Builder`] for `Rust` solutions, using `rustc`.
fn rustc(project_dir: &str) -> Builder {
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--extern",
            format!("libproctor={project_dir}/target/release/libproctor.rlib").as_str(),
        ])
        .args(RUSTC_COMPILE_FLAGS);

    let binfile = PathBuf::from(project_dir).join("bin/test_rs");
    let mut runner = Command::new(&binfile);
    runner.arg("--show-output").args(RUSTC_COLOR_ARGS);

    Builder { lang: String::from("rs"), compiler, runner, binfile }
}
