use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Overwrite an existing global config.
    #[arg(long)]
    pub force: bool,

    /// Refuse to prompt; fail if any required field is missing.
    #[arg(long)]
    pub non_interactive: bool,

    /// Clone (URL) or copy (local path) a dotfiles repo into the chosen home.
    #[arg(long, value_name = "PATH-OR-URL")]
    pub from: Option<String>,
}

pub fn run(_args: InitArgs) -> Result<()> {
    println!("profilectl init: not yet implemented");
    Ok(())
}
