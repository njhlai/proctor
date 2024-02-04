use serde::Serialize;

/// `pyright` config JSON serializer.
#[derive(Serialize)]
pub struct Pyright {
    #[serde(rename = "venvPath")]
    venv_path: String,
    venv: String,
    #[serde(rename = "reportUnusedImport")]
    report_unused_import: bool,
}

impl Pyright {
    /// Returns a [`Pyright`] detailing a `pyright` config for the dev environment.
    pub fn from(venv_path: String, venv: String, report_unused_import: bool) -> Self {
        Pyright { venv_path, venv, report_unused_import }
    }
}
