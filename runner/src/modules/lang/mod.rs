mod lsp;

use std::error::Error;
use std::process::Command;

use regex::{Error as RegexError, Regex};
use serde::Deserialize;
use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString};

use super::config::Config;
use super::dev_env::{Setup, Setups};
use super::source::{Form, Typ};

use self::lsp::Lsp;

const CLANG_COLOR_ARGS: &[&str] = &["--force-colors", "true"];
const CLANG_COMPILE_FLAGS: &[&str] = &["-std=c++20", "-stdlib=libc++", "-Wall", "-fsanitize=address", "-g3", "-O2"];
const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_COMPILE_FLAGS: &[&str] = &["--color", "always", "--edition", "2021", "--test"];

/// An enum listing available code languages.
#[derive(Clone, Debug, Deserialize, Display, EnumCount, EnumIter, EnumProperty, EnumString, PartialEq)]
pub enum Lang {
    #[strum(serialize = "cpp", props(name = "C++"))]
    #[serde(rename = "cpp")]
    Cpp,
    #[strum(serialize = "py", props(name = "Python3"))]
    #[serde(rename = "py")]
    Python,
    #[strum(serialize = "rs", props(name = "Rust"))]
    #[serde(rename = "rs")]
    Rust,
}

impl Lang {
    /// Get the full name of the language.
    pub fn get_name(&self) -> &'static str {
        self.get_str("name").unwrap()
    }

    /// Get all comment symbols of the language.
    pub fn comments(&self) -> Vec<&'static str> {
        match self {
            Lang::Cpp => vec!["//", "/**", " *"],
            Lang::Python => vec!["#"],
            Lang::Rust => vec!["//"],
        }
    }

    /// Returns the [`Command`] that executes the solution-testing `binfile`.
    pub fn tester(&self, config: &Config) -> Command {
        let binfile = config.binfile(&self.to_string());

        match self {
            Lang::Cpp => {
                let mut runner = Command::new(binfile);
                runner
                    .arg("--success")
                    .args(CLANG_COLOR_ARGS)
                    .env("LD_LIBRARY_PATH", format!("{}/lib/cpp/build", config.project_dir_str));

                runner
            }
            Lang::Python => {
                let mut runner = Command::new("python");
                runner
                    .arg(binfile)
                    .arg("-v")
                    .env("PATH", format!("{}/venv/py311/bin:$PATH", config.sol_dir_str));

                runner
            }
            Lang::Rust => {
                let mut runner = Command::new(binfile);
                runner.arg("--show-output").args(RUSTC_COLOR_ARGS);

                runner
            }
        }
    }

    /// Returns the [`Command`] that executes the language compiler.
    pub fn compiler(&self, config: &Config) -> Command {
        match self {
            Lang::Cpp => {
                let mut compiler = Command::new("clang++");
                compiler
                    .args([
                        format!("-I{}/lib/cpp/src", config.project_dir_str).as_str(),
                        format!("-L{}/lib/cpp/build", config.project_dir_str).as_str(),
                        "-lproctor",
                    ])
                    .args(CLANG_COMPILE_FLAGS);

                compiler
            }
            Lang::Python => {
                let mut compiler = Command::new("python");
                compiler.arg(format!("{}/runner/wrappers/compile.py", config.project_dir_str));

                compiler
            }
            Lang::Rust => {
                let mut compiler = Command::new("rustc");
                compiler
                    .args([
                        "--extern",
                        format!("libproctor={}/target/release/libproctor.rlib", config.project_dir_str).as_str(),
                    ])
                    .args(RUSTC_COMPILE_FLAGS);

                compiler
            }
        }
    }

    /// Generates the pair of [`Setup`] and additional commands to run for the language's setup.
    pub fn generate_setup(&self, config: &Config) -> Result<(Setup, Option<Command>), Box<dyn Error>> {
        match self {
            Lang::Cpp => lsp::Clangd::from(config).generate_setup(config),
            Lang::Python => lsp::Pyright::from(config).generate_setup(config),
            Lang::Rust => lsp::RustAnalyzer::from(config).generate_setup(config),
        }
    }

    /// Parses `typ` into the language-appropriate data type name.
    pub fn parse(&self, typ: &str) -> Result<Typ, Box<dyn Error>> {
        if let Some(caps) = Regex::new(r"(?<type>\w+)(?<arr>\[\])?")?.captures(typ) {
            let (mut transformed, mut form) = caps
                .name("type")
                .map_or((String::new(), Form::Unit), |m| match m.as_str() {
                    "integer" => (
                        String::from(match self {
                            Lang::Cpp => "int ",
                            Lang::Python => "int",
                            Lang::Rust => "i32",
                        }),
                        Form::Unit,
                    ),
                    "double" => (
                        String::from(match self {
                            Lang::Cpp => "double ",
                            Lang::Python => "float",
                            Lang::Rust => "f64",
                        }),
                        Form::Unit,
                    ),
                    _ => todo!(),
                });

            if caps.name("arr").is_some() {
                (transformed, form) = (
                    match self {
                        Lang::Cpp => format!("vector<{}> ", transformed.trim_end()),
                        Lang::Python => format!("List[{transformed}]"),
                        Lang::Rust => format!("Vec<{transformed}>"),
                    },
                    Form::Array,
                );
            }

            Ok(Typ { initial: String::from(typ), transformed, form })
        } else {
            Err(Box::new(RegexError::Syntax(String::from("Failed to parse type string"))))
        }
    }

    /// Processes `examples` into the language-appropriate form.
    pub fn process(&self, typ: &Typ, example: &str) -> String {
        match self {
            Lang::Cpp => {
                let cleaned = example.trim_matches(|c| c == '[' || c == ']');
                let mut it = typ.initial.chars();
                let datastruct = match it.next() {
                    None => String::new(),
                    Some(c) => c.to_lowercase().collect::<String>() + it.as_str(),
                };

                match typ.form {
                    Form::Unit => format!("({cleaned})"),
                    Form::Array => format!("({{ {cleaned} }})"),
                    Form::Pointer => format!(" = {datastruct}From(vector<int>({{ {cleaned} }}))"),
                }
            }
            Lang::Python => {
                if typ.form == Form::Pointer {
                    format!("{}From({example})", typ.initial)
                } else {
                    String::from(example)
                }
            }
            Lang::Rust => match typ.form {
                Form::Unit => String::from(example),
                Form::Array => format!("vec!{example}"),
                Form::Pointer => format!(" {}::from(vec!{example})", typ.initial),
            },
        }
    }
}

impl LangIter {
    /// Generates [`Setups`] detailing the setups for all available languages.
    pub fn generate_setups(self, config: &Config) -> Result<Setups, Box<dyn Error>> {
        self.map(|lang| lang.generate_setup(config)).collect()
    }
}
