mod lsp;
mod setup;

use std::io;
use std::path::{Path, PathBuf};

use self::lsp::{Pyright, RustAnalyzer};
use self::setup::Setup;

pub fn setup(project_dir_str: &str, sol_dir_str: &str) -> io::Result<()> {
    let sol_dir = PathBuf::from(sol_dir_str);
    let project_dir = Path::new(project_dir_str);

    let cpp_setup = Setup::from(
        String::from("C++"),
        sol_dir.clone(),
        vec![(PathBuf::from(".clangd"), format!("CompileFlags:\n  Add: -I{project_dir_str}/lib/cpp/src/\n"))],
    );
    cpp_setup.write(false)?;

    let pyright = Pyright::from(String::from("./venv"), String::from("py311"), false);
    let python_setup = Setup::from(
        String::from("Python"),
        sol_dir.clone(),
        vec![
            (PathBuf::from(".python-version"), String::from("3.11.7\n")),
            (PathBuf::from("pyrightconfig.json"), serde_json::to_string_pretty(&pyright)?),
        ],
    );
    python_setup.write(false)?;

    let mut rust_analyzer = RustAnalyzer::new(project_dir);
    rust_analyzer
        .parse_directory_as_crates(sol_dir.as_path())
        .unwrap_or_else(|_| panic!("Couldn't parse directory structure of solution root {sol_dir_str}"));

    let rust_setup = Setup::from(
        String::from("Rust"),
        sol_dir.clone(),
        vec![(PathBuf::from("rust-project.json"), serde_json::to_string_pretty(&rust_analyzer)?)],
    );
    rust_setup.write(false)?;

    Ok(())
}
