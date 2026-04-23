pub mod args;
pub mod commands;

pub use args::{Cli, Command};

use anyhow::Result;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Some(cmd) => commands::dispatch(cmd),
        // No subcommand → caller should fall through to interactive mode.
        None => Ok(()),
    }
}
