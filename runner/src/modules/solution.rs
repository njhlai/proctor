use std::path::PathBuf;
use std::process::Command;

use super::config::Config;
use super::output_streams::OutputStream;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];

/// A structure defining a solution to a coding problem.
pub struct Solution {
    id: String,
    prob_dir: PathBuf,
}

impl Solution {
    /// Constructs a [`Solution`] to the problem `id`.
    pub fn new(id: &str, config: &Config) -> Self {
        Solution { id: String::from(id), prob_dir: PathBuf::from(&config.sol_dir_str).join(id) }
    }

    /// Returns the problem ID of the [`Solution`].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the `PathBuf` to the file containing the solution.
    pub fn solfile(&self, lang: &str) -> PathBuf {
        let mut solfile = self.prob_dir.join("sol");
        solfile.set_extension(lang);

        solfile
    }

    /// Runs the compiled solution-testing bin via [`Solution`]'s `runner` command.
    pub fn run(&mut self, lang: &str, config: &Config) -> Result<OutputStream, OutputStream> {
        let binfile = config.binfile(lang);

        let mut runner = if lang == "py" { Command::new("python") } else { Command::new(&binfile) };
        let output = match lang {
            "cpp" => runner
                .env("LD_LIBRARY_PATH", format!("{}/lib/cpp/build", config.project_dir_str))
                .arg("--success")
                .args(CLANG_COLOR_ARGS),
            "py" => runner.arg(&binfile).arg("-v"),
            "rs" => runner.arg("--show-output").args(RUSTC_COLOR_ARGS),
            _ => todo!(),
        }
        .output()
        .expect("Failed to run compiled binary for solution-testing");

        let output_streams = OutputStream::from(&output);
        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }
}
