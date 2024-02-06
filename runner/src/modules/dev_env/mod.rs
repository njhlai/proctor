mod lsp;
mod setup;

use std::error::Error;

use colored::Colorize;

use super::config::Config;

/// Sets up the dev environment.
pub fn setup(config: &Config) -> Result<(), Box<dyn Error>> {
    Ok(setup::generate_setups(config)?
        .into_iter()
        .try_for_each(|(setup, additional_command)| {
            setup.write(false).and_then(|()| {
                if let Some(mut cmd) = additional_command {
                    println!("Running additional commands for {}...", setup.lang.cyan().bold());

                    cmd.output().map(|_| ())
                } else {
                    Ok(())
                }
            })
        })?)
}
