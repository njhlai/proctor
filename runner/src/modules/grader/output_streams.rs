use std::process::Output;

/// A wrapper around [`Output`] for easy access to the standard streams of a command.
pub struct OutputStream {
    stdout: String,
    stderr: String,
}

impl OutputStream {
    /// Returns the [`OutputStream`] extracted from `output`.
    pub fn from(output: &Output) -> Self {
        OutputStream {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }

    /// Returns the stdout stream.
    pub fn stdout(&self) -> &str {
        self.stdout.as_str()
    }

    /// Returns the stderr stream.
    pub fn stderr(&self) -> &str {
        self.stderr.as_str()
    }

    /// Returns either the non-empty stdout stream, else the stderr stream.
    pub fn stdout_else_stderr(&self) -> &str {
        if self.stdout.is_empty() { self.stderr() } else { self.stdout() }
    }
}
