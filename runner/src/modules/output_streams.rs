use std::process::Output;

pub struct OutputStream {
    stdout: String,
    stderr: String,
}

impl OutputStream {
    pub fn from(output: &Output) -> Self {
        OutputStream {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }

    pub fn error(msg: &str) -> Self {
        OutputStream { stdout: String::new(), stderr: String::from(msg) }
    }

    pub fn stdout(&self) -> &str {
        self.stdout.as_str()
    }

    pub fn stderr(&self) -> &str {
        self.stderr.as_str()
    }
}
