use std::path::PathBuf;
use std::process::Command;

use super::config::Config;
use super::lang::Lang;
use super::output_streams::OutputStream;

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
        let runner = lang.tester(&binfile, config);

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
            .expect("Failed to run compiled binary for solution-testing! Error");

        let output_streams = OutputStream::from(&output);
        if output.status.success() { Ok(output_streams) } else { Err(output_streams) }
    }
}
