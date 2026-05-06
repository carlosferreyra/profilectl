use crate::commands::scope::Scope;
use anyhow::Result;
use clap::Args;

#[derive(Args, Debug)]
pub struct ApplyArgs {
    /// Subset of profile state to apply.
    #[arg(long, value_enum, default_value_t = Scope::All)]
    pub scope: Scope,

    /// Run `git pull --ff-only` in the dotfiles repo before applying.
    #[arg(long)]
    pub pull: bool,

    /// Overwrite existing files / symlinks at link targets.
    #[arg(long)]
    pub force: bool,

    /// Fail fast on the first error instead of continuing and reporting at the end.
    #[arg(long)]
    pub strict: bool,
}

pub fn run(_args: ApplyArgs) -> Result<()> {
    println!("profilectl apply: not yet implemented");
    Ok(())
}
