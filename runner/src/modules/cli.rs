use std::process;

use clap::{Parser, Subcommand};
use colored::{ColoredString, Colorize};

use super::colorize::MoreColorize;
use super::config::Config;
use super::dev_env;
use super::fetcher;
use super::grader;
use super::lang::Lang;
use super::source::Source;

const LEETCODE_MAX_PROBLEM_ID: i64 = 3023;

/// The command-line interface for `proctor`.
#[derive(Parser)]
#[command(name = "proctor")]
#[command(bin_name = "proctor")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Read config from the specified config JSON file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Setups the dev environment
    Setup {
        /// Overwrite existing dev environment setup
        #[arg(long)]
        overwrite: bool,
    },
    /// Fetches the problem
    Fetch {
        /// Problem ID
        #[arg(value_parser = clap::value_parser!(u16).range(1..=LEETCODE_MAX_PROBLEM_ID))]
        id: u16,

        /// Code language to fetch problem
        lang: Lang,

        /// Overwrite existing solution
        #[arg(long)]
        overwrite: bool,

        /// Source of problem
        #[arg(default_value_t = Source::LeetCode)]
        source: Source,
    },
    /// Compile and test solution
    Run {
        /// Problem ID
        #[arg(value_parser = clap::value_parser!(u16).range(1..=LEETCODE_MAX_PROBLEM_ID))]
        id: u16,

        /// Code language to compile and test in
        lang: Lang,

        /// Source of problem
        #[arg(default_value_t = Source::LeetCode)]
        source: Source,
    },
}

impl Cli {
    /// Runs the `proctor` CLI app.
    pub fn run(&self) {
        let config = if let Ok((config, pathbuf)) = Config::read(&self.config) {
            println!(
                "\n{} read {}, with config values:\n{:#?}",
                "Successfully".green().bold(),
                pathbuf.map_or_else(
                    || String::from("default configuration").orange().bold(),
                    |p| ColoredString::from(format!("configuration from {}", p.display().to_string().orange().bold())),
                ),
                config
            );

            config
        } else {
            println!("\n{} to read configuration, exiting proctor", "Failed".red().bold());
            process::exit(1);
        };

        println!();

        println!("Running command: {:#?}\n", self.command);
        match &self.command {
            Commands::Setup { overwrite } => {
                println!("Setting up dev environment at solution root {}:", config.sol_dir_str.orange().bold());

                if let Err(err) = dev_env::setup(&config, *overwrite) {
                    println!(
                        "{} to set up dev environment at solution root {}!\n{}: {err}",
                        "Failed".red().bold(),
                        config.sol_dir_str.orange().bold(),
                        "ERR".red().bold()
                    );
                } else {
                    println!(
                        "\n{} set up dev environment at solution root {}",
                        "Successfully".green().bold(),
                        config.sol_dir_str.orange().bold()
                    );
                }
            }
            Commands::Fetch { id, lang, overwrite, source } => {
                let id = &format!("{id:0>4}");

                println!("Fetching problem {} in {}:", id.blue().bold(), lang.get_name().cyan().bold());

                match fetcher::fetch(id, lang, source, &config, *overwrite) {
                    Ok((sol_file, desc_file)) => {
                        println!("\n{} fetched problem {}", "Successfully".green().bold(), id.blue().bold());
                    }
                    Err(err) => {
                        println!("{}!\n{}: {err}", "FAILED".red().bold(), "ERR".red().bold());
                    }
                }
            }
            Commands::Run { id, lang, source } => {
                let id = &format!("{id:0>4}");

                println!("Proctoring {} solution to problem {}:", lang.get_name().cyan().bold(), id.blue().bold());

                grader::run(id, lang, source, &config);
            }
        }
    }
}
