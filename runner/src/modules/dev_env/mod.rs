mod setup;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use colored::Colorize;
use strum::{EnumCount, IntoEnumIterator};

use super::config::Config;
use super::lang::Lang;

pub use self::setup::{Setup, Setups};

/// Sets up the dev environment.
pub fn setup(config: &Config, overwrite: bool) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(PathBuf::from(&config.sol_dir_str))?;

    Lang::iter()
        .generate_setups(config)?
        .into_iter()
        .enumerate()
        .try_for_each(|(i, (setup, additional_command))| {
            println!(
                "{} Running setup for {}:",
                format!("[{}/{}]", i + 1, Lang::COUNT).dimmed(),
                setup.lang.get_name().cyan().bold()
            );

            setup.run(additional_command, overwrite)
        })
}
