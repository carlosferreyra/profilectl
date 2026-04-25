use anyhow::Result;
use clap::{Args, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
    Pwsh,
}

#[derive(Args, Debug)]
pub struct BootstrapArgs {
    /// Target shell rc to modify (default: detected from $SHELL).
    #[arg(long, value_enum)]
    pub shell: Option<Shell>,

    /// Strip the previously inserted profilectl bootstrap block.
    #[arg(long)]
    pub remove: bool,
}

pub fn run(_args: BootstrapArgs) -> Result<()> {
    println!("profilectl bootstrap: not yet implemented");
    Ok(())
}
