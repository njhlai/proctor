use std::path::PathBuf;
use std::process::Command;

use super::output_streams::OutputStream;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];

/// A structure defining a solution to a coding problem.
pub struct Solution {
    id: String,
    prob_dir: PathBuf,
}

impl Solution {
    /// Constructs a [`Solution`] to the problem with id `id` which is in the `sol_dir` directory.
    pub fn new(id: String, sol_dir: &str) -> Self {
        let prob_dir = PathBuf::from(sol_dir).join(&id);

        Solution { id, prob_dir }
    }

    /// Returns the problem ID of the [`Solution`].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the `PathBuf` to the file containing the solution.
    pub fn solfile(&self, lang: &str) -> PathBuf {
        let mut solfile = self.prob_dir.clone();
        solfile.push("sol");
        solfile.set_extension(lang);

        solfile
    }

    /// Runs the compiled solution-testing bin via [`Solution`]'s `runner` command.
    pub fn run(&mut self, binfile: &PathBuf, project_dir: &str) -> Result<OutputStream, OutputStream> {
        let lang = binfile.file_name().map(|e| e.to_str().unwrap_or(""));

        let mut runner = if let Some("test_py") = lang { Command::new("python") } else { Command::new(binfile) };
        let output = match lang {
            None => panic!(),
            Some("test_cpp") => runner
                .env("LD_LIBRARY_PATH", format!("{project_dir}/lib/cpp/build"))
                .arg("--success")
                .args(CLANG_COLOR_ARGS),
            Some("test_py") => runner.arg(binfile).arg("-v"),
            Some("test_rs") => runner.arg("--show-output").args(RUSTC_COLOR_ARGS),
            _ => todo!(),
        }
        .output()
        .expect("Failed to run compiled binary for solution-testing");

        let output_streams = OutputStream::from(&output);
        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }
}
