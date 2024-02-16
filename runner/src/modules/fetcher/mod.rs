#[allow(clippy::module_name_repetitions)]
mod query;

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::Colorize;
use tera::{Context, Tera, Value};

use super::colorize::MoreColorize;
use super::config::Config;
use super::lang::Lang;
use super::source::Source;

pub use query::{Empty, Query, QueryResponse, Response};

/// Fetches and renders the question data into a solution file, of which its [`PathBuf`] is returned if successful.
pub fn fetch(id: &str, lang: &Lang, source: &Source, config: &Config, overwrite: bool) -> Result<PathBuf, Box<dyn Error>> {
    let dirpath = PathBuf::from(&config.sol_dir_str)
        .join(source.to_string())
        .join(id);
    fs::create_dir_all(&dirpath)?;

    let file = format!("sol.{lang}");
    let filepath = dirpath.join(&file);

    if overwrite || !filepath.exists() {
        let template_name = format!("{}.j2", &file);

        let mut template = Tera::default();
        template.add_template_file(
            PathBuf::from(&config.project_dir_str).join(format!("runner/templates/{template_name}")),
            Some(&template_name),
        )?;
        template.register_filter("camel", |value: &Value, _: &_| {
            let s = tera::try_get_value!("camel", "value", String, value);

            let mut it = s.chars();
            Ok(tera::to_value(match it.next() {
                None => String::new(),
                Some(c) => c.to_lowercase().collect::<String>() + it.as_str(),
            })?)
        });

        let mut context = Context::new();
        context.insert("code", &source.query(id, lang)?);
        context.insert("datastructs", &Vec::<(Source, &str)>::from([]));

        print!("Rendering {}... ", filepath.display().to_string().orange().bold());
        io::stdout().flush()?;

        let code = template.render(&template_name, &context)?;
        println!("{}!", "OK".green().bold());

        fs::write(&filepath, code)?;
    } else {
        println!("{} exists, skipping", filepath.display().to_string().orange().bold());
    }

    println!(
        "{} code for problem {} rendered as {}",
        lang.get_name().cyan().bold(),
        id.blue().bold(),
        filepath.display().to_string().orange().bold()
    );
    Ok(filepath)
}
