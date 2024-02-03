mod modules;

use std::fs::File;
use std::io::BufReader;

use colored::Colorize;
use serde_json::Value;

use modules::builder::Builder;
use modules::solution::Solution;

fn main() {
    let dir: Value = serde_json::from_reader(BufReader::new(File::open("directories.json").unwrap())).unwrap();

    let project_dir = if let Some(dir) = dir["project_dir"].as_str() { dir } else { "" };
    let sol_dir = if let Some(dir) = dir["sol_dir"].as_str() { dir } else { "data" };
    let mut builder = Builder::new("cpp", project_dir);
    let mut solution = Solution::new(String::from("0002"), sol_dir);

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
            match solution.run(&builder.binfile, project_dir) {
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
            println!("{}!\n\n{}:\n{}", "FAILED".red().bold(), "COMPILE STDERR".yellow().bold(), compile_os.stderr());
        }
    }
}
