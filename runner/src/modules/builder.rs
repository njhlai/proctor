use std::fs;
use std::path::PathBuf;
use std::process::Command;

use super::output_streams::OutputStream;
use super::solution::Solution;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];
const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-Wall", "-fsanitize=address", "-g3", "-O2"];

pub struct Builder {
    lang: String,
    compiler: Command,
    runner: Command,
    binfile: PathBuf,
}

impl Builder {
    pub fn compile(&mut self, solution: &Solution) -> Result<OutputStream, OutputStream> {
        let mut path = solution.path.clone();
        path.push("sol");
        path.set_extension(&self.lang);

        let output = self
            .compiler
            .arg(path.to_str().expect("Can't find file"))
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

    pub fn run(&mut self) -> Result<OutputStream, OutputStream> {
        let output = self.runner.output().expect("Failed to run compiled binary");
        let output_streams = OutputStream::from(&output);

        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }

    pub fn new(lang: &str, project_dir: &str) -> Self {
        match lang {
            "cpp" => clang(project_dir),
            "rs" => rustc(project_dir),
            _ => todo!(),
        }
    }
}

fn clang(project_dir: &str) -> Builder {
    let mut compiler = Command::new("clang++");
    compiler
        .args([
            format!("-I{project_dir}/lib/cpp/src").as_str(),
            format!("-L{project_dir}/lib/cpp/build").as_str(),
            "-lproctor",
        ])
        .args(CLANG_COMPILE_FLAGS);

    let binfile = PathBuf::from(&project_dir).join("bin/test_cpp");
    let mut runner = Command::new(&binfile);
    runner
        .env("LD_LIBRARY_PATH", format!("{project_dir}/lib/cpp/build"))
        .arg("--success")
        .args(CLANG_COLOR_ARGS);

    Builder { lang: String::from("cpp"), compiler, runner, binfile }
}

fn rustc(project_dir: &str) -> Builder {
    let mut compiler = Command::new("rustc");
    compiler
        .args([
            "--extern",
            format!("libproctor={project_dir}/target/debug/libproctor.rlib").as_str(),
        ])
        .args(RUSTC_COMPILE_FLAGS);

    let binfile = PathBuf::from(&project_dir).join("bin/test_rs");
    let mut runner = Command::new(&binfile);
    runner.arg("--show-output").args(RUSTC_COLOR_ARGS);

    Builder { lang: String::from("rs"), compiler, runner, binfile }
}
