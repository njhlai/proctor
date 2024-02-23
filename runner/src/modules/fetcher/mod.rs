#[allow(clippy::module_name_repetitions)]
mod query;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use colored::Colorize;
use html2md::{Handle, StructuredPrinter, TagHandler, TagHandlerFactory};
use tera::{Context, Tera, Value};

use super::colorize::MoreColorize;
use super::config::Config;
use super::lang::Lang;
use super::source::Source;

pub use query::{Empty, Method, Query, QueryResponse, Response};

/// Parses `html` into Markdown.
fn render_desc(html: &str) -> String {
    struct SupTagFactory;
    impl TagHandlerFactory for SupTagFactory {
        fn instantiate(&self) -> Box<dyn TagHandler> {
            Box::<SupTagHandler>::default()
        }
    }

    #[derive(Default)]
    struct SupTagHandler;
    impl TagHandler for SupTagHandler {
        fn handle(&mut self, _tag: &Handle, printer: &mut StructuredPrinter) {
            printer.append_str("^{");
        }

        fn after_handle(&mut self, printer: &mut StructuredPrinter) {
            printer.append_str("}");
        }
    }

    let mut custom = HashMap::<String, Box<dyn TagHandlerFactory>>::new();
    custom.insert(String::from("sup"), Box::new(SupTagFactory));

    html2md::parse_html_custom(html, &custom)
}

/// Renders `code` using the Jinja template `template_name`.
fn render_problem(config: &Config, template_name: &str, code: &Option<String>) -> Result<String, Box<dyn Error>> {
    let mut template = Tera::default();
    template.add_template_file(
        PathBuf::from(&config.project_dir_str).join(format!("runner/templates/{template_name}")),
        Some(template_name),
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
    context.insert("code", &code);
    context.insert("datastructs", &Vec::<(Source, &str)>::from([]));

    Ok(template.render(template_name, &context)?)
}

/// Fetches and renders the question data into a solution file, of which its [`PathBuf`] is returned if successful.
pub fn fetch(
    id: &str, lang: &Lang, source: &Source, config: &Config, overwrite: bool,
) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let dirpath = PathBuf::from(&config.sol_dir_str)
        .join(source.to_string())
        .join(id);
    fs::create_dir_all(&dirpath)?;

    let file = format!("sol.{lang}");
    let (sol_file, desc_file) = (dirpath.join(&file), dirpath.join("desc.md"));
    let sol_file_already_exists = sol_file.exists();

    if overwrite || !sol_file_already_exists {
        let (desc, code) = source.query(id, lang)?;

        if overwrite || !desc_file.exists() {
            print!("Rendering {}... ", desc_file.display().to_string().orange().bold());
            io::stdout().flush()?;

            fs::write(&desc_file, render_desc(&desc))?;
            println!("{}!", "OK".green().bold());
        }

        print!("Rendering {}... ", sol_file.display().to_string().orange().bold());
        io::stdout().flush()?;

        let template_name = format!("{}.j2", &file);
        fs::write(&sol_file, render_problem(config, &template_name, &code)?)?;
        println!("{}!", "OK".green().bold());

        if !sol_file_already_exists && lang == &Lang::Rust {
            println!(
                "Updating {}:",
                format!("{}/rust-project.json", &config.sol_dir_str)
                    .orange()
                    .bold()
            );

            lang.generate_setup(config)
                .and_then(|(setup, additional_command)| setup.run(additional_command, true))?;
        }
    } else {
        println!("{} exists, skipping", sol_file.display().to_string().orange().bold());
    }

    println!(
        "{} code for problem {} rendered as {}",
        lang.get_name().cyan().bold(),
        id.blue().bold(),
        sol_file.display().to_string().orange().bold()
    );
    Ok((sol_file, desc_file))
}
