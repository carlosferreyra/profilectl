use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ProfileArgs {
    #[command(subcommand)]
    pub command: ProfileCommand,
}

#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    /// List every profile available in the home repo.
    List,

    /// Print the effective profile (after `extends` resolution) as TOML.
    Show {
        /// Profile name. Defaults to the active profile.
        name: Option<String>,
    },

    /// Persist <name> as the active profile in the global config.
    Use {
        /// Profile name to make active.
        name: String,
    },
}

pub fn run(args: ProfileArgs) -> Result<()> {
    match args.command {
        ProfileCommand::List => {
            println!("profilectl profile list: not yet implemented");
        }
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
