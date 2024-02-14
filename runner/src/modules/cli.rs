use clap::{Parser, Subcommand};
use colored::Colorize;

use super::config::Config;
use super::dev_env;
use super::extcolorize::ExtColorize;
use super::grader;
use super::lang::Lang;

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

    /// Overwrite existing files
    #[arg(long)]
    overwrite: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setups the dev environment
    Setup,
    /// Compile and test solution to <PROBLEM>
    Run {
        /// Problem ID
        #[arg(value_parser = clap::value_parser!(u16).range(1..=LEETCODE_MAX_PROBLEM_ID))]
        problem: u16,
        /// Language to compile and test in
        lang: Lang,
    },
}

impl Cli {
    /// Runs the `proctor` CLI app.
    pub fn run(&self) {
        let config = if let Ok(config) = Config::read(&self.config) {
            println!("{}!", "OK".green().bold());

            config
        } else {
            println!("{}!", "FAILED".red().bold());
            println!(
                "{}: Can't read {}, proceeding with default configuration",
                "WARNING".yellow().bold(),
                "config.json".orange().bold()
            );

            Config::new(String::from("."), String::from("./data"))
        };

        println!();

        match &self.command {
            Commands::Setup => {
                println!("Setting up dev environment at solution root {}:", config.sol_dir_str.orange().bold());

                if let Err(err) = dev_env::setup(&config, self.overwrite) {
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
            Commands::Run { problem, lang } => {
                let id = &format!("{problem:0>4}");

                println!("Proctoring {} solution to problem {}:", lang.get_name().cyan().bold(), id.blue().bold());

                grader::run(id, lang, &config);
            }
        }
    }
}
