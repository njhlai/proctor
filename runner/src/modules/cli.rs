use std::io::{self, Write};

use clap::{Parser, Subcommand};
use colored::Colorize;
use strum::EnumProperty;

use super::builder::Builder;
use super::config::Config;
use super::dev_env;
use super::extcolorize::ExtColorize;
use super::lang::Lang;
use super::solution::Solution;

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

                if let Err(err) = dev_env::setup(&config) {
                    println!(
                        "\n{} to set up dev environment at solution root {}:\n\n{}:\n{err}",
                        "Failed".red().bold(),
                        config.sol_dir_str.orange().bold(),
                        "ERR".yellow().bold()
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

                println!(
                    "Proctoring {} solution to problem {}:",
                    lang.get_str("name").unwrap().cyan().bold(),
                    id.blue().bold()
                );

                let mut builder = Builder::new(lang, &config);
                let mut solution = Solution::new(id, lang, &config);

                print!("Compiling solution to problem {}... ", solution.id().blue());
                io::stdout().flush().unwrap();

                match builder.compile(&solution) {
                    Ok(compile_os) => {
                        println!("{}!", "SUCCESS".green().bold());

                        println!("\n{}:", "COMPILE STDOUT".yellow().bold());
                        println!(
                            "{}\n",
                            if compile_os.stdout().is_empty() { "No compile output\n" } else { compile_os.stdout() }
                        );

                        print!("Testing solution to problem {}... ", solution.id().blue());
                        io::stdout().flush().unwrap();

                        match solution.run() {
                            Ok(run_os) => {
                                println!(
                                    "Solution {}!\n\n{}:\n{}",
                                    "PASSED".green().bold(),
                                    "TEST RESULT".yellow().bold(),
                                    run_os.stdout_else_stderr(),
                                );
                            }
                            Err(run_os) => {
                                println!("Solution {}!\n", "FAILED".red().bold());
                                println!("\n{}:\n{}", "TEST STDOUT".yellow().bold(), run_os.stdout());
                                println!("\n{}:\n{}", "TEST STDERR".yellow().bold(), run_os.stderr());
                            }
                        }
                    }
                    Err(compile_os) => {
                        println!("{}!\n\n{}:\n{}", "ERROR".red().bold(), "COMPILE STDERR".yellow().bold(), compile_os.stderr());
                    }
                }
            }
        }
    }
}
