use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub command: ProfileCommand,
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    /// List available profiles.
    List,

    /// Show a profile's resolved contents (defaults to the active profile).
    Show {
        /// Profile name. Defaults to the active profile (`--profile` / `PCTL_PROFILE`).
        name: Option<String>,
    },

    /// Switch the active profile (writes `~/.config/profilectl/config.toml`).
    Use {
        /// Profile name to activate.
        name: String,
    },
}

pub fn run(args: ProfileArgs) -> Result<()> {
    match args.command {
        ProfileCommand::List => println!("profilectl profile list: not yet implemented"),
        ProfileCommand::Show { name } => {
            let target = name.as_deref().unwrap_or("<active>");
            println!("profilectl profile show {target}: not yet implemented");
        }
        ProfileCommand::Use { name } => {
            println!("profilectl profile use {name}: not yet implemented");
        }
    }
    Ok(())
}
