use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use clap::{Parser, Subcommand};
use colored::Colorize;
use serde_json::Value;

use super::builder::Builder;
use super::solution::Solution;

const LEETCODE_MAX_PROBLEM_ID: i64 = 3023;

/// The command-line interface for `proctor`.
#[derive(Parser)]
#[command(name = "proctor")]
#[command(bin_name = "proctor")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Read from the specified `directories.json` file
    #[arg(short, long, value_name = "FILE")]
    dir_json: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile and test solution
    Run {
        /// Problem ID
        #[arg(value_parser = clap::value_parser!(u16).range(1..=LEETCODE_MAX_PROBLEM_ID))]
        problem: u16,
        /// Language to compile and test in
        lang: String,
    },
}

impl Cli {
    /// Returns the project and solution directories from `dir_json` (default: "./directories.json") if possible.
    fn get_dir(&self) -> Result<Value, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(
            if let Some(dir_json_path) = self.dir_json.as_deref() {
                dir_json_path
            } else {
                "directories.json"
            },
        )?))?)
    }

    /// Runs the `proctor` CLI app.
    pub fn run() {
        let args = Self::parse();

        let dir = args.get_dir().unwrap();
        match &args.command {
            Commands::Run { problem, lang } => {
                let project_dir = if let Some(dir) = dir["project_dir"].as_str() { dir } else { "." };
                let mut builder = Builder::new(lang, project_dir);
                let mut solution = Solution::new(
                    format!("{problem:0>4}"),
                    if let Some(dir) = dir["sol_dir"].as_str() { dir } else { "data" },
                );

                print!("Problem {}: Compiling solution... ", solution.id().blue());
                match builder.compile(&solution) {
                    Ok(compile_os) => {
                        println!("{}!", "SUCCESS".green().bold());

                        if compile_os.stdout().is_empty() {
                            println!("\nNo compile output\n");
                        } else {
                            println!("\n{}:\n{}\n", "COMPILE STDOUT".yellow().bold(), compile_os.stdout());
                        }

                        print!("Testing solution to problem {}... ", solution.id().blue());
                        match solution.run(&builder.binfile(), project_dir) {
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
