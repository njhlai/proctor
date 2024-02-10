use std::path::PathBuf;
use std::process::Command;

use super::config::Config;
use super::lang::Lang;
use super::output_streams::OutputStream;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];

/// A structure defining a solution to a coding problem.
pub struct Solution {
    id: String,
    prob_dir: PathBuf,
    runner: Command,
}

impl Solution {
    /// Constructs a [`Solution`] to the problem `id`.
    pub fn new(id: &str, lang: &Lang, config: &Config) -> Self {
        let binfile = config.binfile(&lang.to_string());
        let runner = match lang {
            Lang::Cpp => {
                let mut runner = Command::new(&binfile);
                runner
                    .arg("--success")
                    .args(CLANG_COLOR_ARGS)
                    .env("LD_LIBRARY_PATH", format!("{}/lib/cpp/build", config.project_dir_str));

                runner
            }
            Lang::Python => {
                let mut runner = Command::new("python");
                runner
                    .arg(&binfile)
                    .arg("-v")
                    .env("PATH", format!("{}/venv/py311/bin:$PATH", config.sol_dir_str));

                runner
            }
            Lang::Rust => {
                let mut runner = Command::new(&binfile);
                runner.arg("--show-output").args(RUSTC_COLOR_ARGS);

                runner
            }
        };

        Solution { id: String::from(id), prob_dir: PathBuf::from(&config.sol_dir_str).join(id), runner }
    }

    /// Returns the problem ID of the [`Solution`].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the `PathBuf` to the file containing the solution.
    pub fn solfile(&self, ext: &str) -> PathBuf {
        let mut solfile = self.prob_dir.join("sol");
        solfile.set_extension(ext);

        solfile
    }

    /// Runs the compiled solution-testing bin via [`Solution`]'s `runner` command.
    pub fn run(&mut self) -> Result<OutputStream, OutputStream> {
        let output = self
            .runner
            .output()
            .expect("Failed to run compiled binary for solution-testing");

        let output_streams = OutputStream::from(&output);
        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }
}
