mod modules;

use clap::Parser;

use modules::cli::Cli;

fn main() {
    Cli::parse().run();
}
