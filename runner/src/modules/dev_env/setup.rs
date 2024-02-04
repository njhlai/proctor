use std::path::PathBuf;
use std::{fs, io};

pub struct Setup {
    lang: String,
    sol_dir: PathBuf,
    file_to_content: Vec<(PathBuf, String)>,
}

impl Setup {
    pub fn from(lang: String, sol_dir: PathBuf, file_to_content: Vec<(PathBuf, String)>) -> Self {
        Setup { lang, sol_dir, file_to_content }
    }

    pub fn write(&self, overwrite: bool) -> io::Result<()> {
        for (file, content) in &self.file_to_content {
            let filepath = self.sol_dir.join(file);

            if filepath.exists() && !overwrite {
                println!(
                    "`{}` for {} dev environment at solution root {} exists, skipping",
                    file.display(),
                    self.lang,
                    self.sol_dir.display(),
                );
            } else {
                println!(
                    "Generating `{}` for {} dev environment at solution root {}",
                    file.display(),
                    self.lang,
                    self.sol_dir.display(),
                );
                fs::write(filepath, content)?;
            }
        }

        Ok(())
    }
}
