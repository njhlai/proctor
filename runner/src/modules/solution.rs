use std::path::PathBuf;

pub struct Solution {
    pub prob: String,
    pub path: PathBuf,
}

impl Solution {
    pub fn new(prob: &str, sol_dir: &str) -> Self {
        let path = PathBuf::from(sol_dir).join(prob);

        Solution { prob: prob.to_string(), path }
    }
}
