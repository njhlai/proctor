use std::path::PathBuf;

/// A structure defining a solution to a coding problem.
pub struct Solution {
    pub prob: String,
    pub path: PathBuf,
}

impl Solution {
    /// Returns a [`Solution`] to the problem `prob` which is in the `sol_dir` directory.
    pub fn new(prob: &str, sol_dir: &str) -> Self {
        let path = PathBuf::from(sol_dir).join(prob);

        Solution { prob: prob.to_string(), path }
    }
}
