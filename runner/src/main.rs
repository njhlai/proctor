mod modules;

use std::fs::File;
use std::io::BufReader;

use colored::Colorize;
use serde_json::Value;

use modules::builder::Builder;
use modules::solution::Solution;

fn main() {
    let dir: Value = serde_json::from_reader(BufReader::new(File::open("directories.json").unwrap())).unwrap();

    let mut builder = Builder::new("rs", dir["project_dir"].as_str().unwrap());
    let solution = Solution::new("0002", dir["sol_dir"].as_str().unwrap());

    print!("Problem {}: Compiling solution... ", solution.prob.blue());
    match builder.compile(&solution) {
        Ok(compile_os) => {
            println!("{}!", "SUCCESS".green().bold());

            if compile_os.stdout().is_empty() {
                println!("\nNo compile output\n");
            } else {
                println!("\n{}:\n{}\n", "COMPILE STDOUT".yellow().bold(), compile_os.stdout());
            }

            print!("Testing solution to problem {}... ", solution.prob.blue());
            match builder.run() {
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
