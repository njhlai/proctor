mod builder;
mod output_streams;
mod solution;

use std::io::{self, Write};

use colored::Colorize;

use super::config::Config;
use super::lang::Lang;
use super::source::Source;

use self::builder::Builder;
use self::solution::Solution;

/// Compiles and tests the solution.
pub fn run(id: &str, lang: &Lang, source: &Source, config: &Config) {
    let mut builder = Builder::new(lang, config);
    let mut solution = Solution::new(id, lang, source, config);

    print!("Compiling solution to problem {}... ", solution.id().blue());
    io::stdout().flush().unwrap();

    match builder.compile(&solution) {
        Ok(compile_os) => {
            println!("{}!", "SUCCESS".green().bold());

            println!("\n{}:", "COMPILE STDOUT".yellow().bold());
            println!("{}\n", if compile_os.stdout().is_empty() { "No compile output\n" } else { compile_os.stdout() });

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
