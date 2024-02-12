mod setup;

use std::error::Error;

use colored::Colorize;
use strum::{EnumCount, EnumProperty, IntoEnumIterator};

use super::config::Config;
use super::lang::Lang;

pub use self::setup::{Setup, Setups};

/// Sets up the dev environment.
pub fn setup(config: &Config, overwrite: bool) -> Result<(), Box<dyn Error>> {
    Ok(Lang::iter()
        .generate_setups(config)?
        .into_iter()
        .enumerate()
        .try_for_each(|(i, (setup, additional_command))| {
            println!(
                "{} Running setup for {}:",
                format!("[{}/{}]", i + 1, Lang::COUNT).dimmed(),
                setup.lang.get_str("name").unwrap().cyan().bold()
            );

            setup.write(overwrite).and_then(|()| {
                if let Some(mut cmd) = additional_command {
                    println!("Running additional commands for {}...", setup.lang.get_str("name").unwrap().cyan().bold());

                    cmd.output().map(|_| ())
                } else {
                    Ok(())
                }
            })
        })?)
}
